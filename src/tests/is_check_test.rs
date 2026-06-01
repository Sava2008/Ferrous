#[allow(unused_imports)]
use crate::{
    board_geometry_templates::*, constants::attacks::*, converters::fen_converter::fen_to_board,
    moves::MoveList,
};

#[test]
fn single_check_queen() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r3k2r/3pp2p/2p5/pP2N1p1/3P4/5Q2/2P5/R3K2R w KQkq a6 0 2");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(21 | (39 << TO_SHIFT), &mut state, 8, &mut 0, &mut 0);
    println!("board: {:?}", board.cached_pieces);
    println!(
        "legal moves mask(black): {:b}",
        state.black_legal_squares_mask
    );
    println!(
        "legal moves mask (white): {:b}\n",
        state.white_legal_squares_mask
    );
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 16, &mut legal_moves, false);
    board.pawn_moves(&state, 16, &mut legal_moves, false);
    board.rook_moves(16, &mut legal_moves, &state, false);

    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn attack_through_king_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1b1kbnr/pppp1ppp/2n5/4p3/4P2q/5P2/PPPP1KPP/RNBQ1BNR w kq - 3 4");
    board.total_occupancy();
    board.update_full_cache();
    board.calculate_check_restrictions(&mut state, 8);

    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves, false); // should not include e1

    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn pinned_piece_blocking_check_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnb1k1nr/ppppqppp/8/8/1b1P4/2N5/PPP2PPP/R1BQKBNR w KQkq - 1 5");
    board.total_occupancy();
    board.update_full_cache();
    board.calculate_check_restrictions(&mut state, 8);

    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves, false); // should not include e1
    board.knight_moves(8, &mut legal_moves, &state, false); // c3 knight cannot move!

    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn pinned_piece_blocking_check_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("Q1b1kbnr/p1p1pppp/8/1B6/8/8/PP1N1KPP/q5NR b - - 1 1");
    board.total_occupancy();
    board.update_full_cache();
    board.calculate_check_restrictions(&mut state, 16);

    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 16, &mut legal_moves, false);
    board.bishop_moves(16, &mut legal_moves, &state, false); // bishop is pinned!

    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
#[test]
fn check_evasion_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnb2knr/ppp3pp/8/8/2Q5/5N2/PPP3PP/RNBq1BKR w - - 0 1");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(26 | (62 << TO_SHIFT), &mut state, 8, &mut 0, &mut 0);

    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 16, &mut legal_moves, false);
    board.bishop_moves(16, &mut legal_moves, &state, false);
    board.queen_moves(16, &mut legal_moves, &state, false);
    board.rook_moves(16, &mut legal_moves, &state, false);
    board.knight_moves(16, &mut legal_moves, &state, false);

    for i in 0..legal_moves.first_not_occupied {
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
    }
}
