#[allow(unused_imports)]
use crate::{constants::attacks::*, converters::fen_converter::fen_to_board};

#[test]
fn check_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let (mut board, mut state) =
        fen_to_board("r3kbnr/ppp2ppp/2n5/3q4/3P2b1/5N2/PP3PPP/RNBQKB1R w KQkq - 1 7");

    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(29443, &mut state, 8, &mut 0, &mut 0);
    // assert_ne!(state.black_legal_squares_mask, u64::MAX);
    println!("state: {:?}", state);
}
