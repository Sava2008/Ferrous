#[allow(unused_imports)]
use crate::{
    board::Board,
    converters::fen_converter::{board_to_fen, fen_to_board},
    enums::PieceColor,
    gamestate::{CastlingRights, CheckInfo, GameState, PinInfo},
};

#[test]
fn board_to_fen_test1() -> () {
    let mut test_board: Board = Board::set();
    test_board.total_occupancy();

    let test_state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights::new(),
        fifty_moves_rule_counter: 0,
        check_info: CheckInfo::new(),
        pin_info: PinInfo::new(),
        moves_history: Vec::new(),
        total_moves_amount: 1,
        whose_turn: PieceColor::White,
        result: crate::enums::GameResult::Going,
        check_contraints: 0,
    };

    assert_eq!(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        board_to_fen(&test_board, &test_state, &PieceColor::White)
    );
}

#[test]
fn fen_to_board_test1() -> () {
    println!(
        "{:?}",
        fen_to_board("rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3")
    );
}
