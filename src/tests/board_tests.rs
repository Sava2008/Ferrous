#[test]
pub fn en_passant_test() -> () {
    use crate::game_logic::{
        Board,
        pieces::{ChessPiece, King, Pawn, Void},
        state_enums::{GameMode, KingChecked, PieceColor, PieceVariant},
    };
    use std::collections::{HashMap, HashSet};
    let mut test_board: Board = Board {
        squares: [
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
            ChessPiece::K(King::new(PieceColor::White, 2, 15)),
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
            ChessPiece::K(King::new(PieceColor::Black, 6, 14)),
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
            ChessPiece::P(Pawn {
                index: 25,
                value: 10,
                key: (PieceColor::Black, PieceVariant::P),
                was_moved: true,
                id: 3,
                is_pinned: false,
            }),
            ChessPiece::P(Pawn {
                index: 26,
                value: 10,
                key: (PieceColor::White, PieceVariant::P),
                was_moved: true,
                id: 8,
                is_pinned: false,
            }),
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
        ],
        white_locations: HashMap::from([(15, 2), (8, 25)]),
        black_locations: HashMap::from([(14, 6), (3, 26)]),
        white_vision: HashSet::from([1, 3, 9, 10, 11, 12]),
        black_vision: HashSet::from([7, 5, 15, 14, 13, 25, 27]),
        checked: KingChecked::None,
        gamemode: GameMode::MovementWhite,
        en_peasant_susceptible: Some(17),
        check: (KingChecked::None, None, None),
        chosen_piece: Some(26),
        dest_square: Some(17),
        engine_move: None,
        legal_moves: Vec::new(),
    };
    let pawn: &ChessPiece = &test_board.squares[test_board.chosen_piece.unwrap()];
    test_board.legal_moves = pawn
        .legal_moves(
            &test_board,
            test_board.en_peasant_susceptible,
            &test_board.check,
            2,
        )
        .unwrap();
    assert_eq!(test_board.legal_moves, vec![18, 17]);
    test_board
        .perform_move(
            pawn.index().unwrap(),
            17,
            test_board.en_peasant_susceptible,
            PieceColor::White,
        )
        .unwrap();

    assert!(test_board.squares[17].is_pawn());
    assert!(test_board.squares[25].is_void());
}
