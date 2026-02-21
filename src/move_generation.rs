use crate::{alpha_beta_pruning::Engine, board::Board, gamestate::GameState};

impl Engine {
    pub fn generate_captures(_board: &Board, _state: &GameState) -> Vec<u16> {
        return Vec::new();
    }
}
