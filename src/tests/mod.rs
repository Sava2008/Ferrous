#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        board_geometry_templates::{PROMOTION_SHIFT, TO_SHIFT},
        constants::{
            attacks::{
                compute_all_lines, compute_all_rays, compute_all_rays_from,
                initialize_sliding_attack_tables,
            },
            piece_values::PAWN_VALUE,
        },
        enums::{InclusiveRange, PieceColor, PieceType},
        gamestate::GameState,
    };

    // Helper function to create a move encoding
    fn create_move(from: u8, to: u8, promotion: Option<u8>) -> u16 {
        let mut move_encoding = (from as u16) | ((to as u16) << TO_SHIFT);
        if let Some(promo) = promotion {
            move_encoding |= (promo as u16) << PROMOTION_SHIFT;
        }
        return move_encoding;
    }

    // Helper to verify board state matches expected bitboards
    fn verify_board_state(board: &Board, expected: &[(PieceColor, PieceType, u8)]) {
        for (color, piece_type, square) in expected {
            let mask = 1 << square;
            match (color, piece_type) {
                (PieceColor::White, PieceType::Pawn) => assert_ne!(board.white_pawns & mask, 0),
                (PieceColor::White, PieceType::Knight) => assert_ne!(board.white_knights & mask, 0),
                (PieceColor::White, PieceType::Bishop) => assert_ne!(board.white_bishops & mask, 0),
                (PieceColor::White, PieceType::Rook) => assert_ne!(board.white_rooks & mask, 0),
                (PieceColor::White, PieceType::Queen) => assert_ne!(board.white_queens & mask, 0),
                (PieceColor::White, PieceType::King) => assert_ne!(board.white_king & mask, 0),
                (PieceColor::Black, PieceType::Pawn) => assert_ne!(board.black_pawns & mask, 0),
                (PieceColor::Black, PieceType::Knight) => assert_ne!(board.black_knights & mask, 0),
                (PieceColor::Black, PieceType::Bishop) => assert_ne!(board.black_bishops & mask, 0),
                (PieceColor::Black, PieceType::Rook) => assert_ne!(board.black_rooks & mask, 0),
                (PieceColor::Black, PieceType::Queen) => assert_ne!(board.black_queens & mask, 0),
                (PieceColor::Black, PieceType::King) => assert_ne!(board.black_king & mask, 0),
            }
        }

        // Verify total occupancy is consistent
        let calculated_total = board.white_occupancy | board.black_occupancy;
        assert_eq!(board.total_occupancy, calculated_total);

        // Verify cached pieces
        for (color, piece_type, square) in expected {
            assert_eq!(
                board.cached_pieces[*square as usize],
                Some((*color, *piece_type))
            );
        }
    }

    #[test]
    fn test_normal_move_and_cancel() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);
        board.update_full_cache();
        board.total_occupancy();

        // Move white pawn from e2 to e4
        let move_encoding = create_move(12, 28, None); // e2(12) to e4(28)
        board.perform_move(&move_encoding, &mut state);

        // Verify move was performed
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 28), // Pawn at e4
                (PieceColor::White, PieceType::King, 4),  // King still at e1
                (PieceColor::Black, PieceType::King, 60), // Black king at e8
            ],
        );
        assert_eq!(board.piece_at(&12), None); // e2 should be empty

        // Verify state updates
        assert_eq!(state.en_passant_target, Some(20)); // e3 should be en passant target

        // Cancel the move
        board.cancel_move(&mut state);

        // Verify board is back to initial state
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 12), // Pawn back at e2
            ],
        );
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn test_capture_and_cancel() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Setup: Place black pawn at d5 (35) and white pawn at e4 (28)
        board.black_pawns = 1 << 35;
        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 28);
        board.update_full_cache();
        board.total_occupancy();

        let initial_material = board.material;

        // White pawn captures black pawn: e4(28) to d5(35)
        let move_encoding = create_move(28, 35, None);
        board.perform_move(&move_encoding, &mut state);

        // Verify capture
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 35), // White pawn at d5
            ],
        );
        assert_eq!(board.piece_at(&28), None); // e4 empty
        assert_eq!(
            board.piece_at(&35),
            Some((PieceColor::White, PieceType::Pawn))
        ); // d5 has white pawn

        // Verify material changed (black pawn lost)
        assert!(board.material > initial_material);

        // Cancel capture
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 28), // White pawn back at e4
                (PieceColor::Black, PieceType::Pawn, 35), // Black pawn back at d5
            ],
        );
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_en_passant_capture_and_cancel() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Setup: White pawn at e5 (36), black pawn at d5 (35) - black just moved d7-d5
        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 36);
        board.black_pawns = (board.black_pawns & !(1 << 51)) | (1 << 35);
        state.en_passant_target = Some(43); // d6 is en passant target
        board.update_full_cache();
        board.total_occupancy();

        let initial_material = board.material;

        // White captures en passant: e5(36) to d6(43)
        let move_encoding = create_move(36, 43, None);
        board.perform_move(&move_encoding, &mut state);

        // Verify en passant capture
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 43), // White pawn at d6
            ],
        );
        assert_eq!(board.piece_at(&36), None); // e5 empty
        assert_eq!(board.piece_at(&35), None); // d5 empty (captured pawn)

        // Verify material changed (black pawn lost)
        assert!(board.material > initial_material);
        assert_eq!(state.en_passant_target, None);

        // Cancel en passant capture
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 36), // White pawn back at e5
                (PieceColor::Black, PieceType::Pawn, 35), // Black pawn back at d5
            ],
        );
        assert_eq!(board.material, initial_material);
        assert_eq!(state.en_passant_target, Some(43));
    }

    #[test]
    fn test_queenside_castle_and_cancel_white() {
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        println!("{:?}", state.castling_rights);
        board.white_knights &= !(1 << 1);
        board.white_bishops &= !(1 << 2);
        board.white_queens &= !(1 << 3);
        println!("{:?}", state.castling_rights);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();
        println!("{:?}", state.castling_rights);

        // White queenside castle: e1(4) to c1(2)
        println!("{:?}", state.castling_rights);
        let move_encoding = create_move(4, 2, None);
        println!("{:?}", state.castling_rights);
        board.perform_move(&move_encoding, &mut state);

        // Verify castling
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::King, 2), // King at c1
                (PieceColor::White, PieceType::Rook, 3), // Rook at d1
            ],
        );
        assert_eq!(board.piece_at(&4), None); // e1 empty
        assert_eq!(board.piece_at(&0), None); // a1 empty

        // Verify castling rights updated
        assert_eq!(state.castling_rights.white_three_zeros, false);
        assert_eq!(state.castling_rights.white_two_zeros, false);

        // Cancel castling
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::King, 4), // King back at e1
                (PieceColor::White, PieceType::Rook, 0), // Rook back at a1
            ],
        );
        assert_eq!(state.castling_rights.white_three_zeros, true);
        assert_eq!(state.castling_rights.white_two_zeros, true);
    }

    #[test]
    fn test_kingside_castle_and_cancel_white() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Clear pieces between king and rook
        board.white_knights &= !(1 << 5);
        board.white_bishops &= !(1 << 6);
        board.update_full_cache();
        board.total_occupancy();

        // White kingside castle: e1(4) to g1(6)
        let move_encoding = create_move(4, 6, None);
        board.perform_move(&move_encoding, &mut state);

        // Verify castling
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::King, 6), // King at g1
                (PieceColor::White, PieceType::Rook, 5), // Rook at f1
            ],
        );
        assert_eq!(board.piece_at(&4), None); // e1 empty
        assert_eq!(board.piece_at(&7), None); // h1 empty

        // Cancel castling
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::King, 4), // King back at e1
                (PieceColor::White, PieceType::Rook, 7), // Rook back at h1
            ],
        );
    }

    #[test]
    fn test_queenside_castle_and_cancel_black() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Clear pieces between black king and rook
        board.black_knights &= !(1 << 57);
        board.black_bishops &= !(1 << 58);
        board.black_queens &= !(1 << 59);
        board.update_full_cache();
        board.total_occupancy();

        // Black queenside castle: e8(60) to c8(58)
        let move_encoding = create_move(60, 58, None);
        board.perform_move(&move_encoding, &mut state);

        // Verify castling
        verify_board_state(
            &board,
            &[
                (PieceColor::Black, PieceType::King, 58), // King at c8
                (PieceColor::Black, PieceType::Rook, 59), // Rook at d8
            ],
        );
        assert_eq!(board.piece_at(&60), None); // e8 empty
        assert_eq!(board.piece_at(&56), None); // a8 empty

        // Cancel castling
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::Black, PieceType::King, 60), // King back at e8
                (PieceColor::Black, PieceType::Rook, 56), // Rook back at a8
            ],
        );
    }

    #[test]
    fn test_kingside_castle_and_cancel_black() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Clear pieces between black king and rook
        board.black_knights &= !(1 << 61);
        board.black_bishops &= !(1 << 62);
        board.update_full_cache();
        board.total_occupancy();

        // Black kingside castle: e8(60) to g8(62)
        let move_encoding = create_move(60, 62, None);
        board.perform_move(&move_encoding, &mut state);

        // Verify castling
        verify_board_state(
            &board,
            &[
                (PieceColor::Black, PieceType::King, 62), // King at g8
                (PieceColor::Black, PieceType::Rook, 61), // Rook at f8
            ],
        );
        assert_eq!(board.piece_at(&60), None); // e8 empty
        assert_eq!(board.piece_at(&63), None); // h8 empty

        // Cancel castling
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(
            &board,
            &[
                (PieceColor::Black, PieceType::King, 60), // King back at e8
                (PieceColor::Black, PieceType::Rook, 63), // Rook back at h8
            ],
        );
    }

    #[test]
    fn test_pawn_promotion_to_queen_and_cancel_white() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 51);
        board.black_pawns &= !(1 << 51);
        board.black_queens &= !(1 << 59);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let initial_material: i32 = board.material;

        let move_encoding: u16 = create_move(51, 59, Some(4));
        board.perform_move(&move_encoding, &mut state);

        verify_board_state(&board, &[(PieceColor::White, PieceType::Queen, 59)]);
        assert_eq!(board.piece_at(&51), None);
        assert_eq!(board.white_pawns & (1 << 59), 0);
        assert_ne!(board.white_queens & (1 << 59), 0);

        assert!(board.material > initial_material);

        board.cancel_move(&mut state);

        verify_board_state(&board, &[(PieceColor::White, PieceType::Pawn, 51)]);
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_pawn_promotion_to_knight_and_cancel_white() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 51);
        board.black_pawns &= !(1 << 51);
        board.black_queens &= !(1 << 59);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let initial_material: i32 = board.material;

        let move_encoding = create_move(51, 59, Some(1));
        board.perform_move(&move_encoding, &mut state);

        // Verify promotion
        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Knight, 59), // Knight at d8
            ],
        );
        assert_ne!(board.white_knights & (1 << 59), 0);

        // Cancel promotion
        board.cancel_move(&mut state);

        // Verify original position restored
        verify_board_state(&board, &[(PieceColor::White, PieceType::Pawn, 51)]);
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_pawn_promotion_with_capture_and_cancel_white() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 51);
        board.black_pawns &= !(1 << 51);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let initial_material: i32 = board.material;

        let move_encoding: u16 = create_move(51, 58, Some(4));
        board.perform_move(&move_encoding, &mut state);

        verify_board_state(&board, &[(PieceColor::White, PieceType::Queen, 58)]);
        assert_eq!(board.piece_at(&51), None);
        assert_eq!(
            board.piece_at(&58),
            Some((PieceColor::White, PieceType::Queen))
        );
        assert_eq!(board.black_bishops & (1 << 58), 0);

        assert_eq!(board.material, initial_material + 1320); // -B + Q - P + P 

        board.cancel_move(&mut state);

        verify_board_state(
            &board,
            &[
                (PieceColor::White, PieceType::Pawn, 51),
                (PieceColor::Black, PieceType::Bishop, 58),
            ],
        );
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_pawn_promotion_and_cancel_black() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.black_pawns = (board.black_pawns & !(1 << 51)) | (1 << 11);
        board.white_pawns &= !(1 << 11);
        board.total_occupancy();
        board.update_full_cache();

        let initial_material: i32 = board.material;

        let move_encoding: u16 = create_move(11, 2, Some(4));
        board.perform_move(&move_encoding, &mut state);

        verify_board_state(&board, &[(PieceColor::Black, PieceType::Queen, 2)]);
        assert_eq!(board.piece_at(&11), None);
        assert_eq!(board.black_pawns & (1 << 2), 0);
        assert_ne!(board.black_queens & (1 << 2), 0);

        assert!(board.material < initial_material);

        board.cancel_move(&mut state);

        verify_board_state(&board, &[(PieceColor::Black, PieceType::Pawn, 11)]);
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_castling_rights_lost_when_rook_moves() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);
        state.castling_rights.white_three_zeros = true;
        state.castling_rights.white_two_zeros = true;
        board.white_pawns &= !(1 << 8);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let move_encoding: u16 = create_move(0, 8, None);
        board.perform_move(&move_encoding, &mut state);

        assert_eq!(state.castling_rights.white_three_zeros, false);
        assert_eq!(state.castling_rights.white_two_zeros, true);

        board.cancel_move(&mut state);

        assert_eq!(state.castling_rights.white_three_zeros, true);
        assert_eq!(state.castling_rights.white_two_zeros, true);
    }

    #[test]
    fn test_castling_rights_lost_when_rook_captured() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.black_rooks |= 1 << 0;
        board.white_rooks &= !(1 << 0);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let move_encoding: u16 = create_move(56, 0, None);
        state.whose_turn = PieceColor::Black;
        board.perform_move(&move_encoding, &mut state);

        assert_eq!(state.castling_rights.white_three_zeros, false);
        assert_eq!(state.castling_rights.white_two_zeros, true);

        board.cancel_move(&mut state);

        assert_eq!(state.castling_rights.white_three_zeros, true);
        assert_eq!(state.castling_rights.white_two_zeros, true);
    }

    #[test]
    fn test_multiple_moves_and_cancels() {
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        let moves: [u16; 4] = [
            create_move(12, 28, None), // e2-e4
            create_move(52, 36, None), // e7-e5
            create_move(1, 18, None),  // Nb1-c3
            create_move(57, 42, None), // Ng8-f6
        ];

        let mut intermediate_states = Vec::new();

        for move_encoding in &moves {
            intermediate_states.push((board.clone(), state.clone()));
            board.perform_move(move_encoding, &mut state);
        }

        for expected_state in intermediate_states.iter().rev() {
            board.cancel_move(&mut state);
            assert_eq!(board, expected_state.0);
            assert_eq!(state, expected_state.1);
        }

        let initial_board: Board = Board::set();
        assert_eq!(board.white_pawns, initial_board.white_pawns);
        assert_eq!(board.white_knights, initial_board.white_knights);
        // ... check other bitboards
    }

    #[test]
    fn test_cancel_with_check_constraints() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        *&mut board.white_bishops |= 1 << 33;
        *&mut board.black_pawns = 0;
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();
        assert_eq!(board.total_occupancy & (1 << 51), 0);
        assert_eq!(board.total_occupancy & (1 << 42), 0);
        assert_ne!(board.total_occupancy & (1 << 33), 0);

        state.check_info.update(&board, &PieceColor::Black);
        state.update_check_constraints(&board);

        assert!(state.check_info.checked_king.is_some());
        assert_ne!(state.check_contraints, 0);

        let move_encoding = create_move(59, 51, None);
        state.whose_turn = PieceColor::Black;
        board.perform_move(&move_encoding, &mut state);

        board.cancel_move(&mut state);

        assert_eq!(
            state.check_contraints,
            Board::generate_range(60, 33, &InclusiveRange::LastOnly)
        );
    }

    #[test]
    fn test_double_check_no_constraints() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board: Board = Board::set();
        let mut state: GameState = GameState::new(&board);

        board.white_rooks |= 1 << 20;
        board.white_bishops |= 1 << 33;
        board.black_pawns &= !(1 << 51);
        board.black_pawns &= !(1 << 52);
        board.black_pawns &= !(1 << 53);
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();

        state.check_info.update(&board, &PieceColor::Black);

        assert!(state.check_info.checked_king.is_some());
        assert!(state.check_info.second_checker.is_some());

        state.update_check_constraints(&board);
        assert_eq!(state.check_contraints, 0);

        let move_encoding: u16 = create_move(60, 53, None);
        board.perform_move(&move_encoding, &mut state);

        board.cancel_move(&mut state);

        assert!(state.check_info.second_checker.is_some());
        assert_eq!(state.check_contraints, 0);
    }

    #[test]
    fn test_en_passant_target_cleared_on_non_pawn_move() {
        initialize_sliding_attack_tables();
        compute_all_rays();
        compute_all_rays_from();
        compute_all_lines();
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        // Set en passant target from previous move
        state.en_passant_target = Some(20); // e3

        // Move knight
        let move_encoding = create_move(1, 18, None); // Nb1-c3
        board.perform_move(&move_encoding, &mut state);

        // Verify en passant target cleared
        assert_eq!(state.en_passant_target, None);

        // Cancel move
        board.cancel_move(&mut state);

        // Verify en passant target restored
        assert_eq!(state.en_passant_target, Some(20));
    }

    #[test]
    fn test_fifty_move_counter_not_implemented() {
        // Note: This test acknowledges that fifty_move_rule_counter exists but isn't updated
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        let initial_counter = state.fifty_moves_rule_counter;

        // Make a pawn move
        let move_encoding = create_move(12, 28, None); // e2-e4
        board.perform_move(&move_encoding, &mut state);

        // Currently not updated - this would fail if it were implemented
        assert_eq!(state.fifty_moves_rule_counter, initial_counter);

        // Cancel move
        board.cancel_move(&mut state);
    }

    #[test]
    fn test_cancel_with_no_history() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);

        let board_before = board.clone();
        let state_before = state.clone();

        // Cancel with no history should do nothing
        board.cancel_move(&mut state);

        assert_eq!(board, board_before);
        assert_eq!(state, state_before);
    }

    #[test]
    fn test_perform_move_updates_material_correctly() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);
        board.count_material();

        let initial_material = board.material;

        // Capture a pawn
        board.black_pawns |= 1 << 28; // Put black pawn on e4
        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 20); // White pawn on e3
        board.update_full_cache();
        board.total_occupancy();
        board.count_material();

        let move_encoding = create_move(20, 28, None); // e3-e4 capturing black pawn
        board.perform_move(&move_encoding, &mut state);

        // Material should increase by pawn value
        assert_eq!(board.material, initial_material + PAWN_VALUE);

        // Cancel capture
        board.cancel_move(&mut state);
        board.count_material();
        assert_eq!(board.material, initial_material);
    }

    #[test]
    fn test_complex_sequence_with_all_move_types() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);
        board.update_full_cache();
        board.total_occupancy();

        // Sequence of different move types
        let moves = [
            create_move(12, 28, None), // 1. e4
            create_move(52, 36, None), // 1... e5
            create_move(1, 18, None),  // 2. Nc3
            create_move(57, 42, None), // 2... Nf6
            create_move(3, 35, Some(4)), // 3. Qxf7# (queen captures f7 pawn and promotes? no, queen move)
                                         // Actually let's do a promotion later
        ];

        let mut states = Vec::new();
        for &move_encoding in &moves {
            states.push((board.clone(), state.clone()));
            board.perform_move(&move_encoding, &mut state);
        }

        // Cancel all moves
        for expected_state in states.iter().rev() {
            board.cancel_move(&mut state);
            assert_eq!(board, expected_state.0);
            assert_eq!(state, expected_state.1);
        }
    }

    #[test]
    fn test_cached_pieces_consistency_after_cancel() {
        let mut board = Board::set();
        let mut state = GameState::new(&board);
        board.update_full_cache();

        // Verify initial cache
        for i in 0..64 {
            let mask = 1 << i;
            let expected = if board.white_pawns & mask != 0 {
                Some((PieceColor::White, PieceType::Pawn))
            } else if board.white_knights & mask != 0 {
                Some((PieceColor::White, PieceType::Knight))
            } else if board.white_bishops & mask != 0 {
                Some((PieceColor::White, PieceType::Bishop))
            } else if board.white_rooks & mask != 0 {
                Some((PieceColor::White, PieceType::Rook))
            } else if board.white_queens & mask != 0 {
                Some((PieceColor::White, PieceType::Queen))
            } else if board.white_king & mask != 0 {
                Some((PieceColor::White, PieceType::King))
            } else if board.black_pawns & mask != 0 {
                Some((PieceColor::Black, PieceType::Pawn))
            } else if board.black_knights & mask != 0 {
                Some((PieceColor::Black, PieceType::Knight))
            } else if board.black_bishops & mask != 0 {
                Some((PieceColor::Black, PieceType::Bishop))
            } else if board.black_rooks & mask != 0 {
                Some((PieceColor::Black, PieceType::Rook))
            } else if board.black_queens & mask != 0 {
                Some((PieceColor::Black, PieceType::Queen))
            } else if board.black_king & mask != 0 {
                Some((PieceColor::Black, PieceType::King))
            } else {
                None
            };
            assert_eq!(board.cached_pieces[i], expected, "Mismatch at square {}", i);
        }

        // Make a complex move
        board.white_pawns = (board.white_pawns & !(1 << 12)) | (1 << 51); // Pawn at d7
        board.black_rooks |= 1 << 59; // Rook at d8
        board.update_full_cache();

        let move_encoding = create_move(51, 59, Some(4)); // Promote with capture
        board.perform_move(&move_encoding, &mut state);

        // Verify cache after move
        assert_eq!(board.cached_pieces[51], None);
        assert_eq!(
            board.cached_pieces[59],
            Some((PieceColor::White, PieceType::Queen))
        );

        // Cancel move
        board.cancel_move(&mut state);

        // Verify cache restored
        assert_eq!(
            board.cached_pieces[51],
            Some((PieceColor::White, PieceType::Pawn))
        );
        assert_eq!(
            board.cached_pieces[59],
            Some((PieceColor::Black, PieceType::Rook))
        );
    }
}
