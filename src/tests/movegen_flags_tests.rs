#[allow(unused)]
use crate::{constants::attacks::*, converters::fen_converter::fen_to_board, moves::MoveList};

#[test]
fn direct_check_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let (mut board, mut state) =
        fen_to_board("r3kbnr/ppp2ppp/2n5/3q4/3P2b1/5N2/PP3PPP/RNBQKB1R w KQkq - 1 7");

    board.total_occupancy();
    board.update_full_cache();
    board.calculate_check_restrictions(&mut state, 16);
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.queen_moves(8, &mut moves, &state, false);
}
#[test]
fn direct_check_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/pp2pppp/2p5/1B1p4/4P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 3");

    board.total_occupancy();
    board.update_full_cache();
    state.calculate_check_squares(board.black_king_square as usize, board.total_occupancy, 16);
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.bishop_moves(8, &mut moves, &state, false);
    println!("moves: {:?}", moves.pseudo_moves);
}
