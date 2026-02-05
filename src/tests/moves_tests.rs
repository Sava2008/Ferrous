#[allow(unused_imports)]
use crate::{
    board::Board,
    constants::attacks::initialize_sliding_attack_tables,
    enums::PieceColor,
    gamestate::{CastlingRights, CheckInfo, GameState, PieceMove, PinInfo},
};

#[test]
fn king_moves_test1() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0b0000000000000000000000000000000000000000000000000000010000000000,
        white_rooks: 0,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights::new(),
        fifty_moves_rule_counter: 7,
        check_info: CheckInfo::new(),
        pin_info: PinInfo::new(),
        moves_history: Vec::new(),
        total_moves_amount: 15,
        whose_turn: PieceColor::Black,
        result: crate::enums::GameResult::Going,
        check_contraints: 0,
    };
    assert_eq!(board.king_moves(&state, &PieceColor::Black), Vec::new());
}

#[test]
fn king_moves_test2() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0b0000000000000000000000000000000000000000000000000000010000000000,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights::new(),
        fifty_moves_rule_counter: 7,
        check_info: CheckInfo::new(),
        pin_info: PinInfo::new(),
        moves_history: Vec::new(),
        total_moves_amount: 15,
        whose_turn: PieceColor::Black,
        result: crate::enums::GameResult::Going,
        check_contraints: 0,
    };
    assert_eq!(
        board.king_moves(&state, &PieceColor::Black),
        vec![PieceMove { from: 0, to: 1 }]
    );
}

#[test]
fn king_moves_test3() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0b0000000000000000000000000000000000000000000000000000000000000100,
        white_bishops: 0b0000000000000000000000000000000000000000000000000000010000000000,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights::new(),
        fifty_moves_rule_counter: 7,
        check_info: CheckInfo::new(),
        pin_info: PinInfo::new(),
        moves_history: Vec::new(),
        total_moves_amount: 15,
        whose_turn: PieceColor::Black,
        result: crate::enums::GameResult::Going,
        check_contraints: 0,
    };
    assert_eq!(
        board.king_moves(&state, &PieceColor::Black),
        vec![PieceMove { from: 0, to: 9 }]
    );
}
