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
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0b0000000000000000000000000000000000000000000000000000010000000000,
        white_bishops: 0b0000000000000000000000000000000000000000000000000000000000001000,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0b0000000000000000000000000000000000000000000000000000000000000010,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0b0100000000000000000000000000000000000000000000000000000000000000,
        black_rooks: 0b0000000000000000000000000000000000000000000000000000000001000000,
        black_king: 0b1000000000000000000000000000000000000000000000000000000000000000,
        white_occupancy: None,
        black_occupancy: None,
        total_occupancy: None,
    };
    board.total_occupancy();
    let mut state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights::new(),
        fifty_moves_rule_counter: 7,
        check_info: CheckInfo::new(),
        pin_info: PinInfo::new(),
        moves_history: Vec::new(),
        total_moves_amount: 15,
        whose_turn: enums::PieceColor::White,
    };
    state.check_info.update(&board, &state.whose_turn);
    state.pin_info.update(&board);
    println!("{:?}", state.pin_info);
}
