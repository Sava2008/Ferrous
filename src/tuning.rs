use std::hint::unreachable_unchecked;

use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    board_geometry_templates::{
        BLACK_BISHOP_U8, BLACK_KING_U8, BLACK_KNIGHT_U8, BLACK_PAWN_U8, BLACK_QUEEN_U8,
        BLACK_ROOK_U8, FROM_MASK, PIECE_TYPE_MASK, TO_MASK, TO_SHIFT, WHITE_BISHOP_U8,
        WHITE_KING_U8, WHITE_KNIGHT_U8, WHITE_PAWN_U8, WHITE_QUEEN_U8, WHITE_ROOK_U8,
    },
    constants::{attacks::MVV_LVA, heuristics::*},
};

impl Engine {
    pub fn move_priority(&self, board: &Board, m: &u16, depth: usize) -> i16 {
        let mut priority_key: i16 = 0;
        let (initial_pos, final_pos): (u8, Option<u8>) = (
            if let Some(a) = board.piece_at(&(m & FROM_MASK)) {
                a
            } else {
                println!("board: {board:?}");
                panic!();
            },
            board.piece_at(&((m & TO_MASK) >> TO_SHIFT)),
        );
        if let Some(dest) = final_pos {
            let victim_value: usize = Self::get_piece_value(dest & PIECE_TYPE_MASK) as usize;
            let attacker_value: usize =
                Self::get_piece_value(initial_pos & PIECE_TYPE_MASK) as usize;
            priority_key -= unsafe { MVV_LVA[victim_value][attacker_value] };
        }
        if self.killer_moves[depth][0] == Some(*m) || self.killer_moves[depth][1] == Some(*m) {
            priority_key -= 100;
        }
        if Self::does_improve_piece(&initial_pos, &m) {
            priority_key -= 5;
        }
        return priority_key;
    }
    #[inline(always)]
    fn get_piece_value(piece_type: u8) -> u8 {
        match piece_type & PIECE_TYPE_MASK {
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
    fn does_improve_piece(piece: &u8, m: &u16) -> bool {
        let (from_sq, to_sq): (usize, usize) = (
            (m & FROM_MASK) as usize,
            ((m & TO_MASK) >> TO_SHIFT) as usize,
        );
        return match *piece {
            BLACK_BISHOP_U8 => BLACK_BISHOP_HEURISTICS[to_sq] > BLACK_BISHOP_HEURISTICS[from_sq],
            WHITE_BISHOP_U8 => WHITE_BISHOP_HEURISTICS[to_sq] > WHITE_BISHOP_HEURISTICS[from_sq],
            BLACK_PAWN_U8 => BLACK_PAWN_HEURISTICS[to_sq] > BLACK_PAWN_HEURISTICS[from_sq],
            WHITE_PAWN_U8 => WHITE_PAWN_HEURISTICS[to_sq] > WHITE_PAWN_HEURISTICS[from_sq],
            BLACK_KNIGHT_U8 => BLACK_KNIGHT_HEURISTICS[to_sq] > BLACK_KNIGHT_HEURISTICS[from_sq],
            WHITE_KNIGHT_U8 => WHITE_KNIGHT_HEURISTICS[to_sq] > WHITE_KNIGHT_HEURISTICS[from_sq],
            BLACK_QUEEN_U8 => BLACK_QUEEN_HEURISTICS[to_sq] > BLACK_QUEEN_HEURISTICS[from_sq],
            WHITE_QUEEN_U8 => WHITE_QUEEN_HEURISTICS[to_sq] > WHITE_QUEEN_HEURISTICS[from_sq],
            BLACK_ROOK_U8 => BLACK_ROOK_HEURISTICS[to_sq] > BLACK_ROOK_HEURISTICS[from_sq],
            WHITE_ROOK_U8 => WHITE_ROOK_HEURISTICS[to_sq] > WHITE_ROOK_HEURISTICS[from_sq],
            BLACK_KING_U8 => BLACK_KING_HEURISTICS[to_sq] > BLACK_KING_HEURISTICS[from_sq],
            WHITE_KING_U8 => WHITE_KING_HEURISTICS[to_sq] > WHITE_KING_HEURISTICS[from_sq],
            _ => unreachable!(),
        };
    }
}
