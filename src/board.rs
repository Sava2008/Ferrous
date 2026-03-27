use crate::{board_geometry_templates::*, constants::masks::BIT_MASKS};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_rooks: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_rooks: u64,
    pub black_king: u64,

    pub white_occupancy: u64,
    pub black_occupancy: u64,
    pub total_occupancy: u64,
    pub cached_pieces: [Option<u32>; 64],

    pub white_king_square: u8,
    pub black_king_square: u8,
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
            white_occupancy: 0,
            black_occupancy: 0,
            total_occupancy: 0,
            cached_pieces: [None; 64],
            white_king_square: 4,
            black_king_square: 60,
        };
    }
    pub fn update_full_cache(&mut self) {
        for square in 0..64 {
            let mask: u64 = BIT_MASKS[square];
            if self.white_pawns & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_PAWN_U32);
            } else if self.white_knights & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_KNIGHT_U32);
            } else if self.white_bishops & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_BISHOP_U32);
            } else if self.white_rooks & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_ROOK_U32);
            } else if self.white_queens & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_QUEEN_U32);
            } else if self.white_king & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_KING_U32);
            } else if self.black_pawns & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_PAWN_U32);
            } else if self.black_knights & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_KNIGHT_U32);
            } else if self.black_bishops & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_BISHOP_U32);
            } else if self.black_rooks & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_ROOK_U32);
            } else if self.black_queens & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_QUEEN_U32);
            } else if self.black_king & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_KING_U32);
            }
        }
    }

    #[inline(always)]
    pub fn piece_at(&self, square: u32) -> Option<u32> {
        return unsafe { *self.cached_pieces.get_unchecked(square as usize) };
    }

    pub fn white_occupancy(&mut self) -> () {
        self.white_occupancy = self.white_bishops
            | self.white_king
            | self.white_knights
            | self.white_pawns
            | self.white_queens
            | self.white_rooks;
    }

    pub fn black_occupancy(&mut self) -> () {
        self.black_occupancy = self.black_bishops
            | self.black_king
            | self.black_knights
            | self.black_pawns
            | self.black_queens
            | self.black_rooks;
    }

    pub fn total_occupancy(&mut self) -> () {
        self.white_occupancy();
        self.black_occupancy();
        self.total_occupancy = self.white_occupancy | self.black_occupancy;
    }

    pub fn bitboard_to_indices(mut bitboard: u64) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        while bitboard != 0 {
            let index: usize = bitboard.trailing_zeros() as usize;
            indices.push(index);
            bitboard &= bitboard - 1;
        }
        return indices;
    }

    pub fn indices_to_bitboard(indices: &Vec<usize>) -> u64 {
        let mut bitboard: u64 = 0;
        for index in indices {
            if *index > 63 {
                panic!(
                    "can annotate a chess board only with indices from 0 (inclusive) to 63 (inclusive)"
                );
            }
            bitboard |= BIT_MASKS[*index];
        }
        return bitboard;
    }

    #[inline(always)]
    pub fn is_capture(&self, m: u32) -> bool {
        return captured_piece(m) != 0;
    }
}
