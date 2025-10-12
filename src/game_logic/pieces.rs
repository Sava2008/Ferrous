use ggez::{GameError, GameResult};
use std::cmp::{max, min};

use crate::game_logic::{
    Board,
    state_enums::{DiagonalDirection, KingChecked, LinearDirection, PieceColor, PieceVariant},
};
use crate::{
    constants::{
        BISHOP_VALUE, DIAGONAL_DIRECTIONS, KING_DELTAS, KING_VALUE, KNIGHT_DELTAS, KNIGHT_VALUE,
        LINEAR_DIRECTIONS, PAWN_VALUE, QUUEN_VALUE, ROOK_VALUE,
    },
    helper_functions::{
        generate_legal_moves, i8_coords_to_index, index_to_coords, is_adjancent_file, is_diagonal,
    },
};

pub trait Piece {
    fn legal_moves(&self, board: &Board, en_peasant_target: Option<usize>) -> Vec<usize>;
    fn index(&self) -> usize;
    fn color(&self) -> PieceColor;
}

pub trait MoveLinearly {
    fn one_direction_moves_linear(
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
        direction: LinearDirection,
        vision: bool,
    ) -> Vec<usize> {
        let mut one_dir_moves: Vec<usize> = Vec::new();
        let mut square_index: usize = self_index;
        let mut enemy_piece_counter: u8 = 0;

        loop {
            if match direction {
                LinearDirection::RankRight => (square_index + 1) % 8 == 0,
                LinearDirection::RankLeft => square_index % 8 == 0,
                LinearDirection::FileUp => (0..=7).any(|x: usize| x == square_index),
                LinearDirection::FileDown => (56..=63).any(|x: usize| x == square_index),
            } {
                break;
            }
            match direction {
                LinearDirection::RankRight => square_index += 1,
                LinearDirection::RankLeft => square_index -= 1,
                LinearDirection::FileUp => square_index -= 8,
                LinearDirection::FileDown => square_index += 8,
            };
            match board.squares[square_index].color() {
                None => one_dir_moves.push(square_index),
                Some(x) => {
                    if x != self_color {
                        match vision {
                            true => {
                                if enemy_piece_counter < 1 && board.squares[square_index].is_king()
                                {
                                    one_dir_moves.push(square_index);
                                    enemy_piece_counter += 1;
                                } else {
                                    one_dir_moves.push(square_index);
                                    break;
                                }
                            }
                            false => {
                                one_dir_moves.push(square_index);
                                break;
                            }
                        };
                    } else {
                        match vision {
                            true => {
                                one_dir_moves.push(square_index);
                                break;
                            }
                            false => break,
                        }
                    }
                }
            }
        }
        return one_dir_moves;
    }

    fn generate_linear_moves(
        &self,
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
    ) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = Vec::new();
        for line in LINEAR_DIRECTIONS {
            legal_moves.extend(Self::one_direction_moves_linear(
                &board, self_color, self_index, line, false,
            ));
        }
        return legal_moves;
    }

    fn generate_linear_vision(
        &self,
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
    ) -> Vec<usize> {
        let mut vision: Vec<usize> = Vec::new();
        for line in LINEAR_DIRECTIONS {
            vision.extend(Self::one_direction_moves_linear(
                &board, self_color, self_index, line, true,
            ));
        }
        return vision;
    }
}

pub trait MoveDiagonally {
    fn one_direction_moves_diagonal(
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
        direction: DiagonalDirection,
        vision: bool,
    ) -> Vec<usize> {
        let mut one_dir_moves: Vec<usize> = Vec::new();
        let mut square_index: usize = self_index;
        let mut enemy_piece_counter: u8 = 0;

        loop {
            if match direction {
                DiagonalDirection::UpRight => {
                    (square_index + 1) % 8 == 0 || (0..=7).any(|x: usize| x == square_index)
                }
                DiagonalDirection::UpLeft => {
                    square_index % 8 == 0 || (0..=7).any(|x: usize| x == square_index)
                }
                DiagonalDirection::DownRight => {
                    (square_index + 1) % 8 == 0 || (56..=63).any(|x: usize| x == square_index)
                }
                DiagonalDirection::DownLeft => {
                    square_index % 8 == 0 || (56..=63).any(|x: usize| x == square_index)
                }
            } {
                break;
            }

            match direction {
                DiagonalDirection::UpRight => square_index -= 7,
                DiagonalDirection::UpLeft => square_index -= 9,
                DiagonalDirection::DownRight => square_index += 9,
                DiagonalDirection::DownLeft => square_index += 7,
            };
            match board.squares[square_index].color() {
                None => one_dir_moves.push(square_index),
                Some(x) => {
                    if x == self_color {
                        match vision {
                            true => {
                                one_dir_moves.push(square_index);
                                break;
                            }
                            false => break,
                        }
                    } else {
                        match vision {
                            true => {
                                if enemy_piece_counter < 1 && board.squares[square_index].is_king()
                                {
                                    one_dir_moves.push(square_index);
                                    enemy_piece_counter += 1;
                                } else {
                                    one_dir_moves.push(square_index);
                                    break;
                                }
                            }
                            false => {
                                one_dir_moves.push(square_index);
                                break;
                            }
                        };
                    }
                }
            };
        }
        return one_dir_moves;
    }
    fn generate_diagonal_moves(
        &self,
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
    ) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = Vec::new();

        for diagonal in DIAGONAL_DIRECTIONS {
            legal_moves.extend(Self::one_direction_moves_diagonal(
                &board, self_color, self_index, diagonal, false,
            ));
        }
        return legal_moves;
    }

    fn generate_diagonal_vision(
        &self,
        board: &Board,
        self_color: PieceColor,
        self_index: usize,
    ) -> Vec<usize> {
        let mut vision: Vec<usize> = Vec::new();

        for diagonal in DIAGONAL_DIRECTIONS {
            vision.extend(Self::one_direction_moves_diagonal(
                &board, self_color, self_index, diagonal, true,
            ));
        }
        return vision;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pawn {
    pub index: usize,
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
    pub id: u8,
    pub is_pinned: bool,
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
                if !self.was_moved {
                    let second_rank_idx: usize = match self.key.0 {
                        PieceColor::Black => self.index + 16,
                        PieceColor::White => self.index - 16,
                    };
                    if board.squares[second_rank_idx].is_void() {
                        legal_moves.push(second_rank_idx);
                    }
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
            if let Some(target) = en_peasant_target {
                let (left_diag, right_diag) = match self.key.0 {
                    PieceColor::White => (self.index - 9, self.index - 7),
                    PieceColor::Black => (self.index + 7, self.index + 9),
                };

                if target == left_diag || target == right_diag {
                    legal_moves.push(target);
                }
            }
        }
        return legal_moves;
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl Pawn {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return Pawn {
            index,
            value: PAWN_VALUE,
            key: (color, PieceVariant::P),
            was_moved: false,
            id,
            is_pinned: false,
        };
    }

    pub fn moved_two_squares(&self, previus_index: usize) -> bool {
        return max(self.index, previus_index) - min(self.index, previus_index) == 16;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Knight {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
    pub id: u8,
    pub is_pinned: bool,
}
impl Piece for Knight {
    fn legal_moves(&self, board: &Board, _en_peasant_target: Option<usize>) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = Vec::new();
        let self_coords: (u8, u8) = index_to_coords(self.index);
        for (x, y) in KNIGHT_DELTAS {
            let spot: (i8, i8) = (self_coords.0 as i8 + x, self_coords.1 as i8 + y);
            if spot.0 > 7 || spot.0 < 0 || spot.1 > 7 || spot.1 < 0 {
                continue;
            }
            let index: usize = i8_coords_to_index(spot);
            if board.squares[index].is_piece() && board.squares[index].color() == Some(self.key.0) {
                continue;
            }
            legal_moves.push(index);
        }
        return legal_moves;
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl Knight {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return Knight {
            value: KNIGHT_VALUE,
            key: (color, PieceVariant::N),
            index,
            id,
            is_pinned: false,
        };
    }
    fn vision(&self, _board: &Board) -> Vec<usize> {
        let mut vision: Vec<usize> = Vec::new();
        let self_coords: (u8, u8) = index_to_coords(self.index);
        for (x, y) in KNIGHT_DELTAS {
            let spot: (i8, i8) = (self_coords.0 as i8 + x, self_coords.1 as i8 + y);
            if spot.0 > 7 || spot.0 < 0 || spot.1 > 7 || spot.1 < 0 {
                continue;
            }
            let index: usize = i8_coords_to_index(spot);
            vision.push(index);
        }
        return vision;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bishop {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
    pub id: u8,
    pub is_pinned: bool,
}
impl MoveDiagonally for Bishop {}
impl Piece for Bishop {
    fn legal_moves(&self, board: &Board, _en_peasant_target: Option<usize>) -> Vec<usize> {
        let v = self.generate_diagonal_moves(&board, self.key.0, self.index);
        return v;
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl Bishop {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return Bishop {
            value: BISHOP_VALUE,
            key: (color, PieceVariant::B),
            index,
            id,
            is_pinned: false,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rook {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
    pub index: usize,
    pub id: u8,
    pub is_pinned: bool,
}
impl MoveLinearly for Rook {}
impl Piece for Rook {
    fn legal_moves(&self, board: &Board, _en_peasant_target: Option<usize>) -> Vec<usize> {
        let v = self.generate_linear_moves(&board, self.key.0, self.index);
        return v;
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl Rook {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return Rook {
            value: ROOK_VALUE,
            key: (color, PieceVariant::R),
            was_moved: false,
            index,
            id,
            is_pinned: false,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Queen {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub index: usize,
    pub id: u8,
    pub is_pinned: bool,
}
impl MoveLinearly for Queen {}
impl MoveDiagonally for Queen {}
impl Piece for Queen {
    fn legal_moves(&self, board: &Board, _en_peasant_target: Option<usize>) -> Vec<usize> {
        return self
            .generate_linear_moves(&board, self.key.0, self.index)
            .into_iter()
            .chain(self.generate_diagonal_moves(&board, self.key.0, self.index))
            .collect();
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl Queen {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return Queen {
            value: QUUEN_VALUE,
            key: (color, PieceVariant::Q),
            index,
            id,
            is_pinned: false,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct King {
    pub value: u16,
    pub key: (PieceColor, PieceVariant),
    pub was_moved: bool,
    pub index: usize,
    pub id: u8,
}
impl Piece for King {
    fn legal_moves(&self, board: &Board, _en_peasant_target: Option<usize>) -> Vec<usize> {
        let mut legal_moves: Vec<usize> = Vec::new();
        let self_coords: (u8, u8) = index_to_coords(self.index);
        for (x, y) in KING_DELTAS {
            let spot: (i8, i8) = (self_coords.0 as i8 + x, self_coords.1 as i8 + y);
            if spot.0 < 0 || spot.1 < 0 {
                continue;
            }
            let index: usize = i8_coords_to_index(spot);
            if index > 63 {
                continue;
            }

            if board.squares[index].is_piece() && board.squares[index].color() == Some(self.key.0) {
                continue;
            }
            if !is_adjancent_file(self.index, index) {
                continue;
            }
            if match self.key.0 {
                PieceColor::Black => !board.white_vision.contains(&index),
                PieceColor::White => !board.black_vision.contains(&index),
            } {
                legal_moves.push(index);
            }
            if !self.was_moved {
                for piece in match self.key.0 {
                    PieceColor::Black => &board.squares[0..=7],
                    PieceColor::White => &board.squares[56..=63],
                } {
                    if let ChessPiece::R(r) = piece {
                        if !r.was_moved
                            && board.checked == KingChecked::None
                            && r.key.0 == self.key.0
                        {
                            match r.index.cmp(&self.index) {
                                std::cmp::Ordering::Greater => {
                                    if board.squares[self.index + 1..r.index]
                                        .iter()
                                        .any(|s: &ChessPiece| s.is_piece())
                                        || [self.index + 1, self.index + 2].iter().any(
                                            |i: &usize| match self.key.0 {
                                                PieceColor::Black => board.white_vision.contains(i),
                                                PieceColor::White => board.black_vision.contains(i),
                                            },
                                        )
                                    {
                                        continue;
                                    }
                                    legal_moves.push(self.index + 2);
                                }
                                std::cmp::Ordering::Less => {
                                    if board.squares[r.index + 1..self.index]
                                        .iter()
                                        .any(|s: &ChessPiece| s.is_piece())
                                        || [self.index - 1, self.index - 2].iter().any(
                                            |i: &usize| match self.key.0 {
                                                PieceColor::Black => board.white_vision.contains(i),
                                                PieceColor::White => board.black_vision.contains(i),
                                            },
                                        )
                                    {
                                        continue;
                                    }
                                    legal_moves.push(self.index - 2);
                                }
                                std::cmp::Ordering::Equal => unreachable!(),
                            };
                        }
                    }
                }
            }
        }
        return legal_moves;
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn color(&self) -> PieceColor {
        return self.key.0;
    }
}
impl King {
    pub fn new(color: PieceColor, index: usize, id: u8) -> Self {
        return King {
            value: KING_VALUE,
            key: (color, PieceVariant::K),
            was_moved: false,
            index,
            id,
        };
    }

    pub fn king_vision(&self) -> Vec<usize> {
        let mut vision: Vec<usize> = Vec::new();
        let self_coords: (u8, u8) = index_to_coords(self.index);
        for (x, y) in KING_DELTAS {
            let spot: (i8, i8) = (self_coords.0 as i8 + x, self_coords.1 as i8 + y);
            if spot.0 < 0 || spot.1 < 0 {
                continue;
            }
            let index: usize = i8_coords_to_index(spot);
            if index > 63 {
                continue;
            }
            vision.push(index);
        }
        return vision;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Void;

#[derive(Debug, PartialEq, Eq)]
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

    pub fn index(&self) -> GameResult<usize> {
        return match self {
            ChessPiece::Square(_) => Err(GameError::CustomError(
                "no index for emtpy square".to_string(),
            )),
            ChessPiece::B(b) => Ok(b.index),
            ChessPiece::K(k) => Ok(k.index),
            ChessPiece::N(n) => Ok(n.index),
            ChessPiece::P(p) => Ok(p.index),
            ChessPiece::Q(q) => Ok(q.index),
            ChessPiece::R(r) => Ok(r.index),
        };
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

    pub fn is_king(&self) -> bool {
        return match self {
            ChessPiece::K(_) => true,
            _ => false,
        };
    }

    pub fn is_diagonal_attacker(&self, color: PieceColor) -> bool {
        return match (
            self,
            if let Some(c) = self.color() {
                c != color
            } else {
                return false;
            },
        ) {
            (ChessPiece::Q(_) | ChessPiece::B(_), true) => true,
            _ => false,
        };
    }

    pub fn is_linear_attacker(&self, color: PieceColor) -> bool {
        return match (
            self,
            if let Some(c) = self.color() {
                c != color
            } else {
                return false;
            },
        ) {
            (ChessPiece::Q(_) | ChessPiece::R(_), true) => true,
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

    pub fn legal_moves(
        &self,
        board: &Board,
        en_peasant_target: Option<usize>,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        checked_king_idx: usize,
    ) -> GameResult<Vec<usize>> {
        return match self {
            ChessPiece::P(p) => {
                return generate_legal_moves(
                    p,
                    &board,
                    checked_king_idx,
                    checked,
                    en_peasant_target,
                );
            }
            ChessPiece::N(n) => {
                return generate_legal_moves(
                    n,
                    &board,
                    checked_king_idx,
                    checked,
                    en_peasant_target,
                );
            }
            ChessPiece::B(b) => {
                return generate_legal_moves(
                    b,
                    &board,
                    checked_king_idx,
                    checked,
                    en_peasant_target,
                );
            }
            ChessPiece::Q(q) => {
                return generate_legal_moves(
                    q,
                    &board,
                    checked_king_idx,
                    checked,
                    en_peasant_target,
                );
            }
            ChessPiece::R(r) => {
                return generate_legal_moves(
                    r,
                    &board,
                    checked_king_idx,
                    checked,
                    en_peasant_target,
                );
            }
            ChessPiece::K(k) => Ok(k.legal_moves(&board, en_peasant_target)),
            ChessPiece::Square(_) => Err(GameError::CustomError(
                "no legal moves for an empty square".to_string(),
            )),
        };
    }

    pub fn id(&self) -> GameResult<u8> {
        return match self {
            ChessPiece::Square(_) => Err(GameError::CustomError("no id for void".to_string())),
            ChessPiece::B(b) => Ok(b.id),
            ChessPiece::K(k) => Ok(k.id),
            ChessPiece::N(n) => Ok(n.id),
            ChessPiece::P(p) => Ok(p.id),
            ChessPiece::Q(q) => Ok(q.id),
            ChessPiece::R(r) => Ok(r.id),
        };
    }

    pub fn pin(&mut self, pinned: bool) -> () {
        match self {
            ChessPiece::Square(_) => (),
            ChessPiece::K(_) => (),
            ChessPiece::B(b) => b.is_pinned = pinned,
            ChessPiece::N(n) => n.is_pinned = pinned,
            ChessPiece::Q(q) => q.is_pinned = pinned,
            ChessPiece::R(r) => r.is_pinned = pinned,
            ChessPiece::P(p) => p.is_pinned = pinned,
        };
    }

    pub fn generate_vision(&self, board: &Board) -> Option<Vec<usize>> {
        return match self {
            ChessPiece::Square(_) => None,
            ChessPiece::B(b) => Some(b.generate_diagonal_vision(&board, b.key.0, b.index)),
            ChessPiece::K(k) => Some(k.king_vision()),
            ChessPiece::N(n) => Some(n.vision(&board)),
            ChessPiece::P(p) => {
                let mut p_vision: Vec<usize> = Vec::new();
                if match (p.key.0, p.index) {
                    (PieceColor::Black, 56..=63) | (PieceColor::White, 0..=7) => true,
                    _ => false,
                } {
                    return Some(p_vision);
                };
                let diag1: usize = match p.key.0 {
                    PieceColor::White => p.index - 9,
                    PieceColor::Black => p.index + 9,
                };

                let diag2: usize = match p.key.0 {
                    PieceColor::White => p.index - 7,
                    PieceColor::Black => p.index + 7,
                };
                if is_diagonal(p.index, diag1) {
                    p_vision.push(diag1);
                }
                if is_diagonal(p.index, diag2) {
                    p_vision.push(diag2);
                }
                return Some(p_vision);
            }
            ChessPiece::Q(q) => {
                let mut q_vision: Vec<usize> = q.generate_diagonal_vision(&board, q.key.0, q.index);
                q_vision.extend(q.generate_linear_vision(&board, q.key.0, q.index));
                return Some(q_vision);
            }
            ChessPiece::R(r) => Some(r.generate_linear_vision(&board, r.key.0, r.index)),
        };
    }
}
