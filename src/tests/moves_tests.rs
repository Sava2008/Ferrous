#[allow(unused_imports)]
use crate::{
    board_geometry_templates::{FROM_MASK, TO_MASK, TO_SHIFT},
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
    board.update_full_cache();
    board.count_material();
    let legal_moves: Vec<u16> = board.rook_moves(&state, &8);
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
    board.update_full_cache();
    board.count_material();
    let legal_moves: Vec<u16> = board.king_moves(&state, &8);
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
    board.update_full_cache();
    board.count_material();
    state.en_passant_target = Some(40);
    let legal_moves: Vec<u16> = board.pawn_moves(&state, &8);
    assert_eq!(legal_moves.len(), 6);
    for m in legal_moves {
        assert!([18, 26, 35, 36, 40, 41].contains(&((m & TO_MASK) >> TO_SHIFT)));
    }
}
