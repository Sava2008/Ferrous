#[allow(unused_imports)]
use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::attacks::{
        compute_all_lines, compute_all_rays, compute_all_rays_from,
        initialize_sliding_attack_tables,
    },
    converters::fen_converter::fen_to_board,
    enums::PieceColor,
    gamestate::{GameState, PieceMove},
};
#[allow(unused_imports)]
use std::time::Instant;

#[test]
fn engine_test_checkmate_in_one() -> () {
    initialize_sliding_attack_tables();
    compute_all_lines();
    compute_all_rays();
    compute_all_rays_from();
    let (mut board, mut state): (Board, GameState) =
        fen_to_board("8/B1k4p/2P1pp2/1P1p4/5r1n/6n1/P4P2/2RRK3 b - - 0 1");
    board.count_material();
    println!("board: {board:?}");
    let mut engine: Engine = Engine {
        side: PieceColor::Black,
        depth: 4,
        evaluation: 0,
    };
    let time = Instant::now();
    engine.evaluate(&board);
    println!("evaluation time: {:.10?}", time.elapsed());
    let engine_move: PieceMove = engine.find_best_move(&board, &mut state).unwrap();
    assert_eq!(engine_move, PieceMove { from: 31, to: 21 });
    let time = Instant::now();
    board.perform_move(&engine_move, &mut state);
    println!("perform move time: {:.10?}", time.elapsed());
    board.total_occupancy();
    let time = Instant::now();
    state.check_info.update(&board, &PieceColor::White);
    println!("check info update time: {:.10?}", time.elapsed());
    let time = Instant::now();
    state.pin_info.update(&board, &PieceColor::White);
    println!("pin info update time: {:.10?}", time.elapsed());
    let time = Instant::now();
    state.update_check_constraints(&board);
    println!("check constraints update time: {:.10?}", time.elapsed());
    let time = Instant::now();
    let engine_moves = Engine::generate_legal_moves(&PieceColor::White, &board, &state);
    println!("engine move time: {:.10?}", time.elapsed());
    assert_eq!(engine_moves.len(), 0);
    let time = Instant::now();
    board.cancel_move(&mut state);
    println!("time per cancelation: {:.10?}", time.elapsed());
}

#[test]
fn engine_test_checkmate_in_two() -> () {
    initialize_sliding_attack_tables();
    compute_all_lines();
    compute_all_rays();
    compute_all_rays_from();
    let (mut board, mut state): (Board, GameState) =
        fen_to_board("r3r1k1/2q2p2/p1b1ppBQ/1p1pn1N1/8/2P1P3/PP4PP/R5K1 w - - 0 1");
    board.count_material();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    let mut engine: Engine = Engine {
        side: PieceColor::White,
        depth: 4,
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
