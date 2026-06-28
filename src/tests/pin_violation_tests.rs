#[allow(unused_imports)]
use crate::{
    board_geometry_templates::{TO_SHIFT, from_square, to_square},
    constants::attacks::*,
    converters::fen_converter::fen_to_board,
    moves::MoveList,
};
#[test]
fn avoiding_check_and_king_exposure() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut test_board, test_state) = fen_to_board("1k2q3/b7/4Q2q/6N1/3B4/1rNBK3/8/6n1 w - - 0 1");
    test_board.total_occupancy();
    test_board.update_full_cache();

    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    test_board.knight_moves(8, &mut legal_moves, &test_state, false);
    test_board.bishop_moves(8, &mut legal_moves, &test_state, false);
    test_board.king_moves(&test_state, 8, &mut legal_moves, false);
    test_board.queen_moves(8, &mut legal_moves, &test_state, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}

#[test]
fn en_passant_pin_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut test_board, mut test_state) = fen_to_board("7k/2p5/8/KP3q2/8/8/8/3Q4 b - - 0 1");
    test_board.total_occupancy();
    test_board.update_full_cache();

    test_board.perform_move(50 | (34 << TO_SHIFT), &mut test_state, &mut 0, &mut 0);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    test_board.pawn_moves(&test_state, 8, &mut legal_moves, false);
    test_board.king_moves(&test_state, 8, &mut legal_moves, false);
    test_board.queen_moves(8, &mut legal_moves, &test_state, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn en_passant_pin_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut test_board, mut test_state) = fen_to_board("8/2k5/8/8/2p5/8/3P3K/2R5 w - - 0 1");
    test_board.total_occupancy();
    test_board.update_full_cache();

    test_board.perform_move(11 | (27 << TO_SHIFT), &mut test_state, &mut 0, &mut 0);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    test_board.pawn_moves(&test_state, 16, &mut legal_moves, false);
    test_board.king_moves(&test_state, 16, &mut legal_moves, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}

#[test]
fn en_passant_no_pin() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/pppp1ppp/8/8/4p3/4P3/PPPPQPPP/RNB1KBNR w KQkq - 0 3");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(11 | (27 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut legal_moves, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}

#[test]
fn en_passant_pin_test3() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(12 | (28 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut legal_moves, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn en_passant_pin_test4() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(14 | (30 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut legal_moves, false);
    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
