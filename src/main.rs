use crate::{
    board::Board,
    constants::attacks::initialize_sliding_attack_tables,
    gamestate::{CastlingRights, CheckInfo, GameState, PinInfo},
};

pub mod alpha_beta_pruning;
pub mod board;
pub mod board_geometry_templates;
pub mod constants;
pub mod converters;
pub mod enums;
pub mod evaluation;
pub mod gamestate;
pub mod moves;
pub mod tests;

fn main() {
    initialize_sliding_attack_tables();
}
