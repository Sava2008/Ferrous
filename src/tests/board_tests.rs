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
        white_locations: HashMap::from([(15, 2), (8, 26)]),
        black_locations: HashMap::from([(14, 6), (3, 25)]),
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
        move_history: Vec::new(),
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
        .perform_move(pawn.index().unwrap(), 17, PieceColor::Black)
        .unwrap();

    assert!(test_board.squares[17].is_pawn());
    assert!(test_board.squares[26].is_void());
}

#[test]
pub fn move_cancellation_test1() -> () {
    use crate::{
        constants::PAWN_VALUE,
        game_logic::{
            Board,
            pieces::{ChessPiece, King, Pawn, Void},
            state_enums::{GameMode, KingChecked, PieceColor, PieceVariant},
        },
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
                value: PAWN_VALUE,
                key: (PieceColor::Black, PieceVariant::P),
                was_moved: true,
                id: 3,
                is_pinned: false,
            }),
            ChessPiece::P(Pawn {
                index: 26,
                value: PAWN_VALUE,
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
        white_locations: HashMap::from([(15, 2), (8, 26)]),
        black_locations: HashMap::from([(14, 6), (3, 25)]),
        white_vision: HashSet::from([1, 3, 9, 10, 11, 12]),
        black_vision: HashSet::from([7, 5, 15, 14, 13, 25, 27]),
        checked: KingChecked::None,
        gamemode: GameMode::MovementWhite,
        en_peasant_susceptible: Some(18),
        check: (KingChecked::None, None, None),
        chosen_piece: Some(25),
        dest_square: Some(33),
        engine_move: None,
        legal_moves: Vec::new(),
        move_history: Vec::new(),
    };
    let mut copy: Board = test_board.clone();
    copy.black_vision().unwrap();
    copy.white_vision().unwrap();

    test_board.perform_move(25, 33, PieceColor::Black).unwrap();
    test_board.cancel_move().unwrap();

    assert_eq!(copy.squares, test_board.squares);
    assert_eq!(copy.white_locations, test_board.white_locations);
    assert_eq!(copy.black_locations, test_board.black_locations);
    assert_eq!(copy.checked, test_board.checked);
    assert_eq!(copy.gamemode, test_board.gamemode);
    assert_eq!(
        copy.en_peasant_susceptible,
        test_board.en_peasant_susceptible
    );
    assert_eq!(copy.check, test_board.check);
    assert_eq!(copy.chosen_piece, test_board.chosen_piece);
    assert_eq!(copy.dest_square, test_board.dest_square);
}
#[test]
pub fn move_cancellation_test2() -> () {
    use crate::game_logic::{
        Board,
        pieces::{ChessPiece, King, Rook, Void},
        state_enums::{GameMode, KingChecked, PieceColor},
    };
    use std::collections::{HashMap, HashSet};
    let mut test_board: Board = Board {
        squares: [
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
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
            ChessPiece::K(King::new(PieceColor::White, 60, 15)),
            ChessPiece::Square(Void),
            ChessPiece::Square(Void),
            ChessPiece::R(Rook::new(PieceColor::White, 63, 20)),
        ],
        white_locations: HashMap::from([(15, 60), (20, 63)]),
        black_locations: HashMap::from([(14, 6)]),
        white_vision: HashSet::from([1, 3, 9, 10, 11, 12]),
        black_vision: HashSet::from([7, 5, 15, 14, 13, 25, 27]),
        checked: KingChecked::None,
        gamemode: GameMode::MovementWhite,
        en_peasant_susceptible: None,
        check: (KingChecked::None, None, None),
        chosen_piece: Some(60),
        dest_square: Some(62),
        engine_move: None,
        legal_moves: Vec::new(),
        move_history: Vec::new(),
    };
    let copy: Board = test_board.clone();
    test_board.perform_move(60, 62, PieceColor::White).unwrap();
    test_board.cancel_move().unwrap();

    assert_eq!(copy.squares, test_board.squares);
    assert_eq!(copy.white_locations, test_board.white_locations);
    assert_eq!(copy.black_locations, test_board.black_locations);
    assert_eq!(copy.checked, test_board.checked);
    assert_eq!(copy.gamemode, test_board.gamemode);
    assert_eq!(
        copy.en_peasant_susceptible,
        test_board.en_peasant_susceptible
    );
    assert_eq!(copy.check, test_board.check);
    assert_eq!(copy.chosen_piece, test_board.chosen_piece);
    assert_eq!(copy.dest_square, test_board.dest_square);
}
