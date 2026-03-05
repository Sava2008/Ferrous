#[allow(unused_imports)]
use crate::{
    board_geometry_templates::*,
    constants::attacks::{
        compute_all_lines, compute_all_rays, compute_all_rays_from,
        initialize_sliding_attack_tables,
    },
    converters::fen_converter::fen_to_board,
};

#[test]
fn pawn_moves_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/1pppp1pp/8/pP2p3/3P2N1/8/2P5/RNBQKBNR w KQkq a6 0 2");
    board.total_occupancy();
    board.count_material();
    state.en_passant_target = Some(40);

    let all_pawn_moves: Vec<u32> = board.pawn_moves(&state, NO_PIECE_WHITE);
    let (cloned_board, cloned_state) = (board.clone(), state.clone());
    for m in all_pawn_moves {
        board.perform_move(m, &mut state, NO_PIECE_WHITE);
        board.cancel_move(&mut state, NO_PIECE_WHITE);
        assert_eq!(
            board,
            cloned_board,
            "{}, {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT
        );
        assert_eq!(
            state,
            cloned_state,
            "{}, {}",
            m & FROM_MASK,
            (m & TO_MASK) >> TO_SHIFT
        );
    }
}
