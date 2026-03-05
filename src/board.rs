use std::hint::unreachable_unchecked;

use crate::{
    board_geometry_templates::*,
    constants::{
        attacks::RAYS_BETWEEN,
        piece_values::{BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE},
    },
    enums::InclusiveRange,
    gamestate::{GameState, PreviousMove},
};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
#[derive(Clone, Debug, PartialEq)]
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

    pub white_occupancy: Bitboard,
    pub black_occupancy: Bitboard,
    pub total_occupancy: Bitboard,

    pub white_king_square: u8,
    pub black_king_square: u8,

    pub cached_pieces: [Option<u8>; 64],

    pub material: i32,
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
            white_king_square: 4,
            black_king_square: 60,
            cached_pieces: [None; 64],
            material: 0, // equal material
        };
    }
    pub fn count_material(&mut self) -> () {
        self.material = (self.white_pawns.count_ones() as i32) * PAWN_VALUE
            + (self.white_knights.count_ones() as i32) * KNIGHT_VALUE
            + (self.white_bishops.count_ones() as i32) * BISHOP_VALUE
            + (self.white_rooks.count_ones() as i32) * ROOK_VALUE
            + (self.white_queens.count_ones() as i32) * QUEEN_VALUE
            - (self.black_pawns.count_ones() as i32) * PAWN_VALUE
            - (self.black_knights.count_ones() as i32) * KNIGHT_VALUE
            - (self.black_bishops.count_ones() as i32) * BISHOP_VALUE
            - (self.black_rooks.count_ones() as i32) * ROOK_VALUE
            - (self.black_queens.count_ones() as i32) * QUEEN_VALUE;
    }
    pub fn update_full_cache(&mut self) {
        for square in 0..64 {
            let mask: Bitboard = 1 << square;
            if self.white_pawns & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_PAWN_U8);
            } else if self.white_knights & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_KNIGHT_U8);
            } else if self.white_bishops & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_BISHOP_U8);
            } else if self.white_rooks & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_ROOK_U8);
            } else if self.white_queens & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_QUEEN_U8);
            } else if self.white_king & mask != 0 {
                self.cached_pieces[square] = Some(WHITE_KING_U8);
            } else if self.black_pawns & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_PAWN_U8);
            } else if self.black_knights & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_KNIGHT_U8);
            } else if self.black_bishops & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_BISHOP_U8);
            } else if self.black_rooks & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_ROOK_U8);
            } else if self.black_queens & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_QUEEN_U8);
            } else if self.black_king & mask != 0 {
                self.cached_pieces[square] = Some(BLACK_KING_U8);
            }
        }
    }

    #[inline(always)]
    pub fn piece_at(&self, square: &u16) -> Option<u8> {
        return unsafe { *self.cached_pieces.get_unchecked(*square as usize) }; // square always in range 0-63
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
    fn reset_bit(&mut self, piece: u8, bit_position1: u8, bit_position2: u8) -> () {
        let from_mask: Bitboard = !(1 << bit_position1);
        let to_mask: Bitboard = 1 << bit_position2;
        let bitboard_to_mutate: &mut Bitboard = match piece {
            WHITE_BISHOP_U8 => &mut self.white_bishops,
            WHITE_KNIGHT_U8 => &mut self.white_knights,
            WHITE_PAWN_U8 => &mut self.white_pawns,
            WHITE_ROOK_U8 => &mut self.white_rooks,
            WHITE_QUEEN_U8 => &mut self.white_queens,
            WHITE_KING_U8 => &mut self.white_king,
            BLACK_BISHOP_U8 => &mut self.black_bishops,
            BLACK_KNIGHT_U8 => &mut self.black_knights,
            BLACK_PAWN_U8 => &mut self.black_pawns,
            BLACK_ROOK_U8 => &mut self.black_rooks,
            BLACK_QUEEN_U8 => &mut self.black_queens,
            BLACK_KING_U8 => &mut self.black_king,
            _ => unreachable!(),
        };

        *bitboard_to_mutate &= from_mask;
        *bitboard_to_mutate |= to_mask;
    }

    fn perform_capture(
        &mut self,
        state: &mut GameState,
        enemy: u8,
        previous_move: &mut PreviousMove,
        to_sq: u8,
    ) -> () {
        let (bitboard_for_capture, occupancy): (&mut Bitboard, &mut Bitboard) = match enemy {
            WHITE_BISHOP_U8 => {
                previous_move.material_difference -= BISHOP_VALUE;
                self.material -= BISHOP_VALUE;
                (&mut self.white_bishops, &mut self.white_occupancy)
            }
            WHITE_KNIGHT_U8 => {
                previous_move.material_difference -= KNIGHT_VALUE;
                self.material -= KNIGHT_VALUE;
                (&mut self.white_knights, &mut self.white_occupancy)
            }
            WHITE_PAWN_U8 => {
                previous_move.material_difference -= PAWN_VALUE;
                self.material -= PAWN_VALUE;
                (&mut self.white_pawns, &mut self.white_occupancy)
            }
            WHITE_QUEEN_U8 => {
                previous_move.material_difference -= QUEEN_VALUE;
                self.material -= QUEEN_VALUE;
                (&mut self.white_queens, &mut self.white_occupancy)
            }
            WHITE_ROOK_U8 => {
                previous_move.material_difference -= ROOK_VALUE;
                self.material -= ROOK_VALUE;
                if to_sq == 7 {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                    state.castling_rights.white_two_zeros = false;
                } else if to_sq == 0 {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                    state.castling_rights.white_three_zeros = false;
                }
                (&mut self.white_rooks, &mut self.white_occupancy)
            }
            WHITE_KING_U8 => {
                panic!("attemped to capture white king. state: {state:?}, board: {self:?}")
            }
            BLACK_BISHOP_U8 => {
                previous_move.material_difference += BISHOP_VALUE;
                self.material += BISHOP_VALUE;
                (&mut self.black_bishops, &mut self.black_occupancy)
            }
            BLACK_KNIGHT_U8 => {
                previous_move.material_difference += KNIGHT_VALUE;
                self.material += KNIGHT_VALUE;
                (&mut self.black_knights, &mut self.black_occupancy)
            }
            BLACK_PAWN_U8 => {
                previous_move.material_difference += PAWN_VALUE;
                self.material += PAWN_VALUE;
                (&mut self.black_pawns, &mut self.black_occupancy)
            }
            BLACK_QUEEN_U8 => {
                previous_move.material_difference += QUEEN_VALUE;
                self.material += QUEEN_VALUE;
                (&mut self.black_queens, &mut self.black_occupancy)
            }
            BLACK_ROOK_U8 => {
                previous_move.material_difference += ROOK_VALUE;
                self.material += ROOK_VALUE;
                if to_sq == 63 {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                    state.castling_rights.black_two_zeros = false;
                } else if to_sq == 56 {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());

                    state.castling_rights.black_three_zeros = false;
                }
                (&mut self.black_rooks, &mut self.black_occupancy)
            }
            BLACK_KING_U8 => {
                panic!("attemped to capture black king. state: {state:?}, board: {self:?}")
            }
            _ => unreachable!(),
        };
        previous_move.changed_cache_indices[1] = (Some((to_sq, to_sq)), Some(enemy));
        let capture: Bitboard = !(1 << to_sq);
        *occupancy &= capture;
        *bitboard_for_capture &= capture;
    }

    fn castling(
        &mut self,
        previous_move: &mut PreviousMove,
        from_sq: u8,
        to_sq: u8,
        color: &u8,
    ) -> () {
        let rook: u8 = COLORLESS_ROOK | *color;
        previous_move.changed_cache_indices[1] = (Some((from_sq as u8, to_sq as u8)), Some(rook));
        self.cached_pieces[from_sq as usize] = None;
        self.cached_pieces[to_sq as usize] = Some(rook);
        self.reset_bit(rook, from_sq, to_sq);
        let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
        let (start, end): (Bitboard, Bitboard) = (!(1 << from_sq), 1 << to_sq);
        *total_occupancy &= start;
        *total_occupancy |= end;
        let occupancy: &mut Bitboard = match color {
            &8 => &mut self.white_occupancy,
            &16 => &mut self.black_occupancy,
            _ => unreachable!(),
        };
        *occupancy &= start;
        *occupancy |= end;
    }

    fn en_passant(&mut self, e_p: u8, previous_move: &mut PreviousMove, color: &u8) -> () {
        let (pawns, occupancy, captured_pawn_square) = match color {
            &8 => (&mut self.white_pawns, &mut self.white_occupancy, e_p + 8),
            &16 => (&mut self.black_pawns, &mut self.black_occupancy, e_p - 8),
            _ => unreachable!(),
        };
        previous_move.changed_cache_indices[1] = (
            Some((captured_pawn_square, captured_pawn_square)),
            Some(COLORLESS_PAWN | *color),
        );
        self.cached_pieces[captured_pawn_square as usize] = None;
        let capture: Bitboard = !(1 << captured_pawn_square);
        *pawns &= capture;
        *occupancy &= capture;
        *&mut self.total_occupancy &= capture;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(&mut self, from_to: &u16, state: &mut GameState) -> () {
        let (from_sq, to_sq): (u8, u8) = (
            (from_to & FROM_MASK) as u8,
            ((from_to & TO_MASK) >> TO_SHIFT) as u8,
        );

        let moving_piece: u8 = unsafe { self.piece_at(&(from_sq as u16)).unwrap_unchecked() }; // a piece on the starting square must be present 
        let captured_piece: Option<u8> = self.piece_at(&(to_sq as u16));
        let mut promotion_choice: Option<u8> = None;
        let mut previous_move: PreviousMove = PreviousMove {
            changed_cache_indices: [(None, None); 3],
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: None,
            material_difference: 0,
            promotion_happened: false,
        };
        if let Some(enemy) = captured_piece {
            self.perform_capture(state, enemy, &mut previous_move, to_sq);
        }

        let color_to_mutate: &mut u64 = match moving_piece {
            WHITE_KING_U8 => {
                previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                self.white_king_square = to_sq;
                match (from_sq, to_sq) {
                    (4, 2) => {
                        let (rook_from, rook_to) = (0, 3);
                        self.castling(&mut previous_move, rook_from, rook_to, &NO_PIECE_WHITE);
                    }
                    (4, 6) => {
                        let (rook_from, rook_to) = (7, 5);
                        self.castling(&mut previous_move, rook_from, rook_to, &NO_PIECE_WHITE);
                    }
                    _ => (),
                };

                (
                    state.castling_rights.white_three_zeros,
                    state.castling_rights.white_two_zeros,
                ) = (false, false);
                state.en_passant_target = None;
                &mut self.white_occupancy
            }
            WHITE_ROOK_U8 => {
                if (to_sq != 56 && to_sq != 63)
                    && self.piece_at(&(to_sq as u16)) != Some(BLACK_ROOK_U8)
                {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                }

                if from_sq == 0 {
                    state.castling_rights.white_three_zeros = false;
                } else if from_sq == 7 {
                    state.castling_rights.white_two_zeros = false;
                }
                state.en_passant_target = None;
                &mut self.white_occupancy
            }
            WHITE_PAWN_U8 => {
                match (from_sq, to_sq) {
                    (8..=15, 24..=31) => {
                        state.en_passant_target = Some(to_sq - 8); // en passant square behind the pawn
                    }
                    (48..56, 56..64) => {
                        promotion_choice = match (from_to & PROMOTION_MASK) >> PROMOTION_SHIFT {
                            1 => {
                                self.material += KNIGHT_VALUE;
                                previous_move.material_difference += KNIGHT_VALUE;
                                Some(WHITE_KNIGHT_U8)
                            }
                            2 => {
                                self.material += BISHOP_VALUE;
                                previous_move.material_difference += BISHOP_VALUE;
                                Some(WHITE_BISHOP_U8)
                            }
                            3 => {
                                self.material += ROOK_VALUE;
                                previous_move.material_difference += ROOK_VALUE;
                                Some(WHITE_ROOK_U8)
                            }
                            4 => {
                                self.material += QUEEN_VALUE;
                                previous_move.material_difference += QUEEN_VALUE;
                                Some(WHITE_QUEEN_U8)
                            }
                            _ => unsafe { unreachable_unchecked() },
                        };
                        state.en_passant_target = None;
                    }
                    _ => {
                        if let Some(e_p) = state.en_passant_target {
                            if to_sq == e_p {
                                self.en_passant(e_p, &mut previous_move, &NO_PIECE_BLACK);
                                previous_move.material_difference += PAWN_VALUE;
                                self.material += PAWN_VALUE;
                            }
                        }
                        state.en_passant_target = None;
                    }
                };
                &mut self.white_occupancy
            }
            WHITE_BISHOP_U8 | WHITE_KNIGHT_U8 | WHITE_QUEEN_U8 => {
                state.en_passant_target = None;
                &mut self.white_occupancy
            }

            BLACK_KING_U8 => {
                previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                self.black_king_square = to_sq;
                match (from_sq, to_sq) {
                    (60, 58) => {
                        let (rook_from, rook_to) = (56, 59);
                        self.castling(&mut previous_move, rook_from, rook_to, &NO_PIECE_BLACK);
                    }
                    (60, 62) => {
                        let (rook_from, rook_to) = (63, 61);
                        self.castling(&mut previous_move, rook_from, rook_to, &NO_PIECE_BLACK);
                    }
                    _ => (),
                };

                (
                    state.castling_rights.black_three_zeros,
                    state.castling_rights.black_two_zeros,
                ) = (false, false);
                state.en_passant_target = None;
                &mut self.black_occupancy
            }
            BLACK_ROOK_U8 => {
                if (to_sq != 0 && to_sq != 7)
                    && self.piece_at(&(to_sq as u16)) != Some(WHITE_ROOK_U8)
                {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                }
                if from_sq == 56 {
                    state.castling_rights.black_three_zeros = false;
                } else if from_sq == 63 {
                    state.castling_rights.black_two_zeros = false;
                }
                state.en_passant_target = None;
                &mut self.black_occupancy
            }
            BLACK_PAWN_U8 => {
                match (from_sq, to_sq) {
                    (48..=55, 32..=39) => {
                        state.en_passant_target = Some(to_sq + 8); // en passant square behind the pawn
                    }
                    (8..16, 0..8) => {
                        promotion_choice = match (from_to & PROMOTION_MASK) >> PROMOTION_SHIFT {
                            1 => {
                                self.material -= KNIGHT_VALUE;
                                previous_move.material_difference -= KNIGHT_VALUE;
                                Some(BLACK_KNIGHT_U8)
                            }
                            2 => {
                                self.material -= BISHOP_VALUE;
                                previous_move.material_difference -= BISHOP_VALUE;
                                Some(BLACK_BISHOP_U8)
                            }
                            3 => {
                                self.material -= ROOK_VALUE;
                                previous_move.material_difference -= ROOK_VALUE;
                                Some(BLACK_ROOK_U8)
                            }
                            4 => {
                                self.material -= QUEEN_VALUE;
                                previous_move.material_difference -= QUEEN_VALUE;
                                Some(BLACK_QUEEN_U8)
                            }
                            _ => unsafe { unreachable_unchecked() },
                        };
                        state.en_passant_target = None;
                    }
                    _ => {
                        if let Some(e_p) = state.en_passant_target {
                            if to_sq == e_p {
                                self.en_passant(e_p, &mut previous_move, &NO_PIECE_WHITE);
                                previous_move.material_difference -= PAWN_VALUE;
                                self.material -= PAWN_VALUE;
                            }
                        }
                        state.en_passant_target = None;
                    }
                };
                &mut self.black_occupancy
            }
            _ => {
                state.en_passant_target = None;
                &mut self.black_occupancy
            }
        };
        let (start, end): (Bitboard, Bitboard) = (!(1 << from_sq), 1 << to_sq);
        *&mut self.total_occupancy &= start;
        *&mut self.total_occupancy |= end;
        *color_to_mutate &= start;
        *color_to_mutate |= end;

        (
            previous_move.changed_cache_indices[0],
            self.cached_pieces[to_sq as usize],
        ) = match promotion_choice {
            None => {
                self.reset_bit(moving_piece, from_sq, to_sq);
                (
                    (Some((from_sq, to_sq)), Some(moving_piece)),
                    Some(moving_piece),
                )
            }
            Some(p) => {
                previous_move.promotion_happened = true;
                previous_move.changed_cache_indices[2] = (Some((from_sq, to_sq)), Some(p));
                match p {
                    WHITE_QUEEN_U8 => {
                        *&mut self.white_pawns &= start;
                        *&mut self.white_queens |= end
                    }
                    WHITE_ROOK_U8 => {
                        *&mut self.white_pawns &= start;
                        *&mut self.white_rooks |= end
                    }
                    WHITE_BISHOP_U8 => {
                        *&mut self.white_pawns &= start;
                        *&mut self.white_bishops |= end
                    }
                    WHITE_KNIGHT_U8 => {
                        *&mut self.white_pawns &= start;
                        *&mut self.white_knights |= end
                    }

                    BLACK_QUEEN_U8 => {
                        *&mut self.black_pawns &= start;
                        *&mut self.black_queens |= end
                    }
                    BLACK_ROOK_U8 => {
                        *&mut self.black_pawns &= start;
                        *&mut self.black_rooks |= end
                    }
                    BLACK_BISHOP_U8 => {
                        *&mut self.black_pawns &= start;
                        *&mut self.black_bishops |= end
                    }
                    BLACK_KNIGHT_U8 => {
                        *&mut self.black_pawns &= start;
                        *&mut self.black_knights |= end
                    }
                    _ => unreachable!(),
                };

                self.cached_pieces[to_sq as usize] = Some(p);
                ((Some((from_sq, to_sq)), Some(moving_piece)), Some(p))
            }
        };
        self.cached_pieces[from_sq as usize] = None;
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(&mut self, state: &mut GameState) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            self.material -= previous_move.material_difference;

            if let (Some(cached_index), Some(piece)) = previous_move.changed_cache_indices[0] {
                let (bitboard_to_restore, occupancy): (&mut u64, &mut u64) = match piece {
                    WHITE_BISHOP_U8 => (&mut self.white_bishops, &mut self.white_occupancy),
                    BLACK_BISHOP_U8 => (&mut self.black_bishops, &mut self.black_occupancy),
                    WHITE_KNIGHT_U8 => (&mut self.white_knights, &mut self.white_occupancy),
                    BLACK_KNIGHT_U8 => (&mut self.black_knights, &mut self.black_occupancy),
                    WHITE_PAWN_U8 => (&mut self.white_pawns, &mut self.white_occupancy),
                    BLACK_PAWN_U8 => (&mut self.black_pawns, &mut self.black_occupancy),
                    WHITE_QUEEN_U8 => (&mut self.white_queens, &mut self.white_occupancy),
                    BLACK_QUEEN_U8 => (&mut self.black_queens, &mut self.black_occupancy),
                    WHITE_ROOK_U8 => (&mut self.white_rooks, &mut self.white_occupancy),
                    BLACK_ROOK_U8 => (&mut self.black_rooks, &mut self.black_occupancy),
                    WHITE_KING_U8 => {
                        self.white_king_square = cached_index.0;
                        (&mut self.white_king, &mut self.white_occupancy)
                    }
                    BLACK_KING_U8 => {
                        self.black_king_square = cached_index.0;
                        (&mut self.black_king, &mut self.black_occupancy)
                    }
                    _ => unreachable!(),
                };

                let total: &mut Bitboard = &mut self.total_occupancy;
                let first_square: Bitboard = 1 << cached_index.0;
                self.cached_pieces[cached_index.0 as usize] = Some(piece);

                *total |= first_square;
                *bitboard_to_restore |= first_square;
                *occupancy |= first_square;

                self.cached_pieces[cached_index.1 as usize] = None;
                let last_square: Bitboard = !(1 << cached_index.1);
                *total &= last_square;
                *bitboard_to_restore &= last_square;
                *occupancy &= last_square;
            }

            // promoted piece
            if let (Some(cached_index), Some(piece)) = previous_move.changed_cache_indices[2] {
                let (bitboard_to_restore, occupancy): (&mut u64, &mut u64) = match piece {
                    WHITE_BISHOP_U8 => (&mut self.white_bishops, &mut self.white_occupancy),
                    BLACK_BISHOP_U8 => (&mut self.black_bishops, &mut self.black_occupancy),
                    WHITE_KNIGHT_U8 => (&mut self.white_knights, &mut self.white_occupancy),
                    BLACK_KNIGHT_U8 => (&mut self.black_knights, &mut self.black_occupancy),
                    WHITE_QUEEN_U8 => (&mut self.white_queens, &mut self.white_occupancy),
                    BLACK_QUEEN_U8 => (&mut self.black_queens, &mut self.black_occupancy),
                    WHITE_ROOK_U8 => (&mut self.white_rooks, &mut self.white_occupancy),
                    BLACK_ROOK_U8 => (&mut self.black_rooks, &mut self.black_occupancy),
                    _ => unreachable!(),
                };

                self.cached_pieces[cached_index.1 as usize] = None;
                let last_square: Bitboard = !(1 << cached_index.1);
                *bitboard_to_restore &= last_square;
                *occupancy &= last_square;
            }

            // captured piece / castled rook / en passant
            if let (Some(cached_index), Some(piece)) = previous_move.changed_cache_indices[1] {
                let (bitboard_to_restore, occupancy): (&mut u64, &mut u64) = match piece {
                    WHITE_BISHOP_U8 => (&mut self.white_bishops, &mut self.white_occupancy),
                    BLACK_BISHOP_U8 => (&mut self.black_bishops, &mut self.black_occupancy),
                    WHITE_KNIGHT_U8 => (&mut self.white_knights, &mut self.white_occupancy),
                    BLACK_KNIGHT_U8 => (&mut self.black_knights, &mut self.black_occupancy),
                    WHITE_PAWN_U8 => (&mut self.white_pawns, &mut self.white_occupancy),
                    BLACK_PAWN_U8 => (&mut self.black_pawns, &mut self.black_occupancy),
                    WHITE_QUEEN_U8 => (&mut self.white_queens, &mut self.white_occupancy),
                    BLACK_QUEEN_U8 => (&mut self.black_queens, &mut self.black_occupancy),
                    WHITE_ROOK_U8 => (&mut self.white_rooks, &mut self.white_occupancy),
                    BLACK_ROOK_U8 => (&mut self.black_rooks, &mut self.black_occupancy),
                    _ => unreachable!(),
                };

                let total: &mut Bitboard = &mut self.total_occupancy;
                let first_square: Bitboard = 1 << cached_index.0;

                self.cached_pieces[cached_index.0 as usize] = Some(piece);

                *total |= first_square;
                *bitboard_to_restore |= first_square;
                *occupancy |= first_square;
                if cached_index.0 != cached_index.1 {
                    self.cached_pieces[cached_index.1 as usize] = None;
                    let last_square: Bitboard = !(1 << cached_index.1);
                    *total &= last_square;
                    *bitboard_to_restore &= last_square;
                    *occupancy &= last_square;
                }
            }
            if let Some(castling_rights) = previous_move.previous_castling_rights {
                state.castling_rights = castling_rights;
            }
            state.en_passant_target = previous_move.previous_en_passant;
        }
    }

    pub fn generate_range(square1: u8, square2: u8, inclusion: &InclusiveRange) -> Bitboard {
        let mut rng: Bitboard = unsafe { RAYS_BETWEEN[square1 as usize][square2 as usize] };

        match inclusion {
            &InclusiveRange::Both => rng |= (1 << square1) | (1 << square2),
            &InclusiveRange::FirstOnly => rng |= 1 << square1,
            &InclusiveRange::LastOnly => rng |= 1 << square2,
            &InclusiveRange::None => (),
        }
        return rng;
    }

    pub fn is_capture(&self, m: &u16) -> bool {
        return self.piece_at(&((m & TO_MASK) >> TO_SHIFT)).is_some();
    }
}
