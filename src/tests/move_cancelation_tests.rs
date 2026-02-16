#[allow(unused_imports)]
use crate::{
    board::Board,
    enums::PieceColor,
    gamestate::{GameState, PieceMove},
};

#[test]
fn cancelation_test1() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state); // en passant update move
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test2() -> () {
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0b1000,
        white_queens: 0,
        white_rooks: 0b10000000000,
        white_king: 0b10000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b100000000000000000,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 10, to: 26 }, &mut state); // check
    board.cancel_move(&mut state);
    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 10, to: 18 }, &mut state); // double check
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test3() -> () {
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0b10000001,
        white_king: 0b10000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
        black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 4, to: 6 }, &mut state); // white two zeros
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 4, to: 2 }, &mut state); // white three zeros
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 60, to: 62 }, &mut state); // black two zeros
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 60, to: 58 }, &mut state); // black three zeros
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test4() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);
    board.perform_move(&PieceMove { from: 62, to: 47 }, &mut state);
    board.perform_move(&PieceMove { from: 28, to: 36 }, &mut state);
    board.perform_move(&PieceMove { from: 51, to: 35 }, &mut state);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 36, to: 43 }, &mut state);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test5() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    board.perform_move(&PieceMove { from: 11, to: 27 }, &mut state);
    board.perform_move(&PieceMove { from: 52, to: 36 }, &mut state);
    board.perform_move(&PieceMove { from: 6, to: 21 }, &mut state);
    board.perform_move(&PieceMove { from: 62, to: 45 }, &mut state);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 2, to: 38 }, &mut state);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}
