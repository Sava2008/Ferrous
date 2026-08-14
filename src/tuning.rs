use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        attacks::{BLACK_PAWN_ATTACKS, MVV_LVA, WHITE_PAWN_ATTACKS},
        heuristics::*,
    },
    search::Engine,
};
const LAZY_SORT_LEN: usize = 8;

impl Engine {
    #[inline(always)]
    pub fn score_all_moves(
        &mut self,
        depth: usize,
        last_occupied: usize,
        previous_best_move: &u16,
        current_board: &Board,
        show_scores: bool,
    ) -> () {
        let pseudo_moves: &[u16; 192] = &self.move_lists[depth].pseudo_moves;
        for i in 0..last_occupied {
            let mv: u16 = pseudo_moves[0..last_occupied][i];

            if pseudo_moves[i] == *previous_best_move {
                self.move_scores[depth][i] = i16::MAX;
            } else {
                self.move_scores[depth][i] = self.move_priority(&mv, depth, current_board);
            }
            if show_scores {
                println!(
                    "move: {} {}, score: {}",
                    from_square(mv),
                    to_square(mv),
                    pseudo_moves[i]
                );
            }
        }
    }

    #[inline(always)]
    pub fn move_priority(&self, m: &u16, depth: usize, current_board: &Board) -> i16 {
        let mut score: i16 = 0;
        let to_square: usize = to_square(*m) as usize;
        let flag: u16 = (m & MARK_MASK) >> MARK_SHIFT;
        let (moving_piece_type, taken_piece_type): (u16, u16) = (
            current_board.cached_pieces[from_square(*m) as usize],
            current_board.cached_pieces[to_square],
        );

        score += match flag as i16 {
            2 => 100,
            7 => 100,
            8 => 150,
            other => other * 20,
        };

        if taken_piece_type != 0 {
            let mut victim_value: usize = Self::get_piece_value(taken_piece_type);
            let mut attacker_value: usize = Self::get_piece_value(moving_piece_type);
            if victim_value > 5 {
                victim_value -= 6;
            }
            if attacker_value > 5 {
                attacker_value -= 6;
            }

            return score + MVV_LVA[victim_value][attacker_value];
        }
        let history_idx: usize =
            (((m & FROM_MASK) as usize) << 6) | ((m & TO_MASK) >> TO_SHIFT) as usize;
        score += self.history_heuristics[history_idx] / 30;

        score += Self::does_improve_piece(*m, moving_piece_type) as i16;

        if self.killer_moves[depth][0] == Some(*m) {
            // killer moves cannot be captures or checks
            score += 100;
        } else if self.killer_moves[depth][1] == Some(*m) {
            score += 80;
        }

        if moving_piece_type < 7 {
            return score
                - 25 * (WHITE_PAWN_ATTACKS[to_square] & current_board.bitboards[6]).count_ones()
                    as i16;
        }
        return score
            - 25 * (BLACK_PAWN_ATTACKS[to_square] & current_board.bitboards[0]).count_ones()
                as i16;
    }

    #[inline(always)]
    pub fn n_log_n_sort_moves(
        moves: &mut [u16; 192],
        scores: &mut [i16; 192],
        last_occupied: usize,
    ) -> () {
        for i in 0..last_occupied {
            let (best_idx, _) = scores[i..last_occupied]
                .iter()
                .enumerate()
                .max_by_key(|(_, score)| **score)
                .unwrap();
            let best_idx = i + best_idx;
            scores.swap(i, best_idx);
            moves.swap(i, best_idx);
        }
    }

    #[inline(always)]
    pub fn lazy_sort_moves(
        moves: &mut [u16; 192],
        scores: &mut [i16; 192],
        last_occupied: usize,
    ) -> () {
        for i in 0..last_occupied {
            if i >= LAZY_SORT_LEN {
                break;
            }
            let true_index: usize = scores[i..last_occupied]
                .iter()
                .enumerate()
                .max_by_key(|&(_, score)| score)
                .unwrap()
                .0
                + i;
            moves.swap(true_index, i);
            scores.swap(true_index, i);
        }
    }

    #[inline(always)]
    pub fn get_piece_value(piece_type: u16) -> usize {
        return (piece_type as usize) - 1;
    }

    #[inline(always)]
    pub fn does_improve_piece(m: u16, t: u16) -> i32 {
        let heuristics_table: &[i32; 64] = unsafe { &HEURISTICS_TABLE[t as usize - 1] };
        return heuristics_table[to_square(m) as usize] - heuristics_table[from_square(m) as usize];
    }
}
