use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::piece_values::{BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE},
};

impl Engine {
    pub fn count_material(board: &Board) -> i32 {
        let mut material: i32 = 0;

        material += (board.white_pawns.count_ones() as i32) * PAWN_VALUE;
        material += (board.white_knights.count_ones() as i32) * KNIGHT_VALUE;
        material += (board.white_bishops.count_ones() as i32) * BISHOP_VALUE;
        material += (board.white_rooks.count_ones() as i32) * ROOK_VALUE;
        material += (board.white_queens.count_ones() as i32) * QUEEN_VALUE;

        material -= (board.black_pawns.count_ones() as i32) * PAWN_VALUE;
        material -= (board.black_knights.count_ones() as i32) * KNIGHT_VALUE;
        material -= (board.black_bishops.count_ones() as i32) * BISHOP_VALUE;
        material -= (board.black_rooks.count_ones() as i32) * ROOK_VALUE;
        material -= (board.black_queens.count_ones() as i32) * QUEEN_VALUE;
        return material;
    }

    pub fn evaluate(_board: &Board) -> () {}
}
