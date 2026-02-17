#[allow(unused_imports)]
use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    enums::PieceColor,
    gamestate::{CastlingRights, GameState, PieceMove},
};
fn create_empty_board() -> Board {
    Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    }
}

#[test]
fn cancelation_test_enpassant_update() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state); // en passant update move
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test_check() -> () {
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0b1000,
        white_queens: 0,
        white_rooks: 0b10000000000,
        white_king: 0b10000000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0b100000000000000000,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 10, to: 26 }, &mut state); // check
    board.cancel_move(&mut state);
    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 10, to: 18 }, &mut state); // double check
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test_castling() -> () {
    let mut board: Board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0b10000001,
        white_king: 0b10000,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
        black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
    };
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 4, to: 6 }, &mut state); // white two zeros
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 4, to: 2 }, &mut state); // white three zeros
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 60, to: 62 }, &mut state); // black two zeros
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);

    board.perform_move(&PieceMove { from: 60, to: 58 }, &mut state); // black three zeros
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test_enpassant() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);
    board.perform_move(&PieceMove { from: 62, to: 47 }, &mut state);
    board.perform_move(&PieceMove { from: 28, to: 36 }, &mut state);
    board.perform_move(&PieceMove { from: 51, to: 35 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 36, to: 43 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test_pin1() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);
    board.perform_move(&PieceMove { from: 51, to: 35 }, &mut state);
    board.perform_move(&PieceMove { from: 6, to: 21 }, &mut state);
    board.perform_move(&PieceMove { from: 57, to: 42 }, &mut state);

    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);

    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 5, to: 33 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn cancelation_test_pin2() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);
    board.perform_move(&PieceMove { from: 51, to: 35 }, &mut state);
    board.perform_move(&PieceMove { from: 6, to: 21 }, &mut state);
    board.perform_move(&PieceMove { from: 57, to: 42 }, &mut state);

    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);

    board.perform_move(&PieceMove { from: 5, to: 33 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    let (copied_board, copied_state): (Board, GameState) = (board.clone(), state.clone());
    board.perform_move(&PieceMove { from: 59, to: 51 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, copied_board);
    assert_eq!(state, copied_state);
}

#[test]
fn double_cancelation_test() -> () {
    let mut board: Board = Board::set();
    let mut state: GameState = GameState::new(&board);

    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::White);
    state.pin_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);

    let (board_copy1, state_copy1): (Board, GameState) = (board.clone(), state.clone());
    board.perform_move(&PieceMove { from: 9, to: 17 }, &mut state); // b3
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);

    let (board_copy2, state_copy2): (Board, GameState) = (board.clone(), state.clone());

    board.perform_move(&PieceMove { from: 52, to: 36 }, &mut state);
    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board.cancel_move(&mut state);

    assert_eq!(board, board_copy2);
    assert_eq!(state, state_copy2);

    board.cancel_move(&mut state);
    assert_eq!(board, board_copy1);
    assert_eq!(state, state_copy1);
}

#[test]
fn all_legal_moves_cancelation_test1() -> () {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut board2: Board = Board::set();
    board2.total_occupancy();
    let mut state: GameState = GameState::new(&board);
    let legal_moves: Vec<PieceMove> =
        Engine::generate_legal_moves(&PieceColor::White, &board, &state);
    for m in legal_moves {
        board.perform_move(&m, &mut state);
        board.cancel_move(&mut state);
        assert_eq!(board, board2);
    }
    board.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);

    board.total_occupancy();
    state.check_info.update(&board, &PieceColor::Black);
    state.pin_info.update(&board, &PieceColor::Black);
    state.update_check_constraints(&board);
    board2.perform_move(&PieceMove { from: 12, to: 28 }, &mut state);

    board2.total_occupancy();
    state.check_info.update(&board2, &PieceColor::Black);
    state.pin_info.update(&board2, &PieceColor::Black);
    state.update_check_constraints(&board2);

    let legal_moves: Vec<PieceMove> =
        Engine::generate_legal_moves(&PieceColor::Black, &board, &state);
    for m in legal_moves {
        board.perform_move(&m, &mut state);
        board.cancel_move(&mut state);
        assert_eq!(board, board2);
    }
}

fn capture_state(board: &Board, state: &GameState) -> (Board, GameState) {
    (board.clone(), state.clone())
}

fn verify_cancel_restores(
    original_board: &Board,
    original_state: &GameState,
    board: &mut Board,
    state: &mut GameState,
    move_made: &PieceMove,
) {
    board.cancel_move(state);

    assert_eq!(
        board, original_board,
        "Board not restored after canceling move {:?}",
        move_made
    );
    assert_eq!(
        state, original_state,
        "GameState not restored after canceling move {:?}",
        move_made
    );
}
#[test]
fn test_simple_pawn_move_and_cancel() {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 12, to: 28 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_pawns & (1 << 12), 0, "Pawn still at e2");
    assert_ne!(board.white_pawns & (1 << 28), 0, "Pawn not at e4");
    assert_eq!(
        state.whose_turn,
        PieceColor::White,
        "Turn should still be white (not updated in perform_move)"
    );

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_simple_knight_move_and_cancel() {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 6, to: 21 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_knights & (1 << 6), 0, "Knight still at g1");
    assert_ne!(board.white_knights & (1 << 21), 0, "Knight not at f3");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}
#[test]
fn test_simple_capture_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_knights = 1 << 28;
    board.black_pawns = 1 << 35;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 28, to: 35 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_knights & (1 << 28), 0, "Knight still at e4");
    assert_ne!(board.white_knights & (1 << 35), 0, "Knight not at d5");
    assert_eq!(board.black_pawns & (1 << 35), 0, "Black pawn not removed");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_capture_updates_castling_rights() {
    let mut board: Board = create_empty_board();
    board.white_rooks = 1 << 7;
    board.black_queens = 1 << 63;

    board.white_king = 1 << 4;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights = CastlingRights {
        white_three_zeros: false,
        white_two_zeros: true,
        black_three_zeros: false,
        black_two_zeros: false,
    };

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 63, to: 7 };
    board.perform_move(&move_piece, &mut state);

    assert!(
        !state.castling_rights.white_two_zeros,
        "Castling right should be removed"
    );

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_white_kingside_castle_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.white_rooks = 1 << 7;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights.white_two_zeros = true;

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 4, to: 6 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_king & (1 << 4), 0, "King still at e1");
    assert_ne!(board.white_king & (1 << 6), 0, "King not at g1");
    assert_eq!(board.white_rooks & (1 << 7), 0, "Rook still at h1");
    assert_ne!(board.white_rooks & (1 << 5), 0, "Rook not at f1");

    assert!(
        !state.castling_rights.white_two_zeros,
        "Kingside castling right should be revoked"
    );
    assert!(
        !state.castling_rights.white_three_zeros,
        "Queenside castling right should be revoked"
    );

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_white_queenside_castle_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.white_rooks = 1 << 0;
    board.white_occupancy();
    board.black_occupancy();
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights.white_three_zeros = true;

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 4, to: 2 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_king & (1 << 4), 0, "King still at e1");
    assert_ne!(board.white_king & (1 << 2), 0, "King not at c1");
    assert_eq!(board.white_rooks & (1 << 0), 0, "Rook still at a1");
    assert_ne!(board.white_rooks & (1 << 3), 0, "Rook not at d1");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_black_kingside_castle_and_cancel() {
    let mut board: Board = create_empty_board();
    board.black_king = 1 << 60;
    board.black_rooks = 1 << 63;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights.black_two_zeros = true;

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 60, to: 62 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.black_king & (1 << 60), 0, "King still at e8");
    assert_ne!(board.black_king & (1 << 62), 0, "King not at g8");
    assert_eq!(board.black_rooks & (1 << 63), 0, "Rook still at h8");
    assert_ne!(board.black_rooks & (1 << 61), 0, "Rook not at f8");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_black_queenside_castle_and_cancel() {
    let mut board: Board = create_empty_board();

    board.black_king = 1 << 60;
    board.black_rooks = 1 << 56;
    board.white_occupancy();
    board.black_occupancy();
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights.black_three_zeros = true;

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 60, to: 58 }; // e8 to c8
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.black_king & (1 << 60), 0, "King still at e8");
    assert_ne!(board.black_king & (1 << 58), 0, "King not at c8");
    assert_eq!(board.black_rooks & (1 << 56), 0, "Rook still at a8");
    assert_ne!(board.black_rooks & (1 << 59), 0, "Rook not at d8");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_white_en_passant_capture_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_pawns = 1 << 32;
    board.black_pawns = 1 << 33;

    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.en_passant_target = Some(41);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 32, to: 41 };
    board.perform_move(&move_piece, &mut state);
    board.total_occupancy();

    assert_eq!(board.white_pawns & (1 << 32), 0, "White pawn still at a5");
    assert_ne!(board.white_pawns & (1 << 41), 0, "White pawn not at a6");
    assert_eq!(
        board.black_pawns & (1 << 33),
        0,
        "Black pawn not removed from b5"
    );
    assert_eq!(
        state.en_passant_target, None,
        "En passant target should be cleared"
    );
    board.cancel_move(&mut state);

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_black_en_passant_capture_and_cancel() {
    let mut board: Board = create_empty_board();

    // Set up en passant: black pawn on a4 (24), white pawn on b4 (15) that just moved two squares
    board.black_pawns = 1 << 24; // a4
    board.white_pawns = 1 << 25; // b4

    board.total_occupancy();

    let mut state = GameState::new(&board);
    state.en_passant_target = Some(17); // a3 is en passant target
    state.whose_turn = PieceColor::Black;

    let (original_board, original_state) = capture_state(&board, &state);

    // Black captures en passant on a3
    let move_piece: PieceMove = PieceMove { from: 24, to: 16 }; // a4 to a3 (en passant capture)
    board.perform_move(&move_piece, &mut state);

    // Verify en passant capture
    assert_eq!(board.black_pawns & (1 << 24), 0, "Black pawn still at a4");
    assert_ne!(board.black_pawns & (1 << 16), 0, "Black pawn not at a3");
    assert_eq!(
        board.white_pawns & (1 << 15),
        0,
        "White pawn not removed from b4"
    );
    assert_eq!(
        state.en_passant_target, None,
        "En passant target should be cleared"
    );

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_double_pawn_push_sets_en_passant() {
    let mut board: Board = create_empty_board();

    // White pawn on a2 (8)
    board.white_pawns = 1 << 8;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.whose_turn = PieceColor::White;

    let (original_board, original_state) = capture_state(&board, &state);

    // Double pawn push from a2 to a4 (24)
    let move_piece: PieceMove = PieceMove { from: 8, to: 24 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(
        state.en_passant_target,
        Some(16),
        "En passant target not set correctly"
    );

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_pinned_piece_move_and_cancel() {
    let mut board = create_empty_board();

    // Set up a pinned white bishop on d4 (27)
    board.white_bishops = 1 << 27;
    board.white_king = 1 << 4; // e1 (4)
    board.black_queens = 1 << 50; // e6 (50) - pinning through diagonal
    board.white_occupancy();
    board.black_occupancy();
    board.total_occupancy();

    let mut state = GameState::new(&board);
    state.whose_turn = PieceColor::White;
    state.pin_info.update(&board, &PieceColor::White);

    let (original_board, original_state) = capture_state(&board, &state);

    // Move pinned bishop along the pin ray (should be allowed)
    let move_piece = PieceMove { from: 27, to: 18 }; // d4 to c3 (along diagonal toward king)
    board.perform_move(&move_piece, &mut state);

    // Verify move was made
    assert_eq!(board.white_bishops & (1 << 27), 0, "Bishop still at d4");
    assert_ne!(board.white_bishops & (1 << 18), 0, "Bishop not at c3");

    // Cancel and verify restoration
    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_capture_of_pinning_piece_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_knights = 1 << 27;
    board.white_king = 1 << 4;
    board.black_queens = 1 << 49;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.whose_turn = PieceColor::White;
    state.pin_info.update(&board, &PieceColor::White);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece = PieceMove { from: 27, to: 49 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_knights & (1 << 27), 0, "Knight still at d4");
    assert_ne!(board.white_knights & (1 << 49), 0, "Knight not at e5");
    assert_eq!(board.black_queens & (1 << 49), 0, "Queen not removed");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_move_out_of_check_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.black_rooks = 1 << 28;
    board.white_knights = 1 << 18;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.whose_turn = PieceColor::White;
    state.check_info.update(&board, &PieceColor::White);
    state.update_check_constraints(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 18, to: 20 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_knights & (1 << 18), 0, "Knight still at c3");
    assert_ne!(board.white_knights & (1 << 20), 0, "Knight not at e3");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_king_move_out_of_check_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.black_knights = 1 << 21;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.whose_turn = PieceColor::White;
    state.check_info.update(&board, &PieceColor::White);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 4, to: 11 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.white_king & (1 << 4), 0, "King still at e1");
    assert_ne!(board.white_king & (1 << 11), 0, "King not at d2");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_double_check_only_king_moves() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.black_knights = 1 << 21;
    board.black_bishops = 1 << 11;
    board.white_rooks = 1 << 0;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.whose_turn = PieceColor::White;
    state.check_info.update(&board, &PieceColor::White);

    let (original_board, original_state) = capture_state(&board, &state);

    let illegal_move: PieceMove = PieceMove { from: 0, to: 1 };
    board.perform_move(&illegal_move, &mut state);

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &illegal_move,
    );
}

#[test]
fn test_castling_with_attacked_squares_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_king = 1 << 4;
    board.white_rooks = (1 << 0) | (1 << 7);
    board.black_queens = 1 << 13;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);
    state.castling_rights.white_two_zeros = true;
    state.castling_rights.white_three_zeros = true;
    state.whose_turn = PieceColor::White;

    let (original_board, original_state) = capture_state(&board, &state);

    let castle_move: PieceMove = PieceMove { from: 4, to: 6 };
    board.perform_move(&castle_move, &mut state);

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &castle_move,
    );
}

#[test]
fn test_capture_that_removes_last_pawn_and_cancel() {
    let mut board: Board = create_empty_board();

    board.white_knights = 1 << 28;
    board.black_pawns = 1 << 35;
    board.total_occupancy();

    let mut state: GameState = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 28, to: 35 };
    board.perform_move(&move_piece, &mut state);

    assert_eq!(board.black_pawns, 0, "Black pawns should be zero");

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}

#[test]
fn test_sequence_of_moves_with_cancel() {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let moves: Vec<PieceMove> = vec![
        PieceMove { from: 12, to: 28 },
        PieceMove { from: 52, to: 36 },
        PieceMove { from: 6, to: 21 },
    ];
    let mut intermediate_board: Board = board.clone();
    let mut intermediate_state: GameState = state.clone();

    for move_piece in &moves {
        board.perform_move(move_piece, &mut state);
    }

    for _ in moves.iter().rev() {
        board.cancel_move(&mut state);

        assert_ne!(
            &board, &intermediate_board,
            "Board should change after cancel"
        );
        assert_ne!(
            &state, &intermediate_state,
            "State should change after cancel"
        );

        intermediate_board = board.clone();
        intermediate_state = state.clone();
    }

    assert_eq!(
        &board, &original_board,
        "Board not restored after canceling all moves"
    );
    assert_eq!(
        &state, &original_state,
        "State not restored after canceling all moves"
    );
}

#[test]
fn test_cancel_on_empty_history() {
    let mut board = Board::set();
    let mut state = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    board.cancel_move(&mut state);

    assert_eq!(
        &board, &original_board,
        "Board changed after cancel on empty history"
    );
    assert_eq!(
        &state, &original_state,
        "State changed after cancel on empty history"
    );
}

#[test]
fn test_cancel_after_illegal_move() {
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let illegal_move: PieceMove = PieceMove { from: 52, to: 36 };
    board.perform_move(&illegal_move, &mut state);

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &illegal_move,
    );
}

#[test]
fn test_cancel_after_promotion_without_promotion_handling() {
    let mut board: Board = create_empty_board();

    board.white_pawns = 1 << 48;
    board.total_occupancy();

    let mut state = GameState::new(&board);

    let (original_board, original_state) = capture_state(&board, &state);

    let move_piece: PieceMove = PieceMove { from: 48, to: 56 };
    board.perform_move(&move_piece, &mut state);

    verify_cancel_restores(
        &original_board,
        &original_state,
        &mut board,
        &mut state,
        &move_piece,
    );
}
