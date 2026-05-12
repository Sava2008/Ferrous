use crate::{
    board_geometry_templates::*,
    constants::{attacks::MVV_LVA, heuristics::*},
    search::Engine,
};

impl Engine {
    pub fn score_all_moves(
        &mut self,
        depth: usize,
        last_occupied: usize,
        previous_best_move: &u32,
        color: u8,
    ) -> () {
        for i in 0..last_occupied {
            if self.move_lists[depth].pseudo_moves[i] == *previous_best_move {
                self.move_scores[depth][i] = i16::MAX;
            } else {
                self.move_scores[depth][i] = self.move_priority(
                    &self.move_lists[depth].pseudo_moves[0..last_occupied][i],
                    depth,
                    color,
                );
            }
        }
    }
    pub fn move_priority(&self, m: &u32, depth: usize, color: u8) -> i16 {
        let mut score: i16 = 0;
        let (moving_piece_type, taken_piece_type): (u32, u32) =
            (moving_piece_type(*m), captured_piece_type(*m));
        if taken_piece_type != 0 {
            let victim_value: usize = Self::get_piece_value(taken_piece_type);
            let attacker_value: usize = Self::get_piece_value(moving_piece_type);
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
        score += Self::does_improve_piece(*m, moving_piece_type, color) as i16;
        return score;
    }
    fn get_piece_value(piece_type: u32) -> usize {
        if piece_type > 6 {
            panic!("bad piece type: {piece_type}");
        }
        return (piece_type as usize) - 1;
    }

    #[inline(always)]
    pub fn does_improve_piece(m: u32, t: u32, color: u8) -> i32 {
        let (from_sq, to_sq): (usize, usize) = (
            (m & FROM_MASK) as usize,
            ((m & TO_MASK) >> TO_SHIFT) as usize,
        );
        return if color == 8 {
            match t {
                COLORLESS_BISHOP => {
                    WHITE_BISHOP_HEURISTICS[to_sq] - WHITE_BISHOP_HEURISTICS[from_sq]
                }
                COLORLESS_PAWN => WHITE_PAWN_HEURISTICS[to_sq] - WHITE_PAWN_HEURISTICS[from_sq],
                COLORLESS_KNIGHT => {
                    WHITE_KNIGHT_HEURISTICS[to_sq] - WHITE_KNIGHT_HEURISTICS[from_sq]
                }
                COLORLESS_QUEEN => WHITE_QUEEN_HEURISTICS[to_sq] - WHITE_QUEEN_HEURISTICS[from_sq],
                COLORLESS_ROOK => WHITE_ROOK_HEURISTICS[to_sq] - WHITE_ROOK_HEURISTICS[from_sq],
                COLORLESS_KING => WHITE_KING_HEURISTICS[to_sq] - WHITE_KING_HEURISTICS[from_sq],
                other => {
                    unreachable!("bits: {other}, from: {from_sq}, to: {to_sq}, color: {color}")
                }
            }
        } else {
            match t {
                COLORLESS_BISHOP => {
                    BLACK_BISHOP_HEURISTICS[to_sq] - BLACK_BISHOP_HEURISTICS[from_sq]
                }
                COLORLESS_PAWN => BLACK_PAWN_HEURISTICS[to_sq] - BLACK_PAWN_HEURISTICS[from_sq],
                COLORLESS_KNIGHT => {
                    BLACK_KNIGHT_HEURISTICS[to_sq] - BLACK_KNIGHT_HEURISTICS[from_sq]
                }
                COLORLESS_QUEEN => BLACK_QUEEN_HEURISTICS[to_sq] - BLACK_QUEEN_HEURISTICS[from_sq],
                COLORLESS_ROOK => BLACK_ROOK_HEURISTICS[to_sq] - BLACK_ROOK_HEURISTICS[from_sq],
                COLORLESS_KING => BLACK_KING_HEURISTICS[to_sq] - BLACK_KING_HEURISTICS[from_sq],
                other => {
                    unreachable!("bits: {other}, from: {from_sq}, to: {to_sq}, color: {color}")
                }
            }
        };
    }
}
