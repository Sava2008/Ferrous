#[allow(unused_imports)]
use crate::{
    board::Board,
    enums::PieceColor,
    gamestate::{CastlingRights, CheckInfo, GameState, PieceMove, PinInfo, PinnedPiece},
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
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
    };
    state.pin_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::White);
    assert_eq!(
        state.pin_info.black_pinned_pieces,
        vec![PinnedPiece {
            square: 35,
            pin_ray: 0b0000000000000000000000000000000000001000000000000000000000000000,
        }]
    );
    assert_eq!(
        state.pin_info.white_pinned_pieces,
        vec![
            PinnedPiece {
                square: 10,
                pin_ray: 0b0000000000000000000000000000000000000000000000000000000000000100,
            },
            PinnedPiece {
                square: 27,
                pin_ray: 0b0000000000000000000000000001000000000000000000000000000000000000,
            },
        ]
    );
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
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
    };
    state.pin_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::White);
    assert_eq!(
        state.pin_info.black_pinned_pieces,
        vec![
            PinnedPiece {
                square: 55,
                pin_ray: 0b0000000000000000100000001000000010000000100000001000000000000000
            },
            PinnedPiece {
                square: 62,
                pin_ray: 0b0011110000000000000000000000000000000000000000000000000000000000
            },
            PinnedPiece {
                square: 54,
                pin_ray: 0b0000000000000000001000000001000000001000000001000000001000000000,
            }
        ]
    );
    assert_eq!(
        state.pin_info.white_pinned_pieces,
        vec![PinnedPiece {
            square: 43,
            pin_ray: 0b0000000000000000000000000000100000001000000010000000100000001000,
        }]
    );
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
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
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
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
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

#[test]
fn block_test1() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0b0000000000000000000000000000000000000000000000001000000000000000,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0b0000000000000000000000000000000000000000000000000000000000001000,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
    };
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    assert_eq!(
        board.knight_moves(&state, &PieceColor::White),
        [
            PieceMove { from: 15, to: 21 },
            PieceMove { from: 15, to: 30 }
        ]
    );
}

#[test]
fn block_test2() -> () {
    initialize_sliding_attack_tables();
    let mut board: Board = Board {
        white_pawns: 0b0000000000000000000000000100000000000000000000000000000000000000,
        white_knights: 0,
        white_bishops: 0b0000000000000000000000000000000000000000100000000000000000000000,
        white_queens: 0,
        white_rooks: 0b0000000000000000000000000000000000000000000000000000000000010000,
        white_king: 0b0000000000000000000000001000000000000000000000000000000000000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0b0000000000000000000000000000000000000000000000000000000000001000,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b0000000000000000000000000000000000000000000000000000000000000001,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
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
        check_contraints: 0,
    };
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    assert_eq!(
        board.rook_moves(&state, &PieceColor::White),
        [PieceMove { from: 4, to: 3 }, PieceMove { from: 4, to: 12 }]
    );
    assert_eq!(
        board.bishop_moves(&state, &PieceColor::White),
        [PieceMove { from: 23, to: 30 }]
    );
    assert_eq!(board.pawn_moves(&state, &PieceColor::White).len(), 0);
}
