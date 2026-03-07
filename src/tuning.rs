use std::hint::unreachable_unchecked;

use crate::{
    alpha_beta_pruning::Engine,
    board_geometry_templates::*,
    constants::{attacks::MVV_LVA, heuristics::*},
};

impl Engine {
    pub fn score_all_moves(&self, depth: usize, all_moves: &Vec<u32>) -> Vec<i16> {
        let mut priorities: Vec<i16> = Vec::with_capacity(all_moves.len());
        for i in 0..all_moves.len() {
            priorities.push(self.move_priority(&all_moves[i], depth));
        }
        return priorities;
    }
    pub fn move_priority(&self, m: &u32, depth: usize) -> i16 {
        let mut priority_key: i16 = 0;
        let (initial_pos, final_pos): (u32, u32) = (moving_piece_type(*m), captured_piece_type(*m));
        if final_pos != 0 {
            let victim_value: usize = Self::get_piece_value(final_pos) as usize;
            let attacker_value: usize = Self::get_piece_value(initial_pos) as usize;
            priority_key -= unsafe { MVV_LVA[victim_value][attacker_value] };
        }
        if self.killer_moves[depth][0] == Some(*m) || self.killer_moves[depth][1] == Some(*m) {
            priority_key -= 100;
        }
        if Self::does_improve_piece(*m) {
            priority_key -= 5;
        }
        return priority_key;
    }
    #[inline(always)]
    fn get_piece_value(piece_type: u32) -> u8 {
        match captured_piece(piece_type) {
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            6 => 5,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn does_improve_piece(m: u32) -> bool {
        let (from_sq, to_sq): (usize, usize) = (
            (m & FROM_MASK) as usize,
            ((m & TO_MASK) >> TO_SHIFT) as usize,
        );
        return match moving_piece(m) {
            BLACK_BISHOP_U32 => BLACK_BISHOP_HEURISTICS[to_sq] > BLACK_BISHOP_HEURISTICS[from_sq],
            WHITE_BISHOP_U32 => WHITE_BISHOP_HEURISTICS[to_sq] > WHITE_BISHOP_HEURISTICS[from_sq],
            BLACK_PAWN_U32 => BLACK_PAWN_HEURISTICS[to_sq] > BLACK_PAWN_HEURISTICS[from_sq],
            WHITE_PAWN_U32 => WHITE_PAWN_HEURISTICS[to_sq] > WHITE_PAWN_HEURISTICS[from_sq],
            BLACK_KNIGHT_U32 => BLACK_KNIGHT_HEURISTICS[to_sq] > BLACK_KNIGHT_HEURISTICS[from_sq],
            WHITE_KNIGHT_U32 => WHITE_KNIGHT_HEURISTICS[to_sq] > WHITE_KNIGHT_HEURISTICS[from_sq],
            BLACK_QUEEN_U32 => BLACK_QUEEN_HEURISTICS[to_sq] > BLACK_QUEEN_HEURISTICS[from_sq],
            WHITE_QUEEN_U32 => WHITE_QUEEN_HEURISTICS[to_sq] > WHITE_QUEEN_HEURISTICS[from_sq],
            BLACK_ROOK_U32 => BLACK_ROOK_HEURISTICS[to_sq] > BLACK_ROOK_HEURISTICS[from_sq],
            WHITE_ROOK_U32 => WHITE_ROOK_HEURISTICS[to_sq] > WHITE_ROOK_HEURISTICS[from_sq],
            BLACK_KING_U32 => BLACK_KING_HEURISTICS[to_sq] > BLACK_KING_HEURISTICS[from_sq],
            WHITE_KING_U32 => WHITE_KING_HEURISTICS[to_sq] > WHITE_KING_HEURISTICS[from_sq],
            other => unreachable!("bits: {other}"),
        };
    }
}
