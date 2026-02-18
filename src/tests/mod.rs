mod bitboard_conversion_tests;
mod converters_tests;
mod engine_tests;
mod move_cancelation_tests;
mod moves_tests;
mod occupancy_tests;
mod pin_check_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::enums::PieceColor;
    use crate::gamestate::{GameState, PieceMove};

    // Helper function to create a custom board for testing
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
            cached_pieces: [None; 64],
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

    mod pawn_moves_tests {
        use super::*;

        #[test]
        fn test_white_pawn_initial_two_square_move() {
            let mut board = create_empty_board();
            board.white_pawns = 1 << 8; // a2
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.pawn_moves(&state, &PieceColor::White);
            let move_strings = moves_to_strings(&moves);

            // Should be able to move to a3 (16) and a4 (24)
            assert_eq!(move_strings.len(), 2);
            assert!(move_strings.contains(&"8-16".to_string()));
            assert!(move_strings.contains(&"8-24".to_string()));
        }

        #[test]
        fn test_white_pawn_single_move_blocked_by_piece() {
            let mut board = create_empty_board();
            board.white_pawns = 1 << 8; // a2
            board.black_pawns = 1 << 16; // a3 - blocking piece
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.pawn_moves(&state, &PieceColor::White);
            let move_strings = moves_to_strings(&moves);

            // Should have no moves (blocked)
            assert_eq!(move_strings.len(), 0);
        }

        #[test]
        fn test_white_pawn_capture() {
            let mut board = create_empty_board();
            board.white_pawns = 1 << 8; // a2
            board.black_pawns = (1 << 17) | (1 << 15); // b3 and a3? Actually b3 is 17? Let's fix
            // Let's use standard squares: a2=8, b3=17, a3=16
            board.black_pawns = (1 << 17) | (1 << 16); // b3 and a3
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.pawn_moves(&state, &PieceColor::White);
            let move_strings = moves_to_strings(&moves);

            // Should be able to capture on b3 (17) and move forward to a3 (16) is blocked
            assert_eq!(move_strings.len(), 1);
            assert!(move_strings.contains(&"8-17".to_string()));
        }

        #[test]
        fn test_black_pawn_initial_two_square_move() {
            let mut board: Board = create_empty_board();
            board.black_pawns = 1 << 48;
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::Black;

            let moves: Vec<PieceMove> = board.pawn_moves(&state, &PieceColor::Black);
            let move_strings: Vec<String> = moves_to_strings(&moves);

            assert_eq!(move_strings.len(), 2);
            assert!(move_strings.contains(&"48-40".to_string()));
            assert!(move_strings.contains(&"48-32".to_string()));
        }

        #[test]
        fn test_en_passant_white() {
            let mut board: Board = create_empty_board();
            board.white_pawns = 1 << 32;
            board.black_pawns = 1 << 33;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.en_passant_target = Some(41);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.pawn_moves(&state, &PieceColor::White);
            let move_strings: Vec<String> = moves_to_strings(&moves);

            assert!(move_strings.contains(&"32-41".to_string()));
        }

        #[test]
        fn test_en_passant_black() {
            let mut board: Board = create_empty_board();
            board.black_pawns = 1 << 24;
            board.white_pawns = 1 << 15;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.en_passant_target = Some(16);
            state.whose_turn = PieceColor::Black;

            let moves: Vec<PieceMove> = board.pawn_moves(&state, &PieceColor::Black);
            let move_strings: Vec<String> = moves_to_strings(&moves);

            assert!(move_strings.contains(&"24-16".to_string()));
        }

        #[test]
        fn test_pawn_pinned_diagonally() {
            let mut board: Board = create_empty_board();
            board.white_pawns = 1 << 12;
            board.white_king = 1 << 4;
            board.black_queens = 1 << 28;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            state.pin_info.update(&board, &PieceColor::White);

            let moves: Vec<PieceMove> = board.pawn_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);
            println!("Pinned pawn moves: {:?}", move_strings);
        }
    }

    mod knight_moves_tests {
        use super::*;

        #[test]
        fn test_knight_center_moves() {
            let mut board = create_empty_board();
            board.white_knights = 1 << 27;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.knight_moves(&state, &PieceColor::White);

            // Knight on d4 should have 8 possible moves
            assert_eq!(moves.len(), 8);

            let expected_squares = vec![
                10, 12, 17, 21, 33, 37, 42, 44, // all knight destinations from d4
            ];

            for m in moves {
                assert!(expected_squares.contains(&m.to));
                assert_eq!(m.from, 27);
            }
        }

        #[test]
        fn test_knight_corner_moves() {
            let mut board = create_empty_board();
            board.white_knights = 1 << 0; // knight on a1 (0)
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.knight_moves(&state, &PieceColor::White);

            // Knight on a1 should have 2 possible moves
            assert_eq!(moves.len(), 2);

            let expected_squares = vec![10, 17]; // b3 and c2

            for m in moves {
                assert!(expected_squares.contains(&m.to));
                assert_eq!(m.from, 0);
            }
        }

        #[test]
        fn test_knight_blocked_by_teammates() {
            let mut board = create_empty_board();
            board.white_knights = 1 << 27; // knight on d4 (27)
            // Place friendly pieces on some destination squares
            board.white_pawns = (1 << 10) | (1 << 12); // block two squares
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.knight_moves(&state, &PieceColor::White);

            // Should have 6 moves (8 total minus 2 blocked)
            assert_eq!(moves.len(), 6);
        }

        #[test]
        fn test_knight_captures() {
            let mut board: Board = create_empty_board();
            board.white_knights = 1 << 27; // knight on d4 (27)
            // Place enemy pieces on some destination squares
            board.black_pawns = (1 << 10) | (1 << 12); // enemy on two squares
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves = board.knight_moves(&state, &PieceColor::White);

            // Should still have 8 moves (captures are allowed)
            assert_eq!(moves.len(), 8);

            // Verify capture moves are included
            let move_strings = moves_to_strings(&moves);
            assert!(move_strings.contains(&"27-10".to_string()));
            assert!(move_strings.contains(&"27-12".to_string()));
        }
    }

    mod bishop_moves_tests {
        use super::*;

        #[test]
        fn test_bishop_center_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_bishops = 1 << 27;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.bishop_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 13);
        }

        #[test]
        fn test_bishop_corner_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_bishops = 1 << 0;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.bishop_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 7);

            let expected: Vec<u8> = vec![9, 18, 27, 36, 45, 54, 63];
            for m in moves {
                assert!(expected.contains(&m.to));
                assert_eq!(m.from, 0);
            }
        }

        #[test]
        fn test_bishop_blocked_by_friendly() {
            let mut board: Board = create_empty_board();
            board.white_bishops = 1 << 27;
            board.white_pawns = (1 << 18) | (1 << 36);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.bishop_moves(&state, &PieceColor::White);
            assert!(moves.len() < 13);
        }

        #[test]
        fn test_bishop_captures() {
            let mut board = create_empty_board();
            board.white_bishops = 1 << 27;
            board.black_pawns = (1 << 18) | (1 << 36);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.bishop_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);
            assert!(move_strings.contains(&"27-18".to_string()));
            assert!(move_strings.contains(&"27-36".to_string()));
        }
    }

    mod rook_moves_tests {
        use super::*;

        #[test]
        fn test_rook_center_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_rooks = 1 << 27;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.rook_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 14);
        }

        #[test]
        fn test_rook_corner_empty_board() {
            let mut board = create_empty_board();
            board.white_rooks = 1 << 0;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.rook_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 14);
        }

        #[test]
        fn test_rook_blocked_by_friendly() {
            let mut board = create_empty_board();
            board.white_rooks = 1 << 27;
            board.white_pawns = (1 << 35) | (1 << 19);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.rook_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);
            assert!(!move_strings.contains(&"27-43".to_string()));
            assert!(!move_strings.contains(&"27-11".to_string()));
        }

        #[test]
        fn test_rook_captures() {
            let mut board: Board = create_empty_board();
            board.white_rooks = 1 << 27;
            board.black_pawns = (1 << 35) | (1 << 19);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.rook_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);
            assert!(move_strings.contains(&"27-35".to_string()));
            assert!(move_strings.contains(&"27-19".to_string()));
            assert!(!move_strings.contains(&"27-43".to_string()));
        }
    }

    mod queen_moves_tests {
        use super::*;

        #[test]
        fn test_queen_center_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_queens = 1 << 27;
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.queen_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 27);
        }

        #[test]
        fn test_queen_corner_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_queens = 1 << 0;
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.queen_moves(&state, &PieceColor::White);
            assert_eq!(moves.len(), 21);
        }

        #[test]
        fn test_queen_blocked_and_captures() {
            let mut board: Board = create_empty_board();
            board.white_queens = 1 << 27;
            board.white_pawns = 1 << 35;
            board.black_pawns = (1 << 19) | (1 << 18);
            board.white_occupancy();
            board.black_occupancy();
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.queen_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);

            assert!(move_strings.contains(&"27-19".to_string()));
            assert!(move_strings.contains(&"27-18".to_string()));

            assert!(!move_strings.contains(&"27-35".to_string()));
            assert!(!move_strings.contains(&"27-43".to_string()));
        }
    }

    mod king_moves_tests {
        use super::*;

        #[test]
        fn test_king_center_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_king = 1 << 27;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.king_moves(&state, &PieceColor::White);

            assert_eq!(moves.len(), 8);

            let expected: Vec<u8> = vec![18, 19, 20, 26, 28, 34, 35, 36];
            for m in moves {
                assert!(expected.contains(&m.to));
                assert_eq!(m.from, 27);
            }
        }

        #[test]
        fn test_king_corner_empty_board() {
            let mut board: Board = create_empty_board();
            board.white_king = 1 << 0;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.king_moves(&state, &PieceColor::White);

            assert_eq!(moves.len(), 3);

            let expected: Vec<u8> = vec![1, 8, 9];
            for m in moves {
                assert!(expected.contains(&m.to));
                assert_eq!(m.from, 0);
            }
        }

        #[test]
        fn test_king_cannot_move_into_check() {
            let mut board: Board = create_empty_board();
            board.white_king = 1 << 27;
            board.black_rooks = 1 << 35;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.king_moves(&state, &PieceColor::White);

            let move_strings: Vec<String> = moves_to_strings(&moves);
            assert!(!move_strings.contains(&"27-19".to_string()));
        }

        #[test]
        fn test_white_king_castling() {
            let mut board: Board = create_empty_board();
            board.white_king = 1 << 4;
            board.white_rooks = (1 << 0) | (1 << 7);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.castling_rights.white_three_zeros = true;
            state.castling_rights.white_two_zeros = true;
            state.whose_turn = PieceColor::White;

            let moves: Vec<PieceMove> = board.king_moves(&state, &PieceColor::White);
            let move_strings = moves_to_strings(&moves);

            println!("King moves with castling: {:?}", move_strings);
        }

        #[test]
        fn test_black_king_castling() {
            let mut board = create_empty_board();
            board.black_king = 1 << 60;
            board.black_rooks = (1 << 56) | (1 << 63);
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.castling_rights.black_three_zeros = true;
            state.castling_rights.black_two_zeros = true;
            state.whose_turn = PieceColor::Black;

            let moves: Vec<PieceMove> = board.king_moves(&state, &PieceColor::Black);
            let move_strings: Vec<String> = moves_to_strings(&moves);
            println!("Black king moves with castling: {:?}", move_strings);
        }
    }

    mod check_and_pin_tests {
        use super::*;

        #[test]
        fn test_pieces_cannot_move_when_double_check() {
            let mut board: Board = create_empty_board();

            board.white_king = 1 << 4;
            board.black_knights = 1 << 21;
            board.black_bishops = 1 << 11;
            board.white_knights = 1 << 18;
            board.total_occupancy();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.check_info.update(&board, &PieceColor::White);

            let knight_moves: Vec<PieceMove> = board.knight_moves(&state, &PieceColor::White);
            let bishop_moves: Vec<PieceMove> = board.bishop_moves(&state, &PieceColor::White);
            let rook_moves: Vec<PieceMove> = board.rook_moves(&state, &PieceColor::White);
            let queen_moves: Vec<PieceMove> = board.queen_moves(&state, &PieceColor::White);

            assert_eq!(knight_moves.len(), 0);
            assert_eq!(bishop_moves.len(), 0);
            assert_eq!(rook_moves.len(), 0);
            assert_eq!(queen_moves.len(), 0);
        }

        #[test]
        fn test_piece_must_block_check() {
            let mut board: Board = create_empty_board();
            board.white_king = 1 << 4;
            board.black_queens = 1 << 28;
            board.white_knights = 1 << 18;
            board.total_occupancy();
            board.update_full_cache();

            let mut state: GameState = GameState::new(&board);
            state.whose_turn = PieceColor::White;
            state.check_info.update(&board, &PieceColor::White);
            state.update_check_constraints(&board);

            let knight_moves: Vec<PieceMove> = board.knight_moves(&state, &PieceColor::White);

            println!(
                "Knight moves in check: {:?}",
                moves_to_strings(&knight_moves)
            );
        }
    }
}
