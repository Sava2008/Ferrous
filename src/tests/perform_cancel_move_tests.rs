#[allow(unused)]
use crate::{
    board_geometry_templates::*,
    constants::attacks::*,
    constants::zobrist_hashes::ZOBRIST_HASH_TABLE,
    converters::fen_converter::fen_to_board,
    moves::MoveList,
    {search::Engine, transposition::TranspositionTable},
};
#[test]
fn knight_move_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P2N1/8/2P5/RNBQKBNR w KQkq a6 0 2");

    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.knight_moves(8, &mut legal_moves, &state, false);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..12 {
        board.perform_move(legal_moves.pseudo_moves[i], &mut state, &mut 0, &mut 0);
        board.cancel_move(&mut state, &mut 0, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}

#[test]
fn pawn_moves_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1P1pp2p/2p5/p3N1p1/3P4/8/2P5/RNBQKBNR w KQkq - 0 2");
    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 8, &mut legal_moves, false);
    assert_eq!(legal_moves.first_not_occupied, 11);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..11 {
        let m: u16 = legal_moves.pseudo_moves[i];
        let from = from_square(m) as usize;
        println!("moving piece: {}", board.cached_pieces[from]);
        board.perform_move(m, &mut state, &mut 0, &mut 0);
        assert!(board.bitboards[5].count_ones() == 1);
        board.cancel_move(&mut state, &mut 0, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}

#[test]
fn en_passant_and_cancel_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/3pp2p/2p5/pP2N1p1/3P4/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    state.en_passant_target = Some(40);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 8, &mut legal_moves, false);
    assert_eq!(legal_moves.first_not_occupied, 6);
    let (copied_board, copied_state) = (board.clone(), state.clone());

    //board.perform_move(33851937, &mut state, 8, &mut 0, &mut 0);
    assert_ne!(board.bitboards[6], 1 << 32);
    board.cancel_move(&mut state, &mut 0, &mut 0);
    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}

#[test]
fn en_passant_and_cancel_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r1bqn3/ppppn1r1/6k1/2b1ppPp/2B1P3/2PP1N2/PP3P1P/RN1Q2RK w - f6 0 1");
    board.total_occupancy();
    board.update_full_cache();
    let mut hash_before: u64 = Engine::rebuild_hash(&board, 8);
    board.perform_move(39782, &mut state, &mut 0, &mut hash_before);
    assert_eq!(board.cached_pieces[37], 0);
    assert_eq!(board.bitboards[6] & (1 << 37), 0);
    assert_eq!(
        hash_before,
        Engine::rebuild_hash(&board, 16),
        "missing part: {}",
        hash_before ^ Engine::rebuild_hash(&board, 16)
    );
    board.cancel_move(&mut state, &mut 0, &mut hash_before);
    assert_eq!(hash_before, Engine::rebuild_hash(&board, 8));
}

#[test]
fn castling_short_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/3pp2p/2p5/pP2N1p1/3P4/8/2P5/RNBQK2R w KQkq a6 0 2");
    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves, false);
    assert_eq!(legal_moves.first_not_occupied, 5);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..5 {
        let m: u16 = legal_moves.pseudo_moves[i];
        let mut hash_before: u64 = Engine::rebuild_hash(&board, 8);
        board.perform_move(m, &mut state, &mut 0, &mut hash_before);
        assert_eq!(hash_before, Engine::rebuild_hash(&board, 16));
        board.cancel_move(&mut state, &mut 0, &mut hash_before);
        assert_eq!(hash_before, Engine::rebuild_hash(&board, 8));
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}
#[test]
fn castling_long_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r3k2r/3pp2p/2p5/pP2N1p1/3P4/8/2P5/R3K2R w KQkq a6 0 2");
    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves, false);
    assert_eq!(legal_moves.first_not_occupied, 7);
    let (copied_board, copied_state) = (board.clone(), state.clone());

    for i in 0..7 {
        let mut hash_before: u64 = Engine::rebuild_hash(&board, 8);
        println!("i = {i}");
        let m: u16 = legal_moves.pseudo_moves[i];
        println!(
            "from: {}, to: {}, flag: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );

        board.perform_move(m, &mut state, &mut 0, &mut hash_before);
        let hash_after: u64 = Engine::rebuild_hash(&board, 16);
        assert_eq!(hash_before, hash_after);
        board.cancel_move(&mut state, &mut 0, &mut hash_before);
        let hash_after: u64 = Engine::rebuild_hash(&board, 8);
        assert_eq!(hash_before, hash_after);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };

    board.king_moves(&state, 16, &mut legal_moves, false);
    assert_eq!(legal_moves.first_not_occupied, 4);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..4 {
        println!("i = {i}");
        let m: u16 = legal_moves.pseudo_moves[i];
        println!("from: {}, to: {}", from_square(m), to_square(m));
        board.perform_move(m, &mut state, &mut 0, &mut 0);
        board.cancel_move(&mut state, &mut 0, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}
#[test]
fn knight_moves_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("Q5r1/4kp1p/5p2/4p3/8/N6n/PP3PrP/3R1R1K b - - 0 1");
    board.total_occupancy();
    board.update_full_cache();
    let mut pos_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            pos_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.knight_moves(16, &mut legal_moves, &state, false);
    assert_eq!(legal_moves.first_not_occupied, 4);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..4 {
        let m: u16 = legal_moves.pseudo_moves[i];
        board.perform_move(m, &mut state, &mut 0, &mut 0);
        board.cancel_move(&mut state, &mut 0, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut second_hash: u64 = 0;
    for (i, piece) in board.cached_pieces.iter().enumerate() {
        let piece: u16 = *piece;
        if piece != 0 {
            let zobrist_index: usize = (piece as usize - 1) * 64 + i;
            second_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
        }
    }
    assert_eq!(second_hash, pos_hash);
}

#[test]
fn buggy_castling_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("r3k1nr/pppqp2p/5ppb/1N6/P7/6Q1/1PPP1PPP/R1B1R1K1 b kq - 2 16");
    board.total_occupancy();
    board.update_full_cache();
    let three_zeros: u16 = 60 | (58 << TO_SHIFT) | (1 << MARK_SHIFT);

    let (board_copy, state_copy) = (board.clone(), state.clone());

    let mut hash_before = Engine::rebuild_hash(&board, 16);
    board.perform_move(three_zeros, &mut state, &mut 0, &mut hash_before);
    assert_eq!(hash_before, Engine::rebuild_hash(&board, 8));
    board.cancel_move(&mut state, &mut 0, &mut hash_before);
    assert_eq!(hash_before, Engine::rebuild_hash(&board, 16));

    assert_eq!(board_copy, board);
    assert_eq!(state_copy, state);
}

#[test]
fn pawn_promo_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, state) = fen_to_board("2r5/1P6/K1pp4/8/1R3p1k/8/4P1P1/8 w - - 1 4");
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.total_occupancy();
    board.update_full_cache();

    board.pawn_moves(&state, 8, &mut moves, false);
    assert_eq!(moves.first_not_occupied, 12);

    // promotion to evade a check
    let (mut board, mut state) = fen_to_board("1r6/1P6/K1pp4/8/1R3p1k/8/4P1P1/8 w - - 1 4");
    let mut moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.total_occupancy();
    board.update_full_cache();
    board.perform_move(
        57 | (56 << TO_SHIFT) | (7 << MARK_SHIFT),
        &mut state,
        &mut 0,
        &mut 0,
    );

    board.pawn_moves(&state, 8, &mut moves, false);
    let (board_copy, state_copy) = (board.clone(), state.clone());
    let mut legal_moves: u8 = 0;
    for i in 0..moves.first_not_occupied {
        let m: u16 = moves.pseudo_moves[i];
        println!(
            "from {}, to {}, promotion: {}",
            from_square(m),
            to_square(m),
            (m & MARK_MASK) >> MARK_SHIFT
        );
        let mut hash_before = Engine::rebuild_hash(&board, 8);
        board.perform_move(m, &mut state, &mut 0, &mut hash_before);
        let hash_after = Engine::rebuild_hash(&board, 16);
        assert_eq!(hash_after, hash_before);
        if !board.is_square_attacked(board.white_king_square, 16) {
            legal_moves += 1;
        }
        board.cancel_move(&mut state, &mut 0, &mut hash_before);
        let hash_after = Engine::rebuild_hash(&board, 8);
        assert_eq!(hash_after, hash_before);
        assert_eq!(board, board_copy);
        assert_eq!(state, state_copy);
    }
    assert_eq!(legal_moves, 4);
}

// `rn2kbnr/pppbpppp/6q1/8/3P4/2N2B2/PPP2PPP/R1BQK1NR b KQkq - 2 6` played Bc6??, Nc6 was best
