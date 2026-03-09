#[allow(unused)]
use crate::{
    alpha_beta_pruning::Engine, board_geometry_templates::*, constants::attacks::*,
    converters::fen_converter::fen_to_board, moves::MoveList,
};

#[test]
fn checkmate_in_two_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("Q5r1/5p1p/4kp2/4p3/8/N6n/PP3PrP/3R1R1K b - - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    let mut engine_depth_8: Engine = Engine {
        side: 16,
        depth: 8,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let mut opponent_engine: Engine = Engine {
        side: 8,
        depth: 7,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let engine_move: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (engine_move & FROM_MASK, (engine_move & TO_MASK) >> TO_SHIFT);
    assert_eq!(from, 14);
    assert_eq!(to, 6);
    board.perform_move(engine_move, &mut state, 16, &mut 0);
    let response: u32 = opponent_engine.find_best_move(&board, &mut state).unwrap();
    board.perform_move(response, &mut state, 8, &mut 0);
    let checkmate_engine_move = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        checkmate_engine_move & FROM_MASK,
        (checkmate_engine_move & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 23);
    assert_eq!(to, 13);
}

#[test]
fn checkmate_in_three_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r3k2r/p1p2pp1/1p2p3/8/6pq/3PBP2/PPP1Q1P1/R2R2K1 b kq - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    let mut engine_depth_8: Engine = Engine {
        side: 16,
        depth: 8,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let mut opponent_engine: Engine = Engine {
        side: 8,
        depth: 7,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let engine_move: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (engine_move & FROM_MASK, (engine_move & TO_MASK) >> TO_SHIFT);
    assert_eq!(from, 31);
    assert_eq!(to, 7);
    board.perform_move(engine_move, &mut state, 16, &mut 0);

    let response: u32 = opponent_engine.find_best_move(&board, &mut state).unwrap();
    board.perform_move(response, &mut state, 8, &mut 0);

    let engine_move2: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        engine_move2 & FROM_MASK,
        (engine_move2 & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 30);
    assert_eq!(to, 22);
    board.perform_move(engine_move2, &mut state, 16, &mut 0);

    let response: u32 = opponent_engine.find_best_move(&board, &mut state).unwrap();
    board.perform_move(response, &mut state, 8, &mut 0);

    let engine_move3: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        engine_move3 & FROM_MASK,
        (engine_move3 & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 7);
    assert_eq!(to, 31);
}

#[test]
fn checkmate_in_four_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("8/6k1/1p3p2/5Q2/1P3P2/2q3Pp/4r2P/5RK1 b - - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    let mut engine_depth_8: Engine = Engine {
        side: 16,
        depth: 8,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let mut opponent_engine: Engine = Engine {
        side: 8,
        depth: 7,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let engine_move: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (engine_move & FROM_MASK, (engine_move & TO_MASK) >> TO_SHIFT);
    assert_eq!(from, 18);
    assert_eq!(to, 20);
    println!("{from}: {to}");

    board.perform_move(engine_move, &mut state, 16, &mut 0);

    let response: u32 = opponent_engine.find_best_move(&board, &mut state).unwrap();
    board.perform_move(response, &mut state, 8, &mut 0);
    println!(
        "response {}: {}",
        response & FROM_MASK,
        (response & TO_MASK) >> TO_SHIFT
    );

    let engine_move2: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        engine_move2 & FROM_MASK,
        (engine_move2 & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 20);
    assert_eq!(to, 21);
    println!("{from}: {to}");
    board.perform_move(engine_move2, &mut state, 16, &mut 0);

    let response = 5
        | (21 << TO_SHIFT)
        | (BLACK_QUEEN_U32 << CAPTURED_PIECE_TYPE_SHIFT)
        | (WHITE_ROOK_U32 << MOVING_PIECE_TYPE_SHIFT);
    board.perform_move(response, &mut state, 8, &mut 0);
    println!(
        "response {}: {}",
        response & FROM_MASK,
        (response & TO_MASK) >> TO_SHIFT
    );

    let engine_move3: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        engine_move3 & FROM_MASK,
        (engine_move3 & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 12);
    assert_eq!(to, 4);
    println!("{from}: {to}");
    board.perform_move(engine_move3, &mut state, 16, &mut 0);

    let response: u32 = opponent_engine.find_best_move(&board, &mut state).unwrap();
    board.perform_move(response, &mut state, 8, &mut 0);
    println!(
        "response {}: {}",
        response & FROM_MASK,
        (response & TO_MASK) >> TO_SHIFT
    );

    let engine_move4: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (
        engine_move4 & FROM_MASK,
        (engine_move4 & TO_MASK) >> TO_SHIFT,
    );
    assert_eq!(from, 4);
    assert_eq!(to, 5);
    println!("{from}: {to}");
}
#[test]
fn checkmate_in_five_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1r3k1/5pPp/1n6/q3PN2/1p3P2/1Ppp3P/P1Q5/1K2B2R w - - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    let mut engine_depth_8: Engine = Engine {
        side: 8,
        depth: 10,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16],
        move_scores: [[0; 192]; 16],
    };
    let engine_move: u32 = engine_depth_8.find_best_move(&board, &mut state).unwrap();
    let (from, to) = (engine_move & FROM_MASK, (engine_move & TO_MASK) >> TO_SHIFT);
    assert_eq!(from, 37);
    assert_eq!(to, 47);
    println!("{from}: {to}");
}
