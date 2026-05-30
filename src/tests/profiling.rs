#[allow(unused)]
use crate::{
    board::Board, converters::fen_converter::fen_to_board, gamestate::GameState, moves::MoveList,
    search::Engine, transposition::TranspositionTable,
};
use std::time::{Duration, Instant};

#[test]
pub fn engine_speed_test() -> () {
    let mut test_engine: Engine = Engine {
        side: 8,
        depth: 9,
        evaluation: 0,
        killer_moves: [[None; 2]; 32],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 32],
        history_heuristics: [0; 4096],
        move_scores: [[0; 192]; 32],
        quiescence_limitation: 9,
        current_hash: 0,
        transposition_table: TranspositionTable::new(),
    };
    let (mut board1, mut state1) =
        fen_to_board("r1bq1rk1/pp3ppp/2nbpn2/2pp4/3P4/1P1BPN2/PBPN1PPP/R2QK2R w KQ - 5 8");
    let (mut board2, mut state2) =
        fen_to_board("r1b1k2r/ppppqppp/2n2n2/8/1bB1P3/2N2N2/PPQ2PPP/R1B1K2R w KQkq - 3 8");
    let (mut board3, mut state3) =
        fen_to_board("r3k1nr/p1p1qppp/2pb4/3pp3/5Pb1/1P2PNP1/PBPP3P/RN1QK2R w KQkq - 3 9");
    let (mut board4, mut state4) =
        fen_to_board("rnb1kb1r/1p3ppp/1qpp1n2/p7/4PP2/1NN5/PPPB2PP/R2QKB1R w KQkq - 0 9");
    let (mut board5, mut state5) =
        fen_to_board("rnq1k2r/pp2ppbp/3p2p1/3NP2n/1PPN1P2/8/P3K1PP/R1BQ3R w kq - 1 13");

    let mut prompt_counts: [u128; 5] = [0; 5];

    let (mut total_movegen_time, mut total_make_time, mut total_unmake_time) =
        (Duration::ZERO, Duration::ZERO, Duration::ZERO);
    movegen_speed_test(
        &mut total_movegen_time,
        &mut total_make_time,
        &mut total_unmake_time,
        &mut test_engine,
        &mut board1,
        &mut state1,
    );
    prompt_counts[0] =
        test_engine.move_lists[test_engine.depth as usize].first_not_occupied as u128;
    movegen_speed_test(
        &mut total_movegen_time,
        &mut total_make_time,
        &mut total_unmake_time,
        &mut test_engine,
        &mut board2,
        &mut state2,
    );
    prompt_counts[1] =
        test_engine.move_lists[test_engine.depth as usize].first_not_occupied as u128;
    movegen_speed_test(
        &mut total_movegen_time,
        &mut total_make_time,
        &mut total_unmake_time,
        &mut test_engine,
        &mut board3,
        &mut state3,
    );
    prompt_counts[2] =
        test_engine.move_lists[test_engine.depth as usize].first_not_occupied as u128;
    movegen_speed_test(
        &mut total_movegen_time,
        &mut total_make_time,
        &mut total_unmake_time,
        &mut test_engine,
        &mut board4,
        &mut state4,
    );
    prompt_counts[3] =
        test_engine.move_lists[test_engine.depth as usize].first_not_occupied as u128;
    movegen_speed_test(
        &mut total_movegen_time,
        &mut total_make_time,
        &mut total_unmake_time,
        &mut test_engine,
        &mut board5,
        &mut state5,
    );
    prompt_counts[4] =
        test_engine.move_lists[test_engine.depth as usize].first_not_occupied as u128;

    for i in 0..5 {
        println!(
            "average results:\nmovegen per position: {}ns\nperform_move time per move: {}ns\ncancel_move time per move: {}ns",
            total_movegen_time.as_nanos() / prompt_counts[i],
            total_make_time.as_nanos() / prompt_counts[i],
            total_unmake_time.as_nanos() / prompt_counts[i]
        );
    }
    is_square_attacked_speed_test(&board1);
    is_square_attacked_speed_test(&board2);
    is_square_attacked_speed_test(&board3);
    is_square_attacked_speed_test(&board4);
    is_square_attacked_speed_test(&board5);
}

#[allow(unused)]
fn movegen_speed_test(
    movegen_time: &mut Duration,
    make_move_time: &mut Duration,
    unmake_move_time: &mut Duration,
    engine: &mut Engine,
    board: &mut Board,
    state: &mut GameState,
) -> () {
    let depth_as_index: usize = engine.depth as usize;
    let timer_start: Instant = Instant::now();
    engine.generate_pseudo_legal_moves(engine.side, board, state, depth_as_index, false);
    *movegen_time += timer_start.elapsed();
    for i in 0..engine.move_lists[depth_as_index].first_not_occupied {
        make_unmake_speed_test(
            board,
            state,
            engine.move_lists[depth_as_index].pseudo_moves[i],
            engine.side,
            make_move_time,
            unmake_move_time,
            &mut engine.current_hash,
        );
    }
}

#[allow(unused)]
fn make_unmake_speed_test(
    board: &mut Board,
    state: &mut GameState,
    piece_move: u16,
    color: u16,
    make_move_time_results: &mut Duration,
    unmake_move_time_results: &mut Duration,
    current_hash: &mut u64,
) -> () {
    let start_timer: Instant = Instant::now();
    board.perform_move(piece_move, state, color, &mut 0, current_hash);
    *make_move_time_results += start_timer.elapsed();

    let start_timer: Instant = Instant::now();
    board.cancel_move(state, color, &mut 0, current_hash);
    *unmake_move_time_results += start_timer.elapsed();
}

#[allow(unused)]
fn is_square_attacked_speed_test(board: &Board) -> () {
    let mut total_time: Duration = Duration::ZERO;
    for sq in 0..64 {
        let start_timer: Instant = Instant::now();
        board.is_square_attacked(sq, 8);
        total_time += start_timer.elapsed();

        let start_timer: Instant = Instant::now();
        board.is_square_attacked(sq, 16);
        total_time += start_timer.elapsed();
    }
    println!(
        "average attack check time: {}ns",
        total_time.as_nanos() / 128
    );
}
