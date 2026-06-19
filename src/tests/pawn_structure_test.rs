#[allow(unused)]
use crate::{
    board_geometry_templates::*, constants::attacks::*, converters::fen_converter::fen_to_board,
    moves::MoveList,
};

#[test]
fn isolated_pawns_count_test1() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/pppp1ppp/8/8/4P3/2P5/P4PPP/RNBQKBNR b KQkq - 0 4"); // Danish gambit position
    board.total_occupancy();
    board.update_full_cache();

    board.modify_pawn_structure(&mut state.pawn_structure);

    assert_eq!(
        state.pawn_structure.isolated_white,
        2 * 3,
        "isolated white: {}",
        state.pawn_structure.isolated_white
    );
    assert_eq!(
        state.pawn_structure.isolated_black, 0,
        "isolated_black: {}",
        state.pawn_structure.isolated_black
    );

    assert_eq!(state.pawn_structure.doubled_white, 0);
    assert_eq!(state.pawn_structure.doubled_black, 0);
}

#[test]
fn isolated_pawns_count_test2() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) = fen_to_board("2k5/2rr3p/4Q3/3p2q1/3P4/4P3/4K3/8 w - - 3 47");
    board.total_occupancy();
    board.update_full_cache();

    board.modify_pawn_structure(&mut state.pawn_structure);

    assert_eq!(
        state.pawn_structure.isolated_white, 0,
        "isolated white: {}",
        state.pawn_structure.isolated_white
    );
    assert_eq!(
        state.pawn_structure.isolated_black,
        2 * 3,
        "isolated_black: {}",
        state.pawn_structure.isolated_black
    );

    assert_eq!(state.pawn_structure.doubled_white, 0);
    assert_eq!(state.pawn_structure.doubled_black, 0);
}

#[test]
fn isolated_pawns_count_test3() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();
    let (mut board, mut state) =
        fen_to_board("rnbqkbnr/ppppp1pp/8/5P2/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 2");
    board.total_occupancy();
    board.update_full_cache();

    board.modify_pawn_structure(&mut state.pawn_structure);

    assert_eq!(
        state.pawn_structure.isolated_white,
        1 * 3,
        "isolated white: {}",
        state.pawn_structure.isolated_white
    );
    assert_eq!(
        state.pawn_structure.isolated_black, 0,
        "isolated_black: {}",
        state.pawn_structure.isolated_black
    );

    assert_eq!(state.pawn_structure.doubled_white, 1 * 3);
    assert_eq!(state.pawn_structure.doubled_black, 0);
}
