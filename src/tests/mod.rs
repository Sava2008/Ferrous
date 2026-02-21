#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board_geometry_templates::{
        FROM_MASK, PROMOTION_MASK, PROMOTION_SHIFT, TO_MASK, TO_SHIFT,
    };
    use crate::constants::attacks::{
        compute_all_lines, compute_all_piece_improvements, compute_all_rays, compute_all_rays_from,
        initialize_sliding_attack_tables,
    };
    use crate::converters::fen_converter::fen_to_board;
    use crate::enums::GameResult;
    use crate::{
        enums::{PieceColor, PieceType},
        gamestate::{CastlingRights, CheckInfo, GameState, PinInfo},
    };

    #[test]
    fn test_simple_move() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        // e2-e4
        let move_data = encode_move(12, 28, 0); // from e2(12) to e4(28), no promotion

        board.perform_move(&move_data, &mut state);
        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_capture_move() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        // e4xf5 (capture)
        let move_data = encode_move(28, 21, 0); // e4(28) to f5(21)

        board.perform_move(&move_data, &mut state);
        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_castling_kingside() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/1NBQK2R w KQkq - 0 1");
        let initial_board: Board = board.clone();
        let initial_state: crate::gamestate::GameState = state.clone();
        println!("board {board:?}");

        let move_data: u16 = encode_move(4, 6, 0);

        board.perform_move(&move_data, &mut state);

        assert_eq!(board.white_king.trailing_zeros(), 6); // King on g1
        assert_eq!(board.white_rooks.trailing_zeros(), 5); // Rook on f1

        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_castling_queenside() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        // O-O-O (e1-c1)
        let move_data = encode_move(4, 2, 0);

        board.perform_move(&move_data, &mut state);

        // Verify castling happened
        assert_eq!(board.white_king.trailing_zeros(), 2); // King on c1
        assert_eq!(board.white_rooks.trailing_zeros(), 3); // Rook on d1

        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_en_passant() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        // d7-d5 (sets up en passant)
        let move_data = encode_move(51, 35, 0); // d7(51) to d5(35)
        board.perform_move(&move_data, &mut state);

        // Now e4xd5 en passant
        let ep_move = encode_move(28, 35, 0); // e4(28) to d5(35)
        board.perform_move(&ep_move, &mut state);

        // Cancel both moves
        board.cancel_move(&mut state);
        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_pawn_promotion() {
        initialize_sliding_attack_tables();
        compute_all_lines();
        let (mut board, mut state) = fen_to_board("8/4P3/8/8/8/8/8/8 w - - 0 1");
        board.total_occupancy();
        let initial_board = board.clone();
        let initial_state = state.clone();

        // e7-e8=Q (promotion to queen)
        let move_data = encode_move(52, 60, 4); // Queen promotion

        board.perform_move(&move_data, &mut state);

        // Verify promotion
        assert_eq!(board.white_queens.trailing_zeros(), 60);
        assert_eq!(board.white_pawns & (1 << 52), 0);

        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_promotion_with_capture() {
        let (mut board, mut state) = fen_to_board("rnbqkbnr/4P3/8/8/8/8/8/8 w - - 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        // e7xd8=Q (capture and promotion)
        let move_data: u16 = encode_move(52, 59, 4); // Capture on d8 with queen promotion

        board.perform_move(&move_data, &mut state);

        // Verify promotion and capture
        assert_eq!(board.white_queens.trailing_zeros(), 59);
        assert_eq!(board.white_pawns & (1 << 52), 0);
        assert_eq!(board.black_queens, 0); // Black queen captured

        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_multiple_moves_sequence() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let initial_board = board.clone();
        let initial_state = state.clone();

        let moves = vec![
            encode_move(12, 28, 0), // e4
            encode_move(52, 36, 0), // e5
            encode_move(6, 21, 0),  // Nf3
            encode_move(62, 45, 0), // Nc6
            encode_move(5, 19, 0),  // Bb5 (Ruy Lopez)
        ];

        // Make all moves
        for m in &moves {
            board.perform_move(m, &mut state);
        }

        // Unmake all moves in reverse order
        for _ in moves {
            board.cancel_move(&mut state);
        }

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    #[test]
    fn test_check_evasion() {
        let (mut board, mut state) =
            fen_to_board("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 0 1");
        let initial_board: Board = board.clone();
        let initial_state: crate::gamestate::GameState = state.clone();

        // Kxe4 (king captures pawn)
        let move_data: u16 = encode_move(60, 28, 0);

        board.perform_move(&move_data, &mut state);
        board.cancel_move(&mut state);

        assert_eq!(board, initial_board);
        assert_eq!(state, initial_state);
    }

    // Helper function to encode moves
    fn encode_move(from: u8, to: u8, promotion: u8) -> u16 {
        let mut move_data = (from as u16) & FROM_MASK;
        move_data |= ((to as u16) & FROM_MASK) << TO_SHIFT;
        move_data |= ((promotion as u16) & 0b111) << PROMOTION_SHIFT;
        move_data
    }

    // Test the move encoding/decoding itself
    #[test]
    fn test_move_encoding() {
        let from = 12;
        let to = 28;
        let promo = 4;

        let encoded = encode_move(from, to, promo);

        let decoded_from = (encoded & FROM_MASK) as u8;
        let decoded_to = ((encoded >> TO_SHIFT) & FROM_MASK) as u8;
        let decoded_promo = ((encoded >> PROMOTION_SHIFT) & 0b111) as u8;

        assert_eq!(decoded_from, from);
        assert_eq!(decoded_to, to);
        assert_eq!(decoded_promo, promo);
    }

    // Helper function to create a board from FEN-like position
    fn setup_board_from_bitboards(
        white_pieces: Vec<(PieceType, Vec<usize>)>,
        black_pieces: Vec<(PieceType, Vec<usize>)>,
    ) -> Board {
        let mut board = Board {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
            white_occupancy: 0,
            black_occupancy: 0,
            total_occupancy: 0,
            cached_pieces: [None; 64],
            material: 0,
        };

        for (piece_type, squares) in white_pieces {
            for sq in squares {
                let mask = 1 << sq;
                match piece_type {
                    PieceType::Pawn => board.white_pawns |= mask,
                    PieceType::Knight => board.white_knights |= mask,
                    PieceType::Bishop => board.white_bishops |= mask,
                    PieceType::Rook => board.white_rooks |= mask,
                    PieceType::Queen => board.white_queens |= mask,
                    PieceType::King => board.white_king |= mask,
                }
            }
        }

        for (piece_type, squares) in black_pieces {
            for sq in squares {
                let mask = 1 << sq;
                match piece_type {
                    PieceType::Pawn => board.black_pawns |= mask,
                    PieceType::Knight => board.black_knights |= mask,
                    PieceType::Bishop => board.black_bishops |= mask,
                    PieceType::Rook => board.black_rooks |= mask,
                    PieceType::Queen => board.black_queens |= mask,
                    PieceType::King => board.black_king |= mask,
                }
            }
        }

        board.total_occupancy();
        board.white_occupancy();
        board.black_occupancy();
        board.update_full_cache();
        board.count_material();
        initialize_sliding_attack_tables();
        compute_all_lines();
        compute_all_piece_improvements();
        compute_all_rays();
        compute_all_rays_from();
        board
    }

    // Helper to convert move to readable format
    fn move_to_string(m: u16) -> String {
        let from = (m & FROM_MASK) as u8;
        let to = ((m >> TO_SHIFT) & FROM_MASK) as u8;
        let promotion = (m >> PROMOTION_SHIFT) & 0b111;

        let from_file = (from % 8) as u8;
        let from_rank = (from / 8) as u8;
        let to_file = (to % 8) as u8;
        let to_rank = (to / 8) as u8;

        let promotion_char = match promotion {
            1 => "n",
            2 => "b",
            3 => "r",
            4 => "q",
            _ => "",
        };

        format!(
            "{}{}{}{}{}",
            (b'a' + from_file) as char,
            from_rank + 1,
            (b'a' + to_file) as char,
            to_rank + 1,
            promotion_char
        )
    }

    #[test]
    fn test_castling_edge_cases() {
        // Test 1: Can't castle through check (white kingside)
        let mut board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]), // e1
                (PieceType::Rook, vec![7]), // h1
            ],
            vec![
                (PieceType::Rook, vec![22]), // Black rook on g3 attacking g1
            ],
        );

        let mut state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights {
                white_two_zeros: true,    // kingside
                white_three_zeros: false, // queenside
                black_two_zeros: false,
                black_three_zeros: false,
            },
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let moves = board.king_moves(&state, &PieceColor::White);
        let castling_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                let from = (*m & FROM_MASK) as u8;
                let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
                from == 4 && (to == 6 || to == 2)
            })
            .collect();

        assert!(
            castling_moves.is_empty(),
            "Should not be able to castle through check"
        );

        // Test 2: Can't castle out of check
        board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]), // e1
                (PieceType::Rook, vec![7]), // h1
            ],
            vec![
                (PieceType::Rook, vec![20]), // Black rook on e3 giving check
            ],
        );

        state.check_info = CheckInfo {
            checked_king: Some(4),
            first_checker: Some(20),
            second_checker: None,
        };

        let moves = board.king_moves(&state, &PieceColor::White);
        let castling_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                let from = (*m & FROM_MASK) as u8;
                let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
                from == 4 && (to == 6 || to == 2)
            })
            .collect();

        assert!(
            castling_moves.is_empty(),
            "Should not be able to castle out of check"
        );

        // Test 3: Can castle when path is clear and safe (white kingside)
        board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]), // e1
                (PieceType::Rook, vec![7]), // h1
            ],
            vec![], // No black pieces
        );
        state.check_info.update(&board, &PieceColor::White);
        state.update_check_constraints(&board);

        let moves: Vec<u16> = board.king_moves(&state, &PieceColor::White);
        let kingside_castle = moves.iter().find(|m| {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
            from == 4 && to == 6
        });

        assert!(
            kingside_castle.is_some(),
            "Should be able to castle kingside"
        );

        // Test 4: Can't castle if squares occupied
        board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),   // e1
                (PieceType::Rook, vec![7]),   // h1
                (PieceType::Knight, vec![5]), // Knight on f1 blocking
            ],
            vec![],
        );

        let moves = board.king_moves(&state, &PieceColor::White);
        let kingside_castle = moves.iter().find(|m| {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
            from == 4 && to == 6
        });

        assert!(
            kingside_castle.is_none(),
            "Should not castle through occupied square"
        );
    }

    #[test]
    fn test_en_passant_edge_cases() {
        let mut board: Board = setup_board_from_bitboards(
            vec![(PieceType::Pawn, vec![28]), (PieceType::King, vec![4])],
            vec![(PieceType::Pawn, vec![27]), (PieceType::King, vec![60])],
        );

        let mut state: GameState = GameState {
            en_passant_target: Some(20),
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let moves: Vec<u16> = board.pawn_moves(&state, &PieceColor::Black);
        let en_passant_capture: Option<&u16> = moves.iter().find(|m: &&u16| {
            let from: u8 = (*m & FROM_MASK) as u8;
            let to: u8 = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
            from == 27 && to == 20
        });

        assert!(
            en_passant_capture.is_some(),
            "White should be able to capture en passant"
        );

        // Test 2: En passant not available after one move
        state.en_passant_target = None;
        let moves = board.pawn_moves(&state, &PieceColor::Black);
        let en_passant_capture = moves.iter().find(|m| {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
            from == 27 && to == 20
        });

        assert!(
            en_passant_capture.is_none(),
            "En passant should not be available next turn"
        );

        // Better: Put white king on e1, black rook on f1 would be check after capture
        board = setup_board_from_bitboards(
            vec![
                (PieceType::Pawn, vec![36]), // White pawn on e5
                (PieceType::King, vec![4]),  // White king on e1
            ],
            vec![
                (PieceType::Pawn, vec![37]), // Black pawn on f5
                (PieceType::Rook, vec![5]),  // Black rook on f1 (will attack e1 after pawn capture)
                (PieceType::King, vec![60]), // Black king on e8
            ],
        );

        state.en_passant_target = Some(45); // f6
        state.check_info.update(&board, &PieceColor::White);
        state.update_check_constraints(&board);

        let moves = board.pawn_moves(&state, &PieceColor::White);
        let illegal_en_passant = moves.iter().find(|m| {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
            from == 36 && to == 45
        });

        assert!(
            illegal_en_passant.is_none(),
            "En passant should be illegal if it exposes check"
        );
    }

    #[test]
    fn test_pin_handling() {
        // Test 1: Absolute pin along file
        let board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),  // White king on e1
                (PieceType::Rook, vec![20]), // White rook on e3 (pinned by black rook)
            ],
            vec![
                (PieceType::Rook, vec![52]), // Black rook on e7
                (PieceType::King, vec![60]), // Black king on e8
            ],
        );

        let mut state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 1 << 20, // e3 is pinned
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let moves = board.rook_moves(&state, &PieceColor::White);

        // Pinned rook should only move along the pin line (e-file)
        for m in &moves {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;

            if from == 20 {
                // Check if move is along e-file
                assert_eq!(from % 8, to % 8, "Pinned piece must move along pin line");
            }
        }

        // Better: Put black queen on a1-h8 diagonal
        let board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),    // White king on e1
                (PieceType::Bishop, vec![18]), // White bishop on c3 (pinned along a1-h8 diagonal)
            ],
            vec![
                (PieceType::Queen, vec![49]), // Black queen on b2 (attacking along diagonal)
                (PieceType::King, vec![60]),  // Black king on e8
            ],
        );

        state.pin_info.pinned_pieces = 1 << 18; // c3 is pinned

        let moves = board.bishop_moves(&state, &PieceColor::White);

        for m in &moves {
            let from = (*m & FROM_MASK) as u8;
            let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;

            if from == 18 {
                // Should only move along the a1-h8 diagonal (difference in rank and file equal)
                let from_rank = from / 8;
                let from_file = from % 8;
                let to_rank = to / 8;
                let to_file = to % 8;

                let rank_diff = (to_rank as i8 - from_rank as i8).abs();
                let file_diff = (to_file as i8 - from_file as i8).abs();

                assert_eq!(
                    rank_diff, file_diff,
                    "Pinned bishop must move along same diagonal"
                );
            }
        }
    }

    #[test]
    fn test_double_check_handling() {
        // Position with double check - only king moves should be generated
        let board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),    // White king on e1
                (PieceType::Rook, vec![3]),    // White rook on d1
                (PieceType::Bishop, vec![11]), // White bishop on d3
            ],
            vec![
                (PieceType::Rook, vec![21]),   // Black rook on e3 (check)
                (PieceType::Bishop, vec![22]), // Black bishop on f3 (check - double check)
                (PieceType::King, vec![60]),   // Black king on e8
            ],
        );

        let state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: Some(4),
                first_checker: Some(21),
                second_checker: Some(22),
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let rook_moves = board.rook_moves(&state, &PieceColor::White);
        let bishop_moves = board.bishop_moves(&state, &PieceColor::White);
        let pawn_moves = board.pawn_moves(&state, &PieceColor::White);
        let knight_moves = board.knight_moves(&state, &PieceColor::White);
        let queen_moves = board.queen_moves(&state, &PieceColor::White);
        let king_moves = board.king_moves(&state, &PieceColor::White);

        assert!(rook_moves.is_empty(), "No rook moves in double check");
        assert!(bishop_moves.is_empty(), "No bishop moves in double check");
        assert!(pawn_moves.is_empty(), "No pawn moves in double check");
        assert!(knight_moves.is_empty(), "No knight moves in double check");
        assert!(queen_moves.is_empty(), "No queen moves in double check");
        assert!(
            !king_moves.is_empty(),
            "King must have moves in double check"
        );
    }

    #[test]
    fn test_discovered_check_handling() {
        // Position where moving a piece discovers check
        let mut board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),    // White king on e1
                (PieceType::Bishop, vec![11]), // White bishop on d3 (blocking rook)
                (PieceType::Rook, vec![3]),    // White rook on d1 (behind bishop)
            ],
            vec![
                (PieceType::Rook, vec![52]), // Black rook on e7
                (PieceType::King, vec![60]), // Black king on e8
            ],
        );

        // Initial state - no check
        let mut state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        // Bishop moves that would discover check should be illegal
        let bishop_moves = board.bishop_moves(&state, &PieceColor::White);

        for m in &bishop_moves {
            let from = (*m & FROM_MASK) as u8;
            if from == 11 {
                let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;

                // Simulate the move to check if it would discover check
                let mut temp_board = board.clone();
                let mut temp_state = state.clone();

                // Create a move that moves the bishop
                let move_data = (from as u16) | ((to as u16) << TO_SHIFT);
                temp_board.perform_move(&move_data, &mut temp_state);

                // After move, white king should not be in check
                assert!(
                    !temp_board.is_king_attacked(&PieceColor::White),
                    "Move {} should not leave king in check",
                    move_to_string(*m)
                );
            }
        }
    }

    #[test]
    fn test_promotion_edge_cases() {
        // Test promotion under check
        let board: Board = setup_board_from_bitboards(
            vec![
                (PieceType::Pawn, vec![52]), // White pawn on e7
                (PieceType::King, vec![4]),  // White king on e1
            ],
            vec![
                (PieceType::Rook, vec![59]), // Black rook on e8 (will be attacked after promotion)
                (PieceType::King, vec![60]), // Black king on d8
            ],
        );

        let state: GameState = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let pawn_moves: Vec<u16> = board.pawn_moves(&state, &PieceColor::White);

        let promotions: Vec<_> = pawn_moves
            .iter()
            .filter(|m: &&u16| {
                let from: u8 = (*m & FROM_MASK) as u8;
                let to: u8 = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
                from == 52 && to == 59 // Promoting to 8th rank
            })
            .collect();

        assert_eq!(promotions.len(), 4, "Should have 4 promotion options");

        // Test promotion delivering check
        for m in promotions {
            let promotion_type: u16 = (*m >> PROMOTION_SHIFT) & 0b111;
            let mut temp_board: Board = board.clone();
            let mut temp_state: GameState = state.clone();

            temp_board.perform_move(m, &mut temp_state);

            // After promotion, black king might be in check depending on piece chosen

            // All promotions are legal as long as they don't leave white king in check
            if promotion_type == 3 || promotion_type == 4 {
                assert!(
                    !temp_board.is_king_attacked(&PieceColor::White),
                    "Promotion move {} should not leave own king in check",
                    move_to_string(*m)
                );
            }
        }
    }

    #[test]
    fn test_blocking_check_constraints() {
        // Single check - pieces can block or capture
        let board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),    // White king on e1
                (PieceType::Rook, vec![19]),   // White rook on e3
                (PieceType::Knight, vec![17]), // White knight on c3
                (PieceType::Bishop, vec![21]), // White bishop on f3
            ],
            vec![
                (PieceType::Rook, vec![52]), // Black rook on e7 (giving check)
                (PieceType::King, vec![60]), // Black king on e8
            ],
        );

        // The checking piece is on e7, check line is e-file from e2-e6
        let check_constraint = (1 << 12) | (1 << 20) | (1 << 28) | (1 << 36) | (1 << 44); // e2-e6

        let state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: Some(4),
                first_checker: Some(52),
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: check_constraint,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let rook_moves = board.rook_moves(&state, &PieceColor::White);
        let knight_moves = board.knight_moves(&state, &PieceColor::White);
        let bishop_moves = board.bishop_moves(&state, &PieceColor::White);

        // All moves must either capture the checking piece or block the check
        for moves in &[rook_moves, bishop_moves] {
            for m in moves {
                let to = ((*m >> TO_SHIFT) & FROM_MASK) as u8;
                if to != 52 {
                    // Not capturing the checking piece
                    assert!(
                        check_constraint & (1 << to) != 0,
                        "Move {} must block check (land on e2-e6)",
                        move_to_string(*m)
                    );
                }
            }
        }

        // Knights can only capture the checking piece (can't block)
        for m in knight_moves {
            let to = ((m >> TO_SHIFT) & FROM_MASK) as u8;
            assert_eq!(to, 52, "Knight must capture checking piece");
        }
    }

    #[test]
    fn test_stalemate_detection() {
        let board: Board = setup_board_from_bitboards(
            vec![(PieceType::King, vec![0]), (PieceType::Queen, vec![53])],
            vec![(PieceType::King, vec![63])],
        );

        let state: GameState = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights {
                white_three_zeros: false,
                white_two_zeros: false,
                black_three_zeros: false,
                black_two_zeros: false,
            },
            check_info: CheckInfo {
                checked_king: None,
                first_checker: None,
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 0,
                black_king: 63,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let king_moves: Vec<u16> = board.king_moves(&state, &PieceColor::Black);
        assert!(
            king_moves.is_empty(),
            "King should have no legal moves (stalemate)"
        );

        // In a real game, we'd also check that no other pieces exist
        // Since white only has a king, no moves means stalemate
    }

    #[test]
    fn test_checkmate_detection() {
        // Back rank mate position
        let board = setup_board_from_bitboards(
            vec![
                (PieceType::King, vec![4]),  // White king on e1
                (PieceType::Pawn, vec![11]), // White pawn on d3 (blocking escape)
                (PieceType::Pawn, vec![13]), // White pawn on f3 (blocking escape)
                (PieceType::Rook, vec![5]),  // White rook on f2 (blocking f1)
                (PieceType::Rook, vec![3]),  // White rook on d2 (blocking d1)
            ],
            vec![
                (PieceType::Rook, vec![52]), // Black rook on e7 (giving check)
                (PieceType::Rook, vec![21]), // Black rook on e3 (taking away escape)
                (PieceType::King, vec![60]), // Black king on e8
            ],
        );

        let state = GameState {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            check_info: CheckInfo {
                checked_king: Some(4),
                first_checker: Some(52),
                second_checker: None,
            },
            pin_info: PinInfo {
                white_king: 4,
                black_king: 60,
                pinned_pieces: 0,
            },
            check_contraints: 0,
            moves_history: Vec::new(),
            fifty_moves_rule_counter: 0,
            total_moves_amount: 0,
            whose_turn: PieceColor::Black,
            result: GameResult::Going,
        };

        let king_moves = board.king_moves(&state, &PieceColor::White);
        assert!(
            king_moves.is_empty(),
            "King should have no legal moves (checkmate)"
        );

        // Check if any piece can block/capture
        let rook_moves = board.rook_moves(&state, &PieceColor::White);
        let pawn_moves = board.pawn_moves(&state, &PieceColor::White);

        assert!(rook_moves.is_empty(), "No rook moves in checkmate");
        assert!(pawn_moves.is_empty(), "No pawn moves in checkmate");
    }

    #[test]
    fn test_perft_positions() {
        // This would be a comprehensive test using known perft results
        // For now, we'll just test starting position move counts
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        compute_all_piece_improvements();

        let mut board = Board::set();
        let state = GameState::new(&board);
        board.total_occupancy();

        let total_moves = board.knight_moves(&state, &PieceColor::White).len()
            + board.pawn_moves(&state, &PieceColor::White).len()
            + board.king_moves(&state, &PieceColor::White).len();

        // Starting position: 20 moves (16 pawn moves + 4 knight moves)
        assert_eq!(
            total_moves, 20,
            "Starting position should have 20 legal moves"
        );

        // Test a few known positions
        // Position after 1.e4
        let mut board = Board::set();
        let mut state = GameState::new(&board);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        // Move e2-e4
        let e4_move: u16 = (12 as u16) | ((28 as u16) << TO_SHIFT); // e2(12) to e4(28)
        println!(
            "from {}, to {}",
            e4_move & FROM_MASK,
            (e4_move & TO_MASK) >> TO_SHIFT
        );
        board.perform_move(&e4_move, &mut state);

        let black_moves = board.knight_moves(&state, &PieceColor::Black).len()
            + board.pawn_moves(&state, &PieceColor::Black).len()
            + board.king_moves(&state, &PieceColor::Black).len();

        // After 1.e4, black should have 20 moves as well
        assert_eq!(black_moves, 20, "After 1.e4, black should have 20 moves");
    }

    #[test]
    fn test_is_square_attacked() -> () {
        initialize_sliding_attack_tables();
        let mut board: Board = Board::set();
        board.total_occupancy();
        for sq in 16..24 {
            assert!(board.is_square_attacked(sq, &PieceColor::White));
        }
        for sq in 40..48 {
            assert!(board.is_square_attacked(sq, &PieceColor::Black));
        }
    }

    #[test]
    fn test_is_square_attacked2() -> () {
        initialize_sliding_attack_tables();
        let mut board: Board = Board::set();
        board.white_pawns &= !(1 << 15);
        board.total_occupancy();
        for sq in (15..56).filter(|x| (*x + 1) % 8 == 0) {
            println!("{sq}");
            assert!(board.is_square_attacked(sq, &PieceColor::White));
        }
    }

    #[test]
    fn test_pawn_promotion2() -> () {
        let mut board: Board = setup_board_from_bitboards(
            vec![(PieceType::Pawn, vec![54])],
            vec![(PieceType::Rook, vec![62, 63])],
        );
        let mut state: GameState = GameState::new(&board);
        for m in board.pawn_moves(&state, &PieceColor::White) {
            println!(
                "from: {}, to: {}, promo: {}",
                m & FROM_MASK,
                (m & TO_MASK) >> TO_SHIFT,
                (m & PROMOTION_MASK) >> PROMOTION_SHIFT
            );
        }
    }
}
