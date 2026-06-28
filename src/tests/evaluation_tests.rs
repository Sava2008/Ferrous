#[allow(unused_imports)]
use crate::{
    board_geometry_templates::{TO_SHIFT, from_square, to_square},
    constants::attacks::*,
    converters::fen_converter::fen_to_board,
    moves::MoveList,
    search::Engine,
    transposition::TranspositionTable,
};

#[allow(unused)]
use std::time::Duration;
#[test]
fn evaluation_test_capture() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbq1rk1/ppppnppp/4p3/8/3P1B2/P1b1PN2/1PP2PPP/R2QKB1R w KQ - 0 7");

    board.total_occupancy();
    board.update_full_cache();

    let mut test_engine: Engine = Engine::new(8, 8);

    test_engine.evaluate(&board);
    let eval: i32 = test_engine.evaluation;
    board.perform_move(
        9 | (18 << TO_SHIFT),
        &mut state,
        &mut test_engine.evaluation,
        &mut 0,
    );

    board.cancel_move(&mut state, &mut test_engine.evaluation, &mut 0);
    assert_eq!(eval, test_engine.evaluation);
}

#[test]
fn confirm_sane_eval() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, state) =
        fen_to_board("rn1q1rk1/pppbppbp/5np1/3p4/3PP3/2NB1N2/PPP2PPP/R1BQK2R b KQ - 0 7");

    board.total_occupancy();
    board.update_full_cache();

    let mut test_engine: Engine = Engine::new(16, 8);

    test_engine.evaluate(&board);
    let m: u16 = test_engine
        .find_best_move(board, state, Duration::from_secs(10), 100)
        .unwrap();
    println!("move: {} {}", from_square(m), to_square(m));
}
