use crate::{board_geometry_templates::*, constants::masks::BIT_MASKS, gamestate::GameState};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub bitboards: [u64; 12], // white pawn, white knight, ..., black pawn, black knight, ..., black king (idx 11)

    pub occupancies: [u64; 2], // [white_occ, black_occ]
    pub total_occupancy: u64,
    pub cached_pieces: [u16; 64],

    pub white_king_square: u8,
    pub black_king_square: u8,
}

impl Board {
    // default starting position
    pub fn set() -> Self {
        return Board {
            bitboards: [
                0b0000000000000000000000000000000000000000000000001111111100000000,
                0b0000000000000000000000000000000000000000000000000000000001000010,
                0b0000000000000000000000000000000000000000000000000000000000100100,
                0b0000000000000000000000000000000000000000000000000000000010000001,
                0b0000000000000000000000000000000000000000000000000000000000001000,
                0b0000000000000000000000000000000000000000000000000000000000010000,
                0b0000000011111111000000000000000000000000000000000000000000000000,
                0b0100001000000000000000000000000000000000000000000000000000000000,
                0b0010010000000000000000000000000000000000000000000000000000000000,
                0b1000000100000000000000000000000000000000000000000000000000000000,
                0b0000100000000000000000000000000000000000000000000000000000000000,
                0b0001000000000000000000000000000000000000000000000000000000000000,
            ],
            occupancies: [0, 0],
            total_occupancy: 0,
            cached_pieces: [0; 64],
            white_king_square: 4,
            black_king_square: 60,
        };
    }
    pub fn update_full_cache(&mut self) {
        for square in 0..64 {
            let mask: u64 = BIT_MASKS[square];
            if self.bitboards[0] & mask != 0 {
                self.cached_pieces[square] = WHITE_PAWN_U16;
            } else if self.bitboards[1] & mask != 0 {
                self.cached_pieces[square] = WHITE_KNIGHT_U16;
            } else if self.bitboards[2] & mask != 0 {
                self.cached_pieces[square] = WHITE_BISHOP_U16;
            } else if self.bitboards[3] & mask != 0 {
                self.cached_pieces[square] = WHITE_ROOK_U16;
            } else if self.bitboards[4] & mask != 0 {
                self.cached_pieces[square] = WHITE_QUEEN_U16;
            } else if self.bitboards[5] & mask != 0 {
                self.cached_pieces[square] = WHITE_KING_U16;
            } else if self.bitboards[6] & mask != 0 {
                self.cached_pieces[square] = BLACK_PAWN_U16;
            } else if self.bitboards[7] & mask != 0 {
                self.cached_pieces[square] = BLACK_KNIGHT_U16;
            } else if self.bitboards[8] & mask != 0 {
                self.cached_pieces[square] = BLACK_BISHOP_U16;
            } else if self.bitboards[9] & mask != 0 {
                self.cached_pieces[square] = BLACK_ROOK_U16;
            } else if self.bitboards[10] & mask != 0 {
                self.cached_pieces[square] = BLACK_QUEEN_U16;
            } else if self.bitboards[11] & mask != 0 {
                self.cached_pieces[square] = BLACK_KING_U16;
            }
        }
    }

    #[inline(always)]
    pub fn piece_at(&self, square: u16) -> u16 {
        return unsafe { *self.cached_pieces.get_unchecked(square as usize) };
    }

    pub fn white_occupancy(&mut self) -> () {
        for i in 0..6 {
            self.occupancies[0] |= self.bitboards[i];
        }
    }

    pub fn black_occupancy(&mut self) -> () {
        for i in 6..12 {
            self.occupancies[1] |= self.bitboards[i];
        }
    }

    pub fn total_occupancy(&mut self) -> () {
        self.white_occupancy();
        self.black_occupancy();
        self.total_occupancy = self.occupancies[0] | self.occupancies[1];
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
    pub fn is_capture(&self, m: u16) -> bool {
        return self.piece_at(to_square(m)) != 0;
    }

    pub fn calculate_check_restrictions(&self, _state: &mut GameState, _for_color: u16) -> () {
        return;
    }
}
