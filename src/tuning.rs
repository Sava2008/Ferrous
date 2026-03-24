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
    ) -> () {
        for i in 0..last_occupied {
            self.move_scores[depth][i] = self.move_priority(
                &self.move_lists[depth].pseudo_moves[0..last_occupied][i],
                depth,
            );
            if self.move_lists[depth].pseudo_moves[i] == *previous_best_move {
                self.move_scores[depth][i] = 10000;
            }
        }
    }
    pub fn move_priority(&self, m: &u32, depth: usize) -> i16 {
        let mut priority_key: i16 = 0;
        let (initial_pos, final_pos): (u32, u32) = (moving_piece_type(*m), captured_piece_type(*m));
        if final_pos != 0 {
            let victim_value: usize = Self::get_piece_value(final_pos) as usize; //
            let attacker_value: usize = Self::get_piece_value(initial_pos) as usize;
            priority_key += unsafe { MVV_LVA[victim_value][attacker_value] };
        }
        if self.killer_moves[depth][0] == Some(*m) || self.killer_moves[depth][1] == Some(*m) {
            priority_key += 5;
        }
        if Self::does_improve_piece(*m, initial_pos) {
            priority_key += 5;
        }
        return priority_key;
    }
    fn get_piece_value(piece_type: u32) -> u8 {
        match piece_type {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            other => unreachable!("captured piece: {other}"),
        }
    }

    #[inline(always)]
    pub fn does_improve_piece(m: u32, t: u32) -> bool {
        let (from_sq, to_sq, color): (usize, usize, u8) = (
            (m & FROM_MASK) as usize,
            ((m & TO_MASK) >> TO_SHIFT) as usize,
            moving_piece_color(m),
        );
        return if color == 8 {
            match t {
                COLORLESS_BISHOP => {
                    WHITE_BISHOP_HEURISTICS[to_sq] > WHITE_BISHOP_HEURISTICS[from_sq]
                }
                COLORLESS_PAWN => WHITE_PAWN_HEURISTICS[to_sq] > WHITE_PAWN_HEURISTICS[from_sq],
                WHITE_KNIGHT_U32 => {
                    WHITE_KNIGHT_HEURISTICS[to_sq] > WHITE_KNIGHT_HEURISTICS[from_sq]
                }
                COLORLESS_QUEEN => WHITE_QUEEN_HEURISTICS[to_sq] > WHITE_QUEEN_HEURISTICS[from_sq],
                COLORLESS_ROOK => WHITE_ROOK_HEURISTICS[to_sq] > WHITE_ROOK_HEURISTICS[from_sq],
                COLORLESS_KING => WHITE_KING_HEURISTICS[to_sq] > WHITE_KING_HEURISTICS[from_sq],
                other => unreachable!("bits: {other}"),
            }
        } else {
            match t {
                COLORLESS_BISHOP => {
                    BLACK_BISHOP_HEURISTICS[to_sq] > BLACK_BISHOP_HEURISTICS[from_sq]
                }
                COLORLESS_PAWN => BLACK_PAWN_HEURISTICS[to_sq] > BLACK_PAWN_HEURISTICS[from_sq],
                COLORLESS_KNIGHT => {
                    BLACK_KNIGHT_HEURISTICS[to_sq] > BLACK_KNIGHT_HEURISTICS[from_sq]
                }
                COLORLESS_QUEEN => BLACK_QUEEN_HEURISTICS[to_sq] > BLACK_QUEEN_HEURISTICS[from_sq],
                COLORLESS_ROOK => BLACK_ROOK_HEURISTICS[to_sq] > BLACK_ROOK_HEURISTICS[from_sq],
                COLORLESS_KING => BLACK_KING_HEURISTICS[to_sq] > BLACK_KING_HEURISTICS[from_sq],
                _ => unreachable!(),
            }
        };
    }
}
