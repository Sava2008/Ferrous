use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    board_geometry_templates::Bitboard,
    constants::{
        heuristics::*,
        piece_values::{BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE},
    },
};

impl Engine {
    pub fn count_material(board: &Board) -> i32 {
        return (board.white_pawns.count_ones() as i32) * PAWN_VALUE
            + (board.white_knights.count_ones() as i32) * KNIGHT_VALUE
            + (board.white_bishops.count_ones() as i32) * BISHOP_VALUE
            + (board.white_rooks.count_ones() as i32) * ROOK_VALUE
            + (board.white_queens.count_ones() as i32) * QUEEN_VALUE
            - (board.black_pawns.count_ones() as i32) * PAWN_VALUE
            - (board.black_knights.count_ones() as i32) * KNIGHT_VALUE
            - (board.black_bishops.count_ones() as i32) * BISHOP_VALUE
            - (board.black_rooks.count_ones() as i32) * ROOK_VALUE
            - (board.black_queens.count_ones() as i32) * QUEEN_VALUE;
    }

    pub fn evaluate(&mut self, board: &Board) -> () {
        self.evaluation = 0;
        let mut p: Bitboard = board.white_bishops;
        while p != 0 {
            self.evaluation += WHITE_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_knights;
        while p != 0 {
            self.evaluation += WHITE_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_rooks;
        while p != 0 {
            self.evaluation += WHITE_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_pawns;
        while p != 0 {
            self.evaluation += WHITE_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_queens;
        while p != 0 {
            self.evaluation += WHITE_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_king;
        while p != 0 {
            self.evaluation += WHITE_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_bishops;
        while p != 0 {
            self.evaluation += BLACK_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_knights;
        while p != 0 {
            self.evaluation += BLACK_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_rooks;
        while p != 0 {
            self.evaluation += BLACK_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_pawns;
        while p != 0 {
            self.evaluation += BLACK_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_queens;
        while p != 0 {
            self.evaluation += BLACK_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_king;
        while p != 0 {
            self.evaluation += BLACK_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        self.evaluation += Self::count_material(&board);
    }
}
