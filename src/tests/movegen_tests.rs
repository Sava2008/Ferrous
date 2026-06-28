#[allow(unused)]
use crate::{
    board_geometry_templates::*, constants::attacks::*, converters::fen_converter::fen_to_board,
    moves::MoveList,
};

#[test]
fn knight_gen_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1b1kbnr/pppp1ppp/2n5/8/4q3/2N5/PP1N1PPP/R1BQKB1R w KQkq - 0 7");
    board.total_occupancy();
    board.update_full_cache();

    board.calculate_check_restrictions(&mut state, 8);

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.knight_moves(8, &mut moves, &state, false);
    println!("{:?}", moves.pseudo_moves);
}

#[test]
fn bishop_gen_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1b1kb1r/ppppqppp/5n2/8/3pP3/2Pn2Q1/PP1N1PPP/RNB1KB1R w KQkq - 7 8");
    board.total_occupancy();
    board.update_full_cache();

    board.calculate_check_restrictions(&mut state, 8);

    let mut moves = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.bishop_moves(8, &mut moves, &state, false);
    println!("{:?}", moves.pseudo_moves);
}

#[test]
fn pawn_gen_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1pp1pppp/p7/3p4/8/P3P3/1PPP1PPP/RNBQKBNR w KQkq - 0 3");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(5 | (33 << TO_SHIFT), &mut state, &mut 0, &mut 0);

    let mut moves = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    println!("{:?}", moves.pseudo_moves);
}
#[test]
fn pawn_gen_block_check_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(14 | (22 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    println!(
        "white restrictions: {:b}, black restrictions: {:b}",
        state.white_legal_squares_mask, state.black_legal_squares_mask
    );
    board.perform_move(31 | (22 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    println!(
        "white restrictions: {:b}, black restrictions: {:b}",
        state.white_legal_squares_mask, state.black_legal_squares_mask
    );
    board.perform_move(25 | (17 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    println!(
        "white restrictions: {:b}, black restrictions: {:b}",
        state.white_legal_squares_mask, state.black_legal_squares_mask
    );

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    println!("{:?}", moves.pseudo_moves);
}

#[test]
fn illegal_en_passant_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkb1r/pppp1pp1/5n2/4p2p/4PP2/2N2Q2/PPPP2PP/R1B1KBNR b KQkq - 1 4");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(54 | (38 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 8, &mut moves, false);
    println!("pawn moves: {:?}", moves.pseudo_moves);
}
#[test]
fn en_passant_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(8 | (24 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    assert_eq!(state.en_passant_target, Some(16));
}

#[test]
fn phantom_capture_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1bq1k1r/ppn1b1pp/2n1p3/2pPPp2/4Q3/3B1N2/PP1P1PPP/RNB2RK1 w - f6 0 11");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(2851, &mut state, &mut 0, &mut 0);
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    println!("pawn moves: {:?}", moves.pseudo_moves);
}

#[test]
fn legal_moves_after_check_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("8/2p5/3p4/KP2P2r/1R5k/5p2/6P1/8 b - - 0 2");
    board.total_occupancy();
    board.update_full_cache();

    board.perform_move(31 | (38 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    board.perform_move(36 | (43 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    board.rook_moves(16, &mut moves, &state, false);
    board.king_moves(&state, 16, &mut moves, false);
    for i in 0..moves.first_not_occupied {
        let m: u16 = moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, mark: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
    }
}

#[test]
fn forcing_only_movegen_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkb1r/ppp2ppp/8/3pP3/4n3/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 1 5");
    board.total_occupancy();
    board.update_full_cache();
    state.calculate_check_squares(board.white_king_square as usize, board.total_occupancy, 8);

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, true);
    board.knight_moves(16, &mut moves, &state, true);
    board.bishop_moves(16, &mut moves, &state, true);
    board.queen_moves(16, &mut moves, &state, true);
    board.rook_moves(16, &mut moves, &state, true);
    board.king_moves(&state, 16, &mut moves, true);
    for i in 0..moves.first_not_occupied {
        let m = moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, mark: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
    }

    assert_eq!(moves.first_not_occupied, 3);
}

#[test]
fn forcing_only_movegen_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnb1kb1r/pp2pppp/2pp1n2/q3P3/3P1P2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 5");
    board.total_occupancy();
    board.update_full_cache();
    state.calculate_check_squares(board.white_king_square as usize, board.total_occupancy, 8);

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, true);
    board.knight_moves(16, &mut moves, &state, true);
    board.bishop_moves(16, &mut moves, &state, true);
    board.queen_moves(16, &mut moves, &state, true);
    board.rook_moves(16, &mut moves, &state, true);
    board.king_moves(&state, 16, &mut moves, true);
    for i in 0..moves.first_not_occupied {
        let m: u16 = moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, mark: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
    }

    assert_eq!(moves.first_not_occupied, 5);
}

#[test]
fn king_movegen_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("4k3/6P1/8/p1Q5/8/7P/8/2K5 w - - 2 57");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(
        54 | (62 << TO_SHIFT) | (13 << MARK_SHIFT),
        &mut state,
        &mut 0,
        &mut 0,
    );

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    board.knight_moves(16, &mut moves, &state, false);
    board.bishop_moves(16, &mut moves, &state, false);
    board.queen_moves(16, &mut moves, &state, false);
    board.rook_moves(16, &mut moves, &state, false);
    board.king_moves(&state, 16, &mut moves, false);
    for i in 0..moves.first_not_occupied {
        let m: u16 = moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, mark: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
    }

    assert_eq!(moves.first_not_occupied, 1);
}
#[test]
fn check_movegen_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1pp1pppp/3p4/p7/8/P1P5/1P1PPPPP/RNBQKBNR b KQkq - 0 1");
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(43 | (35 << TO_SHIFT), &mut state, &mut 0, &mut 0);
    board.perform_move(
        3 | (24 << TO_SHIFT) | (7 << MARK_SHIFT),
        &mut state,
        &mut 0,
        &mut 0,
    );

    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 16, &mut moves, false);
    board.knight_moves(16, &mut moves, &state, false);
    board.bishop_moves(16, &mut moves, &state, false);
    board.queen_moves(16, &mut moves, &state, false);
    board.rook_moves(16, &mut moves, &state, false);
    board.king_moves(&state, 16, &mut moves, false);
    for i in 0..moves.first_not_occupied {
        let m: u16 = moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, mark: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
    }

    assert_eq!(moves.first_not_occupied, 6, "{:?}", state.check_squares);
}
