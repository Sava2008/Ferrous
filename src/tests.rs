#[cfg(test)]
pub mod tests {
    use crate::{
        constants, converters,
        game_logic::{
            pieces::{Bishop, ChessPiece, King, Knight, Pawn, Queen, Rook, Void},
            state_enums::PieceColor,
        },
        helper_functions,
    };
    use ggez::mint::Point2;
    #[test]
    fn coords_to_index_test() -> () {
        assert_eq!(
            0,
            helper_functions::coords_to_index(Point2 { x: 0., y: 0. }).unwrap()
        );
        assert_eq!(
            35,
            helper_functions::coords_to_index(Point2 {
                x: 3. * constants::SQUARE_SIDE,
                y: 4. * constants::SQUARE_SIDE
            })
            .unwrap()
        );
        assert_eq!(
            63,
            helper_functions::coords_to_index(Point2 {
                x: 7. * constants::SQUARE_SIDE,
                y: 7. * constants::SQUARE_SIDE
            })
            .unwrap()
        );
    }

    #[test]
    fn index_to_coords_test() -> () {
        assert_eq!((4, 3), helper_functions::index_to_coords(35));
        assert_eq!((1, 6), helper_functions::index_to_coords(14));
        assert_eq!((6, 5), helper_functions::index_to_coords(53));
    }

    #[test]
    fn i8_coords_to_index_test() -> () {
        assert_eq!(7, helper_functions::i8_coords_to_index((0, 7)));
        assert_eq!(9, helper_functions::i8_coords_to_index((1, 1)));
        assert_eq!(37, helper_functions::i8_coords_to_index((4, 5)));
    }

    #[test]
    fn is_line_test() -> () {
        assert!(helper_functions::is_line(26, 29));
        assert!(helper_functions::is_line(56, 63));
        assert!(helper_functions::is_line(11, 43));
        assert!(!helper_functions::is_line(1, 30));
        assert!(!helper_functions::is_line(36, 40));
        assert!(!helper_functions::is_line(18, 27));
    }

    #[test]
    fn is_diagonal_test() -> () {
        assert!(helper_functions::is_diagonal(0, 63));
        assert!(helper_functions::is_diagonal(6, 20));
        assert!(helper_functions::is_diagonal(43, 16));
        assert!(!helper_functions::is_diagonal(10, 12));
        assert!(!helper_functions::is_diagonal(23, 41));
        assert!(!helper_functions::is_diagonal(60, 52));
    }

    #[test]
    fn is_adjancent_file_test() -> () {
        assert!(helper_functions::is_adjancent_file(33, 2));
        assert!(helper_functions::is_adjancent_file(61, 62));
        assert!(helper_functions::is_adjancent_file(3, 11));
        assert!(!helper_functions::is_adjancent_file(5, 7));
        assert!(!helper_functions::is_adjancent_file(39, 40));
        assert!(!helper_functions::is_adjancent_file(57, 44));
    }

    #[test]
    fn board_to_fen_test() -> () {
        assert_eq!(
            converters::board_to_fen(
                &[
                    ChessPiece::R(Rook::new(PieceColor::Black, 0, 0)),
                    ChessPiece::N(Knight::new(PieceColor::Black, 1, 1)),
                    ChessPiece::B(Bishop::new(PieceColor::Black, 2, 2)),
                    ChessPiece::Q(Queen::new(PieceColor::Black, 3, 3)),
                    ChessPiece::K(King::new(PieceColor::Black, 4, 4)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::P(Pawn::new(PieceColor::White, 55, 5)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::K(King::new(PieceColor::White, 62, 6)),
                    ChessPiece::Square(Void)
                ],
                PieceColor::White,
                [true, true, true, true],
                None,
                0,
                1
            ),
            "rnbqk3/8/8/8/8/8/7P/6K1 w KQkq - 0 1".to_string()
        );

        assert_eq!(
            converters::board_to_fen(
                &[
                    ChessPiece::R(Rook::new(PieceColor::Black, 0, 0)),
                    ChessPiece::N(Knight::new(PieceColor::Black, 1, 1)),
                    ChessPiece::B(Bishop::new(PieceColor::Black, 2, 2)),
                    ChessPiece::Q(Queen::new(PieceColor::Black, 3, 3)),
                    ChessPiece::K(King::new(PieceColor::Black, 4, 4)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::R(Rook::new(PieceColor::White, 21, 12)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Q(Queen::new(PieceColor::White, 33, 11)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::B(Bishop::new(PieceColor::White, 39, 10)),
                    ChessPiece::P(Pawn::new(PieceColor::Black, 40, 9)),
                    ChessPiece::P(Pawn::new(PieceColor::Black, 41, 8)),
                    ChessPiece::P(Pawn::new(PieceColor::Black, 42, 7)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::P(Pawn::new(PieceColor::White, 55, 5)),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::Square(Void),
                    ChessPiece::K(King::new(PieceColor::White, 62, 6)),
                    ChessPiece::Square(Void)
                ],
                PieceColor::White,
                [false, false, false, true],
                None,
                0,
                1
            ),
            "rnbqk3/8/5R2/8/1Q5B/ppp5/7P/6K1 w q - 0 1".to_string()
        );
    }
}
