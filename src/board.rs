use crate::{
    board_geometry_templates::*,
    enums::{InclusiveRange, PieceColor, PieceType},
    gamestate::PieceMove,
};
use std::cmp::{max, min};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
pub struct Board {
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_rooks: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_rooks: Bitboard,
    pub black_king: Bitboard,

    pub white_occupancy: Option<Bitboard>,
    pub black_occupancy: Option<Bitboard>,
    pub total_occupancy: Option<Bitboard>,
}

impl Board {
    // default starting position
    pub fn set() -> Self {
        return Board {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            white_queens: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000010000,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            black_queens: 0b0000100000000000000000000000000000000000000000000000000000000000,
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
            white_occupancy: None,
            black_occupancy: None,
            total_occupancy: None,
        };
    }

    pub fn white_occupancy(&mut self) -> () {
        self.white_occupancy = Some(
            self.white_bishops
                | self.white_king
                | self.white_knights
                | self.white_pawns
                | self.white_queens
                | self.white_rooks,
        );
    }

    pub fn black_occupancy(&mut self) -> () {
        self.black_occupancy = Some(
            self.black_bishops
                | self.black_king
                | self.black_knights
                | self.black_pawns
                | self.black_queens
                | self.black_rooks,
        );
    }

    pub fn total_occupancy(&mut self) -> () {
        match (self.white_occupancy, self.black_occupancy) {
            (None, None) => {
                self.white_occupancy();
                self.black_occupancy();
            }
            (None, Some(_)) => {
                self.white_occupancy();
            }
            (Some(_), None) => {
                self.black_occupancy();
            }
            (Some(_), Some(_)) => (),
        };
        self.total_occupancy = Some(self.white_occupancy.unwrap() | self.black_occupancy.unwrap());
    }

    pub fn bitboard_to_indices(mut bitboard: Bitboard) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        while bitboard != 0 {
            let index: usize = bitboard.trailing_zeros() as usize;
            indices.push(index);
            bitboard &= bitboard - 1;
        }
        return indices;
    }

    pub fn indices_to_bitboard(indices: &Vec<usize>) -> Bitboard {
        let mut bitboard: Bitboard = 0;
        for index in indices {
            if *index > 63 {
                panic!(
                    "can annotate a chess board only with indices from 0 (inclusive) to 63 (inclusive)"
                );
            }
            bitboard |= 1 << index;
        }
        return bitboard;
    }

    #[inline]
    fn reset_bit(&mut self, bitboard: (PieceColor, PieceType), bit_position: u8) -> () {
        let mask: Bitboard = !(1 << bit_position);
        match bitboard {
            (PieceColor::White, PieceType::Bishop) => self.white_bishops &= mask,
            (PieceColor::White, PieceType::Knight) => self.white_knights &= mask,
            (PieceColor::White, PieceType::Pawn) => self.white_pawns &= mask,
            (PieceColor::White, PieceType::Rook) => self.white_rooks &= mask,
            (PieceColor::White, PieceType::Queen) => self.white_queens &= mask,
            (PieceColor::White, PieceType::King) => self.white_king &= mask,
            (PieceColor::Black, PieceType::Bishop) => self.black_bishops &= mask,
            (PieceColor::Black, PieceType::Knight) => self.black_knights &= mask,
            (PieceColor::Black, PieceType::Pawn) => self.black_pawns &= mask,
            (PieceColor::Black, PieceType::Rook) => self.black_rooks &= mask,
            (PieceColor::Black, PieceType::Queen) => self.black_queens &= mask,
            (PieceColor::Black, PieceType::King) => self.black_king &= mask,
        };
    }

    #[inline]
    fn bitboard_contains(&self, index: u8) -> Option<(PieceColor, PieceType)> {
        let mask: Bitboard = 1 << index;
        if self.total_occupancy.unwrap() & mask == 0 {
            return None;
        }

        if &self.white_pawns & mask != 0 {
            return Some((PieceColor::White, PieceType::Pawn));
        }
        if &self.white_knights & mask != 0 {
            return Some((PieceColor::White, PieceType::Knight));
        }
        if &self.white_bishops & mask != 0 {
            return Some((PieceColor::White, PieceType::Bishop));
        }
        if &self.white_rooks & mask != 0 {
            return Some((PieceColor::White, PieceType::Rook));
        }
        if &self.white_queens & mask != 0 {
            return Some((PieceColor::White, PieceType::Queen));
        }
        if &self.white_king & mask != 0 {
            return Some((PieceColor::White, PieceType::King));
        }

        if &self.black_pawns & mask != 0 {
            return Some((PieceColor::Black, PieceType::Pawn));
        }
        if &self.black_knights & mask != 0 {
            return Some((PieceColor::Black, PieceType::Knight));
        }
        if &self.black_bishops & mask != 0 {
            return Some((PieceColor::Black, PieceType::Bishop));
        }
        if &self.black_rooks & mask != 0 {
            return Some((PieceColor::Black, PieceType::Rook));
        }
        if &self.black_queens & mask != 0 {
            return Some((PieceColor::Black, PieceType::Queen));
        }
        if &self.black_king & mask != 0 {
            return Some((PieceColor::Black, PieceType::King));
        }

        return None;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(&mut self, from_to: PieceMove) -> () {
        self.reset_bit(self.bitboard_contains(from_to.from).unwrap(), from_to.to);
    }

    pub fn generate_range(square1: u8, square2: u8, inclusion: &InclusiveRange) -> Bitboard {
        let (mut lower_square, higher_square) = (min(square1, square2), max(square1, square2));
        let mut rng: Bitboard = 0;
        let sq1_sq2_range: u8 = higher_square - lower_square;
        let increment: u8 = match sq1_sq2_range {
            num if num % 9 == 0 => 9,
            num if num % 7 == 0 && ((1 << num) & FILE_A) == 0 => 7,
            num if num <= 7 => 1,
            num if num % 8 == 0 => 8,
            _ => panic!("no straight path between {square1} and {square2}"),
        };
        while lower_square < higher_square - increment {
            lower_square += increment;
            rng |= 1 << lower_square;
        }
        match inclusion {
            &InclusiveRange::Both => rng |= 1 << square1 | 1 << square2,
            &InclusiveRange::FirstOnly => rng |= 1 << square1,
            &InclusiveRange::LastOnly => rng |= 1 << square2,
            &InclusiveRange::None => (),
        }
        return rng;
    }
}
