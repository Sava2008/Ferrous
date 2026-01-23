#[allow(unused_imports)]
use crate::{
    board::Board,
    enums::PieceColor,
    gamestate::{CastlingRights, CheckInfo, GameState, PinInfo},
    initialize_sliding_attack_tables,
};

#[test]
pub fn pin_test1() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0b0000000000000000000000000000000000000000000000000000010000000000,
        white_bishops: 0,
        white_queens: 0b0000000000000000000000000000000000001000000000000000000000000000,
        white_rooks: 0,
        white_king: 0b0000000000000000000000000000000000000000000001000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0b0000000000000000000000000001000000000000000000000000000000000000,
        black_queens: 0,
        black_rooks: 0b0000000000000000000000000000100000000000000000000000000000000100,
        black_king: 0b0000000000000000000010000000000000000000000000000000000000000000,
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
        whose_turn: PieceColor::White,
        result: crate::enums::GameResult::Going,
    };
    state.pin_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::White);
    assert_eq!(state.pin_info.black_pinned_pieces, vec![35]);
    assert_eq!(state.pin_info.white_pinned_pieces, vec![10, 27]);
}

#[test]
pub fn pin_test2() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0b0000000000000000000010000000000000000000000000000000001000000000,
        white_queens: 0b0000010000000000000000000000000000000000000000000000000000000000,
        white_rooks: 0b0000000000000000000000000000000000000000000000001000000000000000,
        white_king: 0b0000000000001000000000000000000000000000000000000000000000000000,
        black_pawns: 0b0000000011000000000000000000000000000000000000000000000000000000,
        black_knights: 0b0100000000000000000000000000000000000000000000000000000000000000,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0b0000000000000000000000000000000000000000000000000000000000001000,
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
        whose_turn: PieceColor::White,
        result: crate::enums::GameResult::Going,
    };
    state.pin_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::White);
    assert_eq!(state.pin_info.black_pinned_pieces, vec![55, 62, 54]);
    assert_eq!(state.pin_info.white_pinned_pieces, vec![43]);
}

#[test]
fn check_test1() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0b0000000000000000000001000000000000000000000000000000000000000000,
        white_bishops: 0,
        white_queens: 0b0000000000000001000000000000000000000000000000000000000000000000,
        white_rooks: 0,
        white_king: 0b1000000000000000000000000000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000010000000000000000000000000000000000000000000000000000,
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
        whose_turn: PieceColor::Black,
        result: crate::enums::GameResult::Going,
    };
    state.check_info.update(&board, &PieceColor::Black);
    assert_eq!(
        (
            state.check_info.first_checker.unwrap(),
            state.check_info.second_checker.unwrap()
        ),
        (48, 42)
    );
}

#[test]
fn check_test2() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0b0000000000000000000000000000000000000000000000000000000000001000,
        black_queens: 0b0000000000000000010000000000000000000000000000000000000000000000,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
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
        whose_turn: PieceColor::Black,
        result: crate::enums::GameResult::Going,
    };
    state.check_info.update(&board, &PieceColor::White);
    assert_eq!(
        (
            state.check_info.first_checker.unwrap(),
            state.check_info.second_checker.unwrap()
        ),
        (46, 3)
    );
}
