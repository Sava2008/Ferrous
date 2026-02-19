use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    gamestate::{GameState, PieceMove},
};

impl Engine {
    pub fn generate_captures(_board: &Board, _state: &GameState) -> Vec<PieceMove> {
        return Vec::new();
    }
}
