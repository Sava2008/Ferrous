use crate::alpha_beta_pruning::Engine;
#[allow(unused_imports)]
use crate::{
    board::Board,
    enums::PieceColor,
    gamestate::{GameState, PieceMove},
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

// Helper to convert move vectors to a set of strings for easy comparison
fn moves_to_strings(moves: &[PieceMove]) -> Vec<String> {
    let mut strings: Vec<String> = moves
        .iter()
        .map(|m| format!("{}-{}", m.from, m.to))
        .collect();
    strings.sort();
    strings
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

#[cfg(test)]
mod move_execution_tests {
    use super::*;
    use crate::enums::{GameResult, PieceColor, PieceType};
    use crate::gamestate::{CastlingRights, CheckInfo, GameState, PieceMove, PinInfo};

    // Helper function to create a copy of board and state for comparison
    fn capture_state(board: &Board, state: &GameState) -> (Board, GameState) {
        (board.clone(), state.clone())
    }

    // Helper to verify that cancel_move restores original state
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

    mod basic_moves_tests {
        use super::*;

        #[test]
        fn test_simple_pawn_move_and_cancel() {
            let mut board = Board::set(); // Start from initial position
            board.total_occupancy();
            let mut state = GameState::new(&board);

            // Capture original state
            let (original_board, original_state) = capture_state(&board, &state);

            // Move white pawn from e2 to e4
            let move_piece = PieceMove { from: 12, to: 28 }; // e2(12) to e4(28)
            board.perform_move(&move_piece, &mut state);

            // Verify move was made
            assert_eq!(board.white_pawns & (1 << 12), 0, "Pawn still at e2");
            assert_ne!(board.white_pawns & (1 << 28), 0, "Pawn not at e4");
            assert_eq!(
                state.whose_turn,
                PieceColor::White,
                "Turn should still be white (not updated in perform_move)"
            );

            // Cancel move and verify restoration
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
    }

    mod capture_tests {
        use super::*;

        #[test]
        fn test_simple_capture_and_cancel() {
            let mut board = create_empty_board();

            // Place white knight on e4 (28) and black pawn on d5 (35)
            board.white_knights = 1 << 28;
            board.black_pawns = 1 << 35;
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            let (original_board, original_state) = capture_state(&board, &state);

            // White knight captures on d5
            let move_piece = PieceMove { from: 28, to: 35 };
            board.perform_move(&move_piece, &mut state);

            // Verify capture
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

            // Set up position where capturing a rook affects castling rights
            board.white_rooks = 1 << 7; // white rook on h1
            board.black_queens = 1 << 63; // black queen on h8

            board.white_king = 1 << 4; // white king on e1
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

            // Verify castling right was removed
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
    }

    mod castling_tests {
        use super::*;

        #[test]
        fn test_white_kingside_castle_and_cancel() {
            let mut board = create_empty_board();

            // Set up white kingside castling
            board.white_king = 1 << 4; // e1
            board.white_rooks = 1 << 7; // h1
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.castling_rights.white_two_zeros = true;

            let (original_board, original_state) = capture_state(&board, &state);

            // Castle kingside
            let move_piece = PieceMove { from: 4, to: 6 }; // e1 to g1
            board.perform_move(&move_piece, &mut state);

            // Verify castling
            assert_eq!(board.white_king & (1 << 4), 0, "King still at e1");
            assert_ne!(board.white_king & (1 << 6), 0, "King not at g1");
            assert_eq!(board.white_rooks & (1 << 7), 0, "Rook still at h1");
            assert_ne!(board.white_rooks & (1 << 5), 0, "Rook not at f1");

            // Verify castling rights are revoked
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
            let mut board = create_empty_board();

            // Set up white queenside castling
            board.white_king = 1 << 4; // e1
            board.white_rooks = 1 << 0; // a1
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.castling_rights.white_three_zeros = true;

            let (original_board, original_state) = capture_state(&board, &state);

            // Castle queenside
            let move_piece = PieceMove { from: 4, to: 2 }; // e1 to c1
            board.perform_move(&move_piece, &mut state);

            // Verify castling
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
            let mut board = create_empty_board();

            // Set up black kingside castling
            board.black_king = 1 << 60; // e8
            board.black_rooks = 1 << 63; // h8
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.castling_rights.black_two_zeros = true;

            let (original_board, original_state) = capture_state(&board, &state);

            // Castle kingside
            let move_piece = PieceMove { from: 60, to: 62 }; // e8 to g8
            board.perform_move(&move_piece, &mut state);

            // Verify castling
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
            let mut board = create_empty_board();

            // Set up black queenside castling
            board.black_king = 1 << 60; // e8
            board.black_rooks = 1 << 56; // a8
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.castling_rights.black_three_zeros = true;

            let (original_board, original_state) = capture_state(&board, &state);

            // Castle queenside
            let move_piece = PieceMove { from: 60, to: 58 }; // e8 to c8
            board.perform_move(&move_piece, &mut state);

            // Verify castling
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
    }

    mod en_passant_tests {
        use super::*;

        #[test]
        fn test_white_en_passant_capture_and_cancel() {
            let mut board = create_empty_board();

            // Set up en passant: white pawn on a5 (32), black pawn on b5 (41) that just moved two squares
            board.white_pawns = 1 << 32; // a5
            board.black_pawns = 1 << 33; // b5

            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.en_passant_target = Some(41);

            let (original_board, original_state) = capture_state(&board, &state);

            // White captures en passant on a6
            let move_piece: PieceMove = PieceMove { from: 32, to: 41 }; // a5 to a6 (en passant capture)
            board.perform_move(&move_piece, &mut state);
            board.total_occupancy();

            // Verify en passant capture
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
    }

    mod pin_scenarios_tests {
        use super::*;

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
            let mut board = create_empty_board();

            // Set up a pinned piece that captures the pinning piece
            board.white_knights = 1 << 27; // knight on d4
            board.white_king = 1 << 4; // e1
            board.black_queens = 1 << 49; // e5 (49) - pinning
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.pin_info.update(&board, &PieceColor::White);

            let (original_board, original_state) = capture_state(&board, &state);

            // Knight captures the pinning queen (should resolve pin)
            let move_piece = PieceMove { from: 27, to: 49 };
            board.perform_move(&move_piece, &mut state);

            // Verify capture
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
    }

    mod check_scenarios_tests {
        use super::*;

        #[test]
        fn test_move_out_of_check_and_cancel() {
            let mut board = create_empty_board();

            // Set up white king in check from black rook
            board.white_king = 1 << 4; // e1
            board.black_rooks = 1 << 28; // e4 - giving check
            board.white_knights = 1 << 18; // knight on c3
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.check_info.update(&board, &PieceColor::White);
            state.update_check_constraints(&board);

            let (original_board, original_state) = capture_state(&board, &state);

            // Knight moves to block the check (e3=20)
            let move_piece = PieceMove { from: 18, to: 20 }; // c3 to e3
            board.perform_move(&move_piece, &mut state);

            // Verify move was made
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
            let mut board = create_empty_board();

            // Set up white king in check from black knight
            board.white_king = 1 << 4; // e1
            board.black_knights = 1 << 21; // f3 - giving check
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.check_info.update(&board, &PieceColor::White);

            let (original_board, original_state) = capture_state(&board, &state);

            // King moves out of check to d2 (11)
            let move_piece = PieceMove { from: 4, to: 11 };
            board.perform_move(&move_piece, &mut state);

            // Verify move was made
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
            let mut board = create_empty_board();

            // Set up double check on white king
            board.white_king = 1 << 4; // e1
            board.black_knights = 1 << 21; // f3
            board.black_bishops = 1 << 11; // d2 - double check
            board.white_rooks = 1 << 0; // rook on a1
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.check_info.update(&board, &PieceColor::White);

            let (original_board, original_state) = capture_state(&board, &state);

            // Try to move rook (should be illegal in double check, but perform_move doesn't check legality)
            let illegal_move = PieceMove { from: 0, to: 1 }; // a1 to b1
            board.perform_move(&illegal_move, &mut state);

            // Cancel should still restore correctly
            verify_cancel_restores(
                &original_board,
                &original_state,
                &mut board,
                &mut state,
                &illegal_move,
            );
        }
    }

    mod complex_scenarios_tests {
        use super::*;

        #[test]
        fn test_castling_with_attacked_squares_and_cancel() {
            let mut board = create_empty_board();

            // Set up castling with attacked squares
            board.white_king = 1 << 4; // e1
            board.white_rooks = (1 << 0) | (1 << 7); // a1 and h1
            board.black_queens = 1 << 13; // f2 - attacks f1
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);
            state.castling_rights.white_two_zeros = true;
            state.castling_rights.white_three_zeros = true;
            state.whose_turn = PieceColor::White;

            let (original_board, original_state) = capture_state(&board, &state);

            // Try to castle kingside (should be illegal due to attacked f1, but perform_move doesn't check)
            let castle_move = PieceMove { from: 4, to: 6 };
            board.perform_move(&castle_move, &mut state);

            // Cancel should restore correctly
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
            let mut board = create_empty_board();

            // Set up position where capture removes last pawn
            board.white_knights = 1 << 28; // e4
            board.black_pawns = 1 << 35; // e5 - last black pawn
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);

            let (original_board, original_state) = capture_state(&board, &state);

            // Knight captures last pawn
            let move_piece = PieceMove { from: 28, to: 35 };
            board.perform_move(&move_piece, &mut state);

            // Verify pawn is gone
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

            // Make a sequence of moves
            let moves: Vec<PieceMove> = vec![
                PieceMove { from: 12, to: 28 }, // e2-e4
                PieceMove { from: 52, to: 36 }, // e7-e5
                PieceMove { from: 6, to: 21 },  // g1-f3
            ];
            let mut intermediate_board: Board = board.clone();
            let mut intermediate_state: GameState = state.clone();

            // Perform all moves
            for move_piece in &moves {
                board.perform_move(move_piece, &mut state);
            }

            for move_piece in moves.iter().rev() {
                board.cancel_move(&mut state);

                // After cancel, we should have a different state than before cancel
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

            // After canceling all moves, we should be back to original
            assert_eq!(
                &board, &original_board,
                "Board not restored after canceling all moves"
            );
            assert_eq!(
                &state, &original_state,
                "State not restored after canceling all moves"
            );
        }
    }

    mod cancellation_edge_cases_tests {
        use super::*;

        #[test]
        fn test_cancel_on_empty_history() {
            let mut board = Board::set();
            let mut state = GameState::new(&board);

            let (original_board, original_state) = capture_state(&board, &state);

            // Cancel with no moves in history
            board.cancel_move(&mut state);

            // State should remain unchanged
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

            // Try to make an illegal move (moving opponent's piece)
            let illegal_move = PieceMove { from: 52, to: 36 }; // black pawn on e7 with white to move
            board.perform_move(&illegal_move, &mut state);

            // Cancel should restore
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
            // Note: This tests that cancel works even when promotion isn't handled yet
            let mut board = create_empty_board();

            // White pawn on a7 about to promote
            board.white_pawns = 1 << 48; // a7
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state = GameState::new(&board);

            let (original_board, original_state) = capture_state(&board, &state);

            // Move pawn to a8 (promotion square) - promotion not implemented yet
            let move_piece = PieceMove { from: 48, to: 56 };
            board.perform_move(&move_piece, &mut state);

            // Cancel should still restore correctly
            verify_cancel_restores(
                &original_board,
                &original_state,
                &mut board,
                &mut state,
                &move_piece,
            );
        }
    }
}
