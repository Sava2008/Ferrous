use crate::{board::Board, gamestate::GameState, search::Engine};

impl Engine {
    pub fn generate_captures(_board: &Board, _state: &GameState) -> Vec<u16> {
        return Vec::new();
    }
}
