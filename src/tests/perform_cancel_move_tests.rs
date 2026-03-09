#[allow(unused)]
use crate::{
    board_geometry_templates::*, constants::attacks::*, converters::fen_converter::fen_to_board,
    moves::MoveList,
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
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.knight_moves(8, &mut legal_moves);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..12 {
        board.perform_move(legal_moves.pseudo_moves[i], &mut state, 8, &mut 0);
        board.cancel_move(&mut state, 8, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
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
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 8, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 11);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..11 {
        let m = legal_moves.pseudo_moves[i];
        println!(
            "from {}, to {}, capture {}, promotion: {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT,
            captured_piece(m),
            promotion(m)
        );
        board.perform_move(m, &mut state, 8, &mut 0);
        board.cancel_move(&mut state, 8, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
}

#[test]
fn en_passant_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/3pp2p/2p5/pP2N1p1/3P4/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.update_full_cache();
    state.en_passant_target = Some(40);
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.pawn_moves(&state, 8, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 6);
    let (copied_board, copied_state) = (board.clone(), state.clone());

    board.perform_move(33851937, &mut state, 8, &mut 0);
    assert_ne!(board.black_pawns, 1 << 32);
    board.cancel_move(&mut state, 8, &mut 0);
    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
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
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 5);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..5 {
        let m = legal_moves.pseudo_moves[i];
        println!(
            "from {}, to {}, capture {}, castling: {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT,
            captured_piece(m),
            castling(m)
        );
        board.perform_move(m, &mut state, 8, &mut 0);
        board.cancel_move(&mut state, 8, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
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
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.king_moves(&state, 8, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 7);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..7 {
        let m = legal_moves.pseudo_moves[i];
        println!(
            "from {}, to {}, capture {}, castling: {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT,
            captured_piece(m),
            castling(m)
        );
        board.perform_move(m, &mut state, 8, &mut 0);
        board.cancel_move(&mut state, 8, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };

    board.king_moves(&state, 16, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 5);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..5 {
        let m = legal_moves.pseudo_moves[i];
        println!(
            "from {}, to {}, capture {}, castling: {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT,
            captured_piece(m),
            castling(m)
        );
        board.perform_move(m, &mut state, 16, &mut 0);
        board.cancel_move(&mut state, 16, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
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
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.knight_moves(16, &mut legal_moves);
    assert_eq!(legal_moves.first_not_occupied, 4);
    let (copied_board, copied_state) = (board.clone(), state.clone());
    for i in 0..4 {
        let m = legal_moves.pseudo_moves[i];
        println!(
            "from {}, to {}, capture {}, castling: {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT,
            captured_piece(m),
            castling(m)
        );
        board.perform_move(m, &mut state, 16, &mut 0);
        board.cancel_move(&mut state, 16, &mut 0);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
}
