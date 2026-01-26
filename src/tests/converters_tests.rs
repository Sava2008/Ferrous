#[allow(unused_imports)]
use crate::{
    board::Board,
    converters::fen_converter::board_to_fen,
    enums::PieceColor,
    gamestate::{CastlingRights, CheckInfo, GameState, PinInfo},
};

#[test]
fn board_to_fen_test1() -> () {
    let mut test_board: Board = Board {
        white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
        white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
        white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
        white_queens: 0b0000000000000000000000000000000000000000000000000000000000001000,
        white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
        white_king: 0b0000000000000000000000000000000000000000000000000000000000010000,
        black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
        black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
        black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
        black_queens: 0b0000100000000000000000000000000000000000000000000000000000000000,
        black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
        black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
        white_occupancy: None,
        black_occupancy: None,
        total_occupancy: None,
    };
    test_board.white_occupancy();
    test_board.black_occupancy();
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
        board_to_fen(&test_board, &test_state)
    );
}
