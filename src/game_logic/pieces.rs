use std::cmp::{max, min};

use crate::game_logic::{
    Board,
    state_enums::{PieceColor, PieceVariant},
};
use crate::{
    constants::{BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUUEN_VALUE, ROOK_VALUE},
    helper_functions::is_diagonal,
};

pub trait Piece {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize>;
}

#[derive(Debug)]
pub struct Pawn {
    pub index: usize,
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
}
impl Piece for Pawn {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = Vec::new();

        let idx_in_front: Option<usize> = match self.key.0 {
            PieceColor::White if self.index >= 8 => Some(self.index - 8),
            PieceColor::Black if self.index <= 55 => Some(self.index + 8),
            _ => None,
        };

        if let Some(x) = idx_in_front {
            if board.squares[x].is_void() {
                legal_moves.push(x);
                let mut second_rank: std::ops::RangeInclusive<usize> = match self.key.0 {
                    PieceColor::White => (48..=55).into_iter(),
                    PieceColor::Black => (8..=15).into_iter(),
                };
                if !self.was_moved && second_rank.any(|idx: usize| idx == self.index) {
                    legal_moves.push(match self.key.0 {
                        PieceColor::White => self.index - 16,
                        PieceColor::Black => self.index + 16,
                    });
                }
            }
            for i in [x + 1, x - 1] {
                if board.squares[i].is_piece()
                    && board.squares[i].color()
                        == match self.key.0 {
                            PieceColor::White => Some(PieceColor::Black),
                            PieceColor::Black => Some(PieceColor::White),
                        }
                    && is_diagonal(self.index, i)
                {
                    legal_moves.push(i);
                }
            }
            if let Some(target) = en_peasant_target
                && match self.key.0 {
                    PieceColor::Black => (40..=47).into_iter(),
                    PieceColor::White => (24..=31).into_iter(),
                }
                .any(|x: usize| x == self.index)
            {
                let (left_diag, right_diag) = match self.key.0 {
                    PieceColor::White => (self.index - 9, self.index - 7),
                    PieceColor::Black => (self.index + 7, self.index + 9),
                };

                if target == left_diag || target == right_diag {
                    legal_moves.push(target);
                }
            }
        }
        println!("legal_moves = {legal_moves:?}");
        return legal_moves;
    }
}
impl Pawn {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return Pawn {
            index,
            value: PAWN_VALUE,
            key: (color, PieceVariant::P),
            was_moved: false,
        };
    }

    pub fn moved_two_squares(&self, previus_index: usize) -> bool {
        return max(self.index, previus_index) - min(self.index, previus_index) == 16;
    }
}

#[derive(Debug)]
pub struct Knight {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
}
impl Piece for Knight {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        todo!();
    }
}
impl Knight {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return Knight {
            value: KNIGHT_VALUE,
            key: (color, PieceVariant::N),
            index,
        };
    }
}

#[derive(Debug)]
pub struct Bishop {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
}
impl Piece for Bishop {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        todo!();
    }
}
impl Bishop {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return Bishop {
            value: BISHOP_VALUE,
            key: (color, PieceVariant::B),
            index,
        };
    }
}

#[derive(Debug)]
pub struct Rook {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
    pub index: usize,
}
impl Piece for Rook {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        todo!();
    }
}
impl Rook {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return Rook {
            value: ROOK_VALUE,
            key: (color, PieceVariant::R),
            was_moved: false,
            index,
        };
    }
}

#[derive(Debug)]
pub struct Queen {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
}
impl Piece for Queen {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        todo!();
    }
}
impl Queen {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return Queen {
            value: QUUEN_VALUE,
            key: (color, PieceVariant::Q),
            index,
        };
    }
}

#[derive(Debug)]
pub struct King {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
    pub index: usize,
}
impl Piece for King {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize> {
        todo!();
    }
}
impl King {
    pub fn new(color: PieceColor, index: usize) -> Self {
        return King {
            value: KING_VALUE,
            key: (color, PieceVariant::K),
            was_moved: false,
            index,
        };
    }
}

#[derive(Debug)]
pub struct Void;

#[derive(Debug)]
pub enum ChessPiece {
    P(Pawn),
    N(Knight),
    B(Bishop),
    R(Rook),
    Q(Queen),
    K(King),
    Square(Void),
}
impl ChessPiece {
    pub fn is_piece(&self) -> bool {
        return match self {
            ChessPiece::Square(_) => false,
            _ => true,
        };
    }

    pub fn is_void(&self) -> bool {
        return !self.is_piece();
    }

    pub fn color(&self) -> Option<PieceColor> {
        return match self {
            ChessPiece::Square(_) => None,
            ChessPiece::B(b) => Some(b.key.0),
            ChessPiece::K(k) => Some(k.key.0),
            ChessPiece::N(n) => Some(n.key.0),
            ChessPiece::R(r) => Some(r.key.0),
            ChessPiece::Q(q) => Some(q.key.0),
            ChessPiece::P(p) => Some(p.key.0),
        };
    }

    pub fn key<'a>(&self) -> Result<(PieceColor, PieceVariant), &'a str> {
        return match self {
            ChessPiece::B(b) => Ok(b.key),
            ChessPiece::K(k) => Ok(k.key),
            ChessPiece::N(n) => Ok(n.key),
            ChessPiece::P(p) => Ok(p.key),
            ChessPiece::Q(q) => Ok(q.key),
            ChessPiece::R(r) => Ok(r.key),
            ChessPiece::Square(_) => Err("Can't get key of nothing"),
        };
    }

    pub fn value<'a>(&self) -> Result<u16, &'a str> {
        return match self {
            ChessPiece::B(b) => Ok(b.value),
            ChessPiece::K(k) => Ok(k.value),
            ChessPiece::N(n) => Ok(n.value),
            ChessPiece::P(p) => Ok(p.value),
            ChessPiece::Q(q) => Ok(q.value),
            ChessPiece::R(r) => Ok(r.value),
            ChessPiece::Square(_) => Err("Can't get value of nothing"),
        };
    }

    pub fn is_white(&self) -> bool {
        return match self {
            ChessPiece::B(b) => b.key.0 == PieceColor::White,
            ChessPiece::K(k) => k.key.0 == PieceColor::White,
            ChessPiece::N(n) => n.key.0 == PieceColor::White,
            ChessPiece::P(p) => p.key.0 == PieceColor::White,
            ChessPiece::Q(q) => q.key.0 == PieceColor::White,
            ChessPiece::R(r) => r.key.0 == PieceColor::White,
            ChessPiece::Square(_) => false,
        };
    }

    pub fn is_pawn(&self) -> bool {
        return match self {
            ChessPiece::P(_) => true,
            _ => false,
        };
    }

    pub fn new_idx(&mut self, new_idx: usize) -> () {
        match self {
            ChessPiece::B(b) => b.index = new_idx,
            ChessPiece::K(k) => k.index = new_idx,
            ChessPiece::N(n) => n.index = new_idx,
            ChessPiece::P(p) => p.index = new_idx,
            ChessPiece::Q(q) => q.index = new_idx,
            ChessPiece::R(r) => r.index = new_idx,
            ChessPiece::Square(_) => (),
        };
    }

    pub fn legal_moves(&self, board: &Board) -> Vec<(u8, u8)> {
        todo!();
    }
}
