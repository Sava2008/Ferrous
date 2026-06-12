use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        attacks::{BLACK_PAWN_ATTACKS, MVV_LVA, WHITE_PAWN_ATTACKS},
        heuristics::*,
    },
    search::Engine,
};

impl Engine {
    pub fn score_all_moves(
        &mut self,
        depth: usize,
        last_occupied: usize,
        previous_best_move: &u16,
        current_board: &Board,
    ) -> () {
        for i in 0..last_occupied {
            let mv: u16 = self.move_lists[depth].pseudo_moves[0..last_occupied][i];

            if self.move_lists[depth].pseudo_moves[i] == *previous_best_move {
                self.move_scores[depth][i] = i16::MAX;
            } else {
                self.move_scores[depth][i] = self.move_priority(&mv, depth, current_board);
            }
        }
    }
    pub fn move_priority(&self, m: &u16, depth: usize, current_board: &Board) -> i16 {
        let mut score: i16 = 0;
        let to_square: usize = to_square(*m) as usize;
        let (moving_piece_type, taken_piece_type): (u16, u16) = (
            current_board.cached_pieces[from_square(*m) as usize],
            current_board.cached_pieces[to_square],
        );
        if taken_piece_type != 0 {
            let mut victim_value: usize = Self::get_piece_value(taken_piece_type);
            let mut attacker_value: usize = Self::get_piece_value(moving_piece_type);
            if victim_value > 5 {
                victim_value -= 6;
            }
            if attacker_value > 5 {
                attacker_value -= 6;
            }

            if attacker_value > 12 {
                panic!("index: {}", from_square(*m));
            }
            return unsafe { MVV_LVA[victim_value][attacker_value] } * 1000;
        }
        if self.killer_moves[depth][0] == Some(*m) {
            score += 900;
        } else if self.killer_moves[depth][1] == Some(*m) {
            score += 800;
        }

        let history_idx: usize =
            (((m & FROM_MASK) as usize) << 6) | ((m & TO_MASK) >> TO_SHIFT) as usize;
        score += self.history_heuristics[history_idx] / 100;
        score +=
            Self::does_improve_piece(*m, moving_piece_type, &current_board.cached_pieces) as i16;
        if moving_piece_type < 7 {
            score -= 200
                * (WHITE_PAWN_ATTACKS[to_square] & current_board.bitboards[6]).count_ones() as i16;
            return score;
        }
        score -=
            200 * (BLACK_PAWN_ATTACKS[to_square] & current_board.bitboards[0]).count_ones() as i16;
        return score;
    }
    fn get_piece_value(piece_type: u16) -> usize {
        if piece_type > 12 {
            panic!("bad piece type: {piece_type}");
        }
        return (piece_type as usize) - 1;
    }

    #[inline(always)]
    pub fn does_improve_piece(m: u16, t: u16, board: &[u16; 64]) -> i32 {
        let (from_sq, to_sq): (usize, usize) = (from_square(m) as usize, to_square(m) as usize);
        let piece_table_idx: usize = t as usize - 1;
        if piece_table_idx > 11 {
            panic!(
                "board: {:?}, index: {}, piece_table_idx: {piece_table_idx}, t: {t}",
                board, from_sq
            );
        }
        let heuristics_table: &[i32; 64] = unsafe { &HEURISTICS_TABLE[piece_table_idx] };
        return heuristics_table[to_sq] - heuristics_table[from_sq];
    }
}
