#[allow(unused_imports)]
use crate::{
    board_geometry_templates::{FROM_MASK, TO_MASK, TO_SHIFT, captured_piece, moving_piece},
    constants::attacks::{
        compute_all_lines, compute_all_rays, compute_all_rays_from,
        initialize_sliding_attack_tables,
    },
    converters::fen_converter::fen_to_board,
};

#[test]
fn rook_moves_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, state) = fen_to_board("r4rk1/pp6/2p2np1/3p1B2/8/2P1P2P/Pq3P2/R3K3 w Q - 0 23");
    board.total_occupancy();
    board.count_material();
    let legal_moves: Vec<u32> = board.rook_moves(&state, 8);
    assert_ne!(legal_moves.len(), 0);
    for m in legal_moves {
        assert_eq!(m & FROM_MASK, 0);
        assert!((1..=3).contains(&((m & TO_MASK) >> TO_SHIFT)));
    }
}

#[test]
fn king_moves_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, state) =
        fen_to_board("rn1qkb1r/pp3ppp/2p2n2/3p4/3Q4/2NBP2b/PPP2PPP/R1B1K2R w KQkq - 0 8");
    board.total_occupancy();
    board.count_material();
    let legal_moves: Vec<u32> = board.king_moves(&state, 8);
    assert_eq!(legal_moves.len(), 5);
    for m in legal_moves {
        assert_eq!(m & FROM_MASK, 4);
        assert!([6, 5, 3, 11, 12].contains(&((m & TO_MASK) >> TO_SHIFT)));
    }
}

#[test]
fn pawn_moves_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P4/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.count_material();
    state.en_passant_target = Some(40);
    let legal_moves: Vec<u32> = board.pawn_moves(&state, 8);
    assert_eq!(legal_moves.len(), 6);
    for m in legal_moves {
        assert!([18, 26, 35, 36, 40, 41].contains(&((m & TO_MASK) >> TO_SHIFT)));
    }
}

#[test]
fn knight_move_tests() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P2N1/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.count_material();
    let legal_moves: Vec<u32> = board.knight_moves(&state, 8);
    assert_eq!(legal_moves.len(), 12);
    let expected_values: Vec<(u32, u32, u32, u32)> = vec![
        (1, 11, 0, 10),
        (1, 16, 0, 10),
        (1, 18, 0, 10),
        (6, 12, 0, 10),
        (6, 21, 0, 10),
        (6, 23, 0, 10),
        (30, 13, 0, 10),
        (30, 15, 0, 10),
        (30, 20, 0, 10),
        (30, 36, 17, 10),
        (30, 45, 0, 10),
        (30, 47, 0, 10),
    ];
    for (m, v) in legal_moves.iter().zip(expected_values) {
        assert_eq!(m & FROM_MASK, v.0);
        assert_eq!((m & TO_MASK) >> TO_SHIFT, v.1);
        assert_eq!(captured_piece(*m), v.2);
        assert_eq!(moving_piece(*m), v.3);
    }
}

#[test]
fn bishop_moves_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P2N1/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.count_material();
    let legal_moves: Vec<u32> = board.bishop_moves(&state, 8);
    assert_eq!(legal_moves.len(), 12);
    let expected_values: Vec<(u32, u32, u32, u32)> = vec![
        (2, 9, 0, 11),
        (2, 11, 0, 11),
        (2, 16, 0, 11),
        (2, 20, 0, 11),
        (2, 29, 0, 11),
        (2, 38, 0, 11),
        (2, 47, 0, 11),
        (5, 12, 0, 11),
        (5, 14, 0, 11),
        (5, 19, 0, 11),
        (5, 23, 0, 11),
        (5, 26, 0, 11),
    ];
    for (m, v) in legal_moves.iter().zip(expected_values) {
        assert_eq!(m & FROM_MASK, v.0);
        assert_eq!((m & TO_MASK) >> TO_SHIFT, v.1);
        assert_eq!(captured_piece(*m), v.2);
        assert_eq!(moving_piece(*m), v.3);
    }
}

#[test]
fn queen_moves_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P2N1/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.count_material();
    let legal_moves: Vec<u32> = board.queen_moves(&state, 8);
    assert_eq!(legal_moves.len(), 4);
    let expected_values: Vec<(u32, u32, u32, u32)> = vec![
        (3, 11, 0, 13),
        (3, 12, 0, 13),
        (3, 19, 0, 13),
        (3, 21, 0, 13),
    ];
    for (m, v) in legal_moves.iter().zip(expected_values) {
        assert_eq!(m & FROM_MASK, v.0);
        assert_eq!((m & TO_MASK) >> TO_SHIFT, v.1);
        assert_eq!(captured_piece(*m), v.2);
        assert_eq!(moving_piece(*m), v.3);
    }
}
