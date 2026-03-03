use std::hint::unreachable_unchecked;

use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    board_geometry_templates::{FROM_MASK, PIECE_TYPE_MASK, TO_MASK, TO_SHIFT},
};

impl Engine {
    pub fn move_priority(&self, board: &Board, m: &u16, depth: usize) -> u16 {
        let mut priority_key: u16 = 0;
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
            let victim_value: u16 = Self::get_piece_value(dest & PIECE_TYPE_MASK) as u16;
            let attacker_value: u16 = Self::get_piece_value(initial_pos & PIECE_TYPE_MASK) as u16;
            return (victim_value * 6 + (5 - attacker_value)) as u16;
        }
        if self.killer_moves[depth][0] == Some(*m) || self.killer_moves[depth][1] == Some(*m) {
            priority_key += 100;
        }
        /*if Self::does_improve_piece(&initial_pos, &m) {
            priority_key += 1;
        }*/
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
    /*
    #[inline(always)]
    #[allow(unused)]
    fn does_improve_piece(piece: &(PieceColor, PieceType), m: &u16) -> bool {
        let (from_sq, to_sq): (usize, Bitboard) = (
            (m & FROM_MASK) as usize,
            ((m & TO_MASK) >> TO_SHIFT) as Bitboard,
        );
        return match piece.1 {
            PieceType::Bishop => match piece.0 {
                PieceColor::Black => unsafe { BLACK_BISHOP_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_BISHOP_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
            PieceType::Pawn => match piece.0 {
                PieceColor::Black => unsafe { BLACK_PAWN_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_PAWN_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
            PieceType::Knight => match piece.0 {
                PieceColor::Black => unsafe { BLACK_KNIGHT_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_KNIGHT_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
            PieceType::Queen => match piece.0 {
                PieceColor::Black => unsafe { BLACK_QUEEN_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_QUEEN_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
            PieceType::Rook => match piece.0 {
                PieceColor::Black => unsafe { BLACK_ROOK_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_ROOK_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
            PieceType::King => match piece.0 {
                PieceColor::Black => unsafe { BLACK_KING_IMPROVEMENTS[from_sq] & to_sq != 0 },
                PieceColor::White => unsafe { WHITE_KING_IMPROVEMENTS[from_sq] & to_sq != 0 },
            },
        };
    }*/
}
