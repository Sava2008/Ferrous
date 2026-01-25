use std::ops::Not;

#[derive(Debug, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

impl Not for PieceColor {
    type Output = Self;
    fn not(self) -> Self::Output {
        return match self {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        };
    }
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// handles how the range is handled e.g. 1..10
#[derive(Debug)]
pub enum InclusiveRange {
    Both,      // 1 inclusive, 10 inclusive
    None,      // 1 exclusive, 10 exclusive
    FirstOnly, // 1 inclusive, 10 exclusive
    LastOnly,  // 1 exclusive, 10 inclusive
}

#[derive(Debug)]
pub enum GameResult {
    Going,
    WhiteWins,
    BlackWins,
    Draw,
}
