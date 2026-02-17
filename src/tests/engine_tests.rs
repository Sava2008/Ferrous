use crate::constants::attacks::initialize_sliding_attack_tables;
#[allow(unused_imports)]
use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    converters::fen_converter::fen_to_board,
    enums::PieceColor,
    gamestate::{GameState, PieceMove},
};

#[test]
fn engine_test_checkmate_in_one() -> () {
    initialize_sliding_attack_tables();
    let (mut board, mut state): (Board, GameState) =
        fen_to_board("8/B1k4p/2P1pp2/1P1p4/5r1n/6n1/P4P2/2RRK3 b - - 0 1");
    let mut engine: Engine = Engine {
        side: PieceColor::Black,
        depth: 6,
        evaluation: 0,
    };
    engine.evaluate(&board);
    let engine_move: PieceMove = engine.find_best_move(&board, &mut state).unwrap();
    assert_eq!(engine_move, PieceMove { from: 31, to: 21 });
    board.perform_move(&engine_move, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    assert_eq!(
        Engine::generate_legal_moves(&PieceColor::White, &board, &state).len(),
        0
    );
}

#[test]
fn engine_test_checkmate_in_two() -> () {
    initialize_sliding_attack_tables();
    let (mut board, mut state): (Board, GameState) =
        fen_to_board("r3r1k1/2q2p2/p1b1ppBQ/1p1pn1N1/8/2P1P3/PP4PP/R5K1 w - - 0 1");
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    let mut engine: Engine = Engine {
        side: PieceColor::White,
        depth: 6,
        evaluation: 0,
    };
    println!("board: {board:?}");
    println!("state: {state:?}");
    println!(
        "bishop moves: {:?}, queen moves: {:?}",
        board.bishop_moves(&state, &PieceColor::White),
        board.queen_moves(&state, &PieceColor::White)
    );
    let engine_move: PieceMove = engine.find_best_move(&board, &mut state).unwrap();
    assert_eq!(engine_move, PieceMove { from: 46, to: 55 });
    board.perform_move(&PieceMove { from: 46, to: 55 }, &mut state);
    board.perform_move(&PieceMove { from: 62, to: 63 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    let engine_move: PieceMove = engine.find_best_move(&board, &mut state).unwrap();
    assert_eq!(engine_move, PieceMove { from: 47, to: 45 });
}
