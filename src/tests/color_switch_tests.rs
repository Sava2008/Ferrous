#[allow(unused)]
use crate::converters::fen_converter::fen_to_board;

#[test]
fn color_switch_test1() -> () {
    let (mut board, mut state) =
        fen_to_board("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3");
    let (board1, state1) = (board.clone(), state.clone());

    board.perform_move(1669, &mut state, &mut 0, &mut 0); // Bc4
    assert_eq!(state.whose_turn, 16);
    let (board2, state2) = (board.clone(), state.clone());
    board.perform_move(3388, &mut state, &mut 0, &mut 0); // Ke7?
    assert_eq!(state.whose_turn, 8);
    let (board3, state3) = (board.clone(), state.clone());
    board.perform_move(4484, &mut state, &mut 0, &mut 0); // O-O
    assert_eq!(state.whose_turn, 16);

    board.cancel_move(&mut state, &mut 0, &mut 0);
    assert_eq!(board, board3);
    assert_eq!(state, state3);
    assert_eq!(state.whose_turn, 8);
    board.cancel_move(&mut state, &mut 0, &mut 0);
    assert_eq!(board, board2);
    assert_eq!(state, state2);
    assert_eq!(state.whose_turn, 16);
    board.cancel_move(&mut state, &mut 0, &mut 0);
    assert_eq!(board, board1);
    assert_eq!(state, state1);
    assert_eq!(state.whose_turn, 8);
}
