#[allow(unused)]
use crate::{constants::attacks::*, converters::fen_converter::fen_to_board, search::Engine};
#[allow(unused)]
use std::time::Duration;

#[test]
fn move_order_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("1r4k1/p1pqb1p1/2n4p/4n3/3p4/2N1BP2/PPQ2PKP/3RR3 w - - 0 20");
    board.total_occupancy();
    board.update_full_cache();
    let mut engine = Engine::new(8, 1);
    let _ = engine.find_best_move(&board, &mut state, Duration::from_mins(1), engine.depth);
}
