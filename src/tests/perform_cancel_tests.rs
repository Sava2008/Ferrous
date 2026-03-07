#[allow(unused_imports)]
use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::attacks::{
        compute_all_lines, compute_all_rays, compute_all_rays_from,
        initialize_sliding_attack_tables,
    },
    converters::fen_converter::fen_to_board,
    gamestate::GameState,
};

#[test]
fn pawn_moves_and_cancel_test() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, mut state): (Board, GameState) =
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

#[test]
fn promotion_black_and_cancel() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, mut state) = fen_to_board("8/4N2k/p5RP/8/8/P1P5/4pKP1/8 b - - 1 44");
    let (copied_board, copied_state) = (board.clone(), state.clone());
    let legal_pawn_moves: Vec<u32> = board.pawn_moves(&state, 16);
    for m in legal_pawn_moves {
        board.perform_move(m, &mut state, 16);
        board.cancel_move(&mut state, 16);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
}

#[test]
fn castling_and_cancel() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    let (mut board, mut state): (Board, GameState) =
        fen_to_board("rnbqk2r/ppp1ppbp/5np1/3p2B1/3P4/2NQ4/PPP1PPPP/R3KBNR w KQkq - 2 5");
    let (copied_board, copied_state) = (board.clone(), state.clone());
    let legal_king_moves: Vec<u32> = board.king_moves(&state, 8);
    assert_eq!(legal_king_moves.len(), 3);
    for m in legal_king_moves {
        board.perform_move(m, &mut state, 8);
        assert_ne!(board.white_king_square, 4);
        board.cancel_move(&mut state, 8);
        assert_eq!(board, copied_board);
        assert_eq!(state, copied_state);
    }
}
