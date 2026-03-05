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

    #[inline(always)]
    pub fn piece_at(&self, square: u32) -> Option<u32> {
        let transformed_square: u64 = 1 << (square as u64);
        if transformed_square & self.white_occupancy != 0 {
            if transformed_square & self.white_pawns != 0 {
                return Some(WHITE_PAWN_U32);
            }
            if transformed_square & self.white_bishops != 0 {
                return Some(WHITE_BISHOP_U32);
            }
            if transformed_square & self.white_knights != 0 {
                return Some(WHITE_KNIGHT_U32);
            }
            if transformed_square & self.white_queens != 0 {
                return Some(WHITE_QUEEN_U32);
            }
            if transformed_square & self.white_rooks != 0 {
                return Some(WHITE_ROOK_U32);
            }
            if transformed_square & self.white_king != 0 {
                return Some(WHITE_KING_U32);
            }
        } else {
            if transformed_square & self.black_pawns != 0 {
                return Some(BLACK_PAWN_U32);
            }
            if transformed_square & self.black_bishops != 0 {
                return Some(BLACK_BISHOP_U32);
            }
            if transformed_square & self.black_knights != 0 {
                return Some(BLACK_KNIGHT_U32);
            }
            if transformed_square & self.black_queens != 0 {
                return Some(BLACK_QUEEN_U32);
            }
            if transformed_square & self.black_rooks != 0 {
                return Some(BLACK_ROOK_U32);
            }
            if transformed_square & self.black_king != 0 {
                return Some(BLACK_KING_U32);
            }
        }
        return None;
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
    fn reset_bit(&mut self, piece: u32, bit_position1: u8, bit_position2: u8, color: u32) -> () {
        let from_mask: Bitboard = !(1 << bit_position1);
        let to_mask: Bitboard = 1 << bit_position2;
        let bitboard_to_mutate: &mut Bitboard = if color == 8 {
            match self.piece_at(piece).unwrap() {
                WHITE_PAWN_U32 => &mut self.white_pawns,
                WHITE_KNIGHT_U32 => &mut self.white_knights,
                WHITE_BISHOP_U32 => &mut self.white_bishops,
                WHITE_QUEEN_U32 => &mut self.white_queens,
                WHITE_KING_U32 => &mut self.white_king,
                WHITE_ROOK_U32 => &mut self.white_rooks,
                _ => unreachable!(),
            }
        } else {
            match self.piece_at(piece).unwrap() {
                BLACK_PAWN_U32 => &mut self.black_pawns,
                BLACK_KNIGHT_U32 => &mut self.black_knights,
                BLACK_BISHOP_U32 => &mut self.black_bishops,
                BLACK_QUEEN_U32 => &mut self.black_queens,
                BLACK_KING_U32 => &mut self.black_king,
                BLACK_ROOK_U32 => &mut self.black_rooks,
                _ => unreachable!(),
            }
        };

        *bitboard_to_mutate &= from_mask;
        *bitboard_to_mutate |= to_mask;
    }

    fn perform_capture(
        &mut self,
        state: &mut GameState,
        enemy: u32,
        previous_move: &mut PreviousMove,
        to_sq: u8,
        color: u32,
    ) -> () {
        let (bitboard_for_capture, occupancy): (&mut Bitboard, &mut Bitboard) = if color == 16 {
            match enemy {
                WHITE_BISHOP_U32 => {
                    previous_move.material_difference -= BISHOP_VALUE;
                    self.material -= BISHOP_VALUE;
                    (&mut self.white_bishops, &mut self.white_occupancy)
                }
                WHITE_KNIGHT_U32 => {
                    previous_move.material_difference -= KNIGHT_VALUE;
                    self.material -= KNIGHT_VALUE;
                    (&mut self.white_knights, &mut self.white_occupancy)
                }
                WHITE_PAWN_U32 => {
                    previous_move.material_difference -= PAWN_VALUE;
                    self.material -= PAWN_VALUE;
                    (&mut self.white_pawns, &mut self.white_occupancy)
                }
                WHITE_QUEEN_U32 => {
                    previous_move.material_difference -= QUEEN_VALUE;
                    self.material -= QUEEN_VALUE;
                    (&mut self.white_queens, &mut self.white_occupancy)
                }
                WHITE_ROOK_U32 => {
                    previous_move.material_difference -= ROOK_VALUE;
                    self.material -= ROOK_VALUE;
                    if to_sq == 7 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.white_two_zeros = false;
                    } else if to_sq == 0 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.white_three_zeros = false;
                    }
                    (&mut self.white_rooks, &mut self.white_occupancy)
                }
                WHITE_KING_U32 => {
                    panic!("attemped to capture white king. state: {state:?}, board: {self:?}")
                }
                _ => unreachable!(),
            }
        } else {
            match enemy {
                BLACK_BISHOP_U32 => {
                    previous_move.material_difference += BISHOP_VALUE;
                    self.material += BISHOP_VALUE;
                    (&mut self.black_bishops, &mut self.black_occupancy)
                }
                BLACK_KNIGHT_U32 => {
                    previous_move.material_difference += KNIGHT_VALUE;
                    self.material += KNIGHT_VALUE;
                    (&mut self.black_knights, &mut self.black_occupancy)
                }
                BLACK_PAWN_U32 => {
                    previous_move.material_difference += PAWN_VALUE;
                    self.material += PAWN_VALUE;
                    (&mut self.black_pawns, &mut self.black_occupancy)
                }
                BLACK_QUEEN_U32 => {
                    previous_move.material_difference += QUEEN_VALUE;
                    self.material += QUEEN_VALUE;
                    (&mut self.black_queens, &mut self.black_occupancy)
                }
                BLACK_ROOK_U32 => {
                    previous_move.material_difference += ROOK_VALUE;
                    self.material += ROOK_VALUE;
                    if to_sq == 63 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.black_two_zeros = false;
                    } else if to_sq == 56 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        state.castling_rights.black_three_zeros = false;
                    }
                    (&mut self.black_rooks, &mut self.black_occupancy)
                }
                BLACK_KING_U32 => {
                    panic!("attemped to capture black king. state: {state:?}, board: {self:?}")
                }
                _ => unreachable!(),
            }
        };
        previous_move.moved_piece |= enemy << CAPTURED_PIECE_TYPE_SHIFT;
        let capture: Bitboard = !(1 << to_sq);
        *occupancy &= capture;
        *bitboard_for_capture &= capture;
    }

    fn castling(
        &mut self,
        previous_move: &mut PreviousMove,
        from_sq: u8,
        to_sq: u8,
        color: u32,
    ) -> () {
        let rook: u32 = COLORLESS_ROOK | color;
        previous_move.moved_piece |= 1 << CASTLING_SHIFT;
        self.reset_bit(rook, from_sq, to_sq, color);
        let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
        let (start, end): (Bitboard, Bitboard) = (!(1 << from_sq), 1 << to_sq);
        *total_occupancy &= start;
        *total_occupancy |= end;
        let occupancy: &mut Bitboard = match color {
            8 => &mut self.white_occupancy,
            16 => &mut self.black_occupancy,
            _ => unreachable!(),
        };
        *occupancy &= start;
        *occupancy |= end;
    }

    fn en_passant(&mut self, e_p: u8, previous_move: &mut PreviousMove, color: u32) -> () {
        let (pawns, occupancy, captured_pawn_square) = match color {
            8 => (&mut self.white_pawns, &mut self.white_occupancy, e_p + 8),
            16 => (&mut self.black_pawns, &mut self.black_occupancy, e_p - 8),
            _ => unreachable!(),
        };
        previous_move.moved_piece |= 1 << EN_PASSANT_SHIFT;
        let capture: Bitboard = !(1 << captured_pawn_square);
        *pawns &= capture;
        *occupancy &= capture;
        *&mut self.total_occupancy &= capture;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(&mut self, piece_move: u32, state: &mut GameState, color: u32) -> () {
        let (from_sq, to_sq): (u32, u32) =
            ((piece_move & FROM_MASK), (piece_move & TO_MASK) >> TO_SHIFT);
        let to_sq_u8: u8 = to_sq as u8;

        let moving_piece: u32 = moving_piece(piece_move);
        let captured_piece: u32 = captured_piece(piece_move);

        let mut promotion_choice: Option<u32> = None;
        let mut previous_move: PreviousMove = PreviousMove {
            moved_piece: piece_move,
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: None,
            material_difference: 0,
        };
        if captured_piece != 0 {
            self.perform_capture(state, captured_piece, &mut previous_move, to_sq_u8, color);
        }

        let color_to_mutate: &mut u64 = match moving_piece {
            WHITE_KING_U32 => {
                previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                self.white_king_square = to_sq as u8;
                match (from_sq, to_sq) {
                    (4, 2) => {
                        let (rook_from, rook_to) = (0, 3);
                        self.castling(&mut previous_move, rook_from, rook_to, NO_PIECE_WHITE);
                    }
                    (4, 6) => {
                        let (rook_from, rook_to) = (7, 5);
                        self.castling(&mut previous_move, rook_from, rook_to, NO_PIECE_WHITE);
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
            WHITE_ROOK_U32 => {
                if moving_piece_type(piece_move) != COLORLESS_ROOK
                    || captured_piece_type(piece_move) != COLORLESS_ROOK
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
            WHITE_PAWN_U32 => {
                match (from_sq, to_sq) {
                    (8..=15, 24..=31) => {
                        state.en_passant_target = Some(to_sq_u8 - 8); // en passant square behind the pawn
                    }
                    (48..56, 56..64) => {
                        promotion_choice = match (piece_move & PROMOTION_MASK) >> PROMOTION_SHIFT {
                            1 => {
                                self.material += KNIGHT_VALUE;
                                previous_move.material_difference += KNIGHT_VALUE;
                                Some(WHITE_KNIGHT_U32)
                            }
                            2 => {
                                self.material += BISHOP_VALUE;
                                previous_move.material_difference += BISHOP_VALUE;
                                Some(WHITE_BISHOP_U32)
                            }
                            3 => {
                                self.material += ROOK_VALUE;
                                previous_move.material_difference += ROOK_VALUE;
                                Some(WHITE_ROOK_U32)
                            }
                            4 => {
                                self.material += QUEEN_VALUE;
                                previous_move.material_difference += QUEEN_VALUE;
                                Some(WHITE_QUEEN_U32)
                            }
                            _ => unsafe { unreachable_unchecked() },
                        };
                        state.en_passant_target = None;
                    }
                    _ => {
                        if let Some(e_p) = state.en_passant_target {
                            if to_sq_u8 == e_p {
                                self.en_passant(e_p, &mut previous_move, NO_PIECE_BLACK);
                                previous_move.material_difference += PAWN_VALUE;
                                self.material += PAWN_VALUE;
                            }
                        }
                        state.en_passant_target = None;
                    }
                };
                &mut self.white_occupancy
            }
            WHITE_BISHOP_U32 | WHITE_KNIGHT_U32 | WHITE_QUEEN_U32 => {
                state.en_passant_target = None;
                &mut self.white_occupancy
            }

            BLACK_KING_U32 => {
                previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                self.black_king_square = to_sq as u8;
                match (from_sq, to_sq) {
                    (60, 58) => {
                        let (rook_from, rook_to) = (56, 59);
                        self.castling(&mut previous_move, rook_from, rook_to, NO_PIECE_BLACK);
                    }
                    (60, 62) => {
                        let (rook_from, rook_to) = (63, 61);
                        self.castling(&mut previous_move, rook_from, rook_to, NO_PIECE_BLACK);
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
            BLACK_ROOK_U32 => {
                if moving_piece_type(piece_move) != COLORLESS_ROOK
                    || captured_piece_type(piece_move) != COLORLESS_ROOK
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
            BLACK_PAWN_U32 => {
                match (from_sq, to_sq) {
                    (48..=55, 32..=39) => {
                        state.en_passant_target = Some(to_sq_u8 + 8); // en passant square behind the pawn
                    }
                    (8..16, 0..8) => {
                        promotion_choice = match (piece_move & PROMOTION_MASK) >> PROMOTION_SHIFT {
                            1 => {
                                self.material -= KNIGHT_VALUE;
                                previous_move.material_difference -= KNIGHT_VALUE;
                                Some(BLACK_KNIGHT_U32)
                            }
                            2 => {
                                self.material -= BISHOP_VALUE;
                                previous_move.material_difference -= BISHOP_VALUE;
                                Some(BLACK_BISHOP_U32)
                            }
                            3 => {
                                self.material -= ROOK_VALUE;
                                previous_move.material_difference -= ROOK_VALUE;
                                Some(BLACK_ROOK_U32)
                            }
                            4 => {
                                self.material -= QUEEN_VALUE;
                                previous_move.material_difference -= QUEEN_VALUE;
                                Some(BLACK_QUEEN_U32)
                            }
                            _ => unsafe { unreachable_unchecked() },
                        };
                        state.en_passant_target = None;
                    }
                    _ => {
                        if let Some(e_p) = state.en_passant_target {
                            if to_sq_u8 == e_p {
                                self.en_passant(e_p, &mut previous_move, NO_PIECE_WHITE);
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
        self.total_occupancy &= start;
        self.total_occupancy |= end;
        *color_to_mutate &= start;
        *color_to_mutate |= end;

        match promotion_choice {
            Some(p) => {
                if color == 8 {
                    match p {
                        WHITE_QUEEN_U32 => {
                            *&mut self.white_pawns &= start;
                            *&mut self.white_queens |= end
                        }
                        WHITE_ROOK_U32 => {
                            *&mut self.white_pawns &= start;
                            *&mut self.white_rooks |= end
                        }
                        WHITE_BISHOP_U32 => {
                            *&mut self.white_pawns &= start;
                            *&mut self.white_bishops |= end
                        }
                        WHITE_KNIGHT_U32 => {
                            *&mut self.white_pawns &= start;
                            *&mut self.white_knights |= end
                        }
                        _ => unreachable!(),
                    }
                } else {
                    match p {
                        BLACK_QUEEN_U32 => {
                            *&mut self.black_pawns &= start;
                            *&mut self.black_queens |= end
                        }
                        BLACK_ROOK_U32 => {
                            *&mut self.black_pawns &= start;
                            *&mut self.black_rooks |= end
                        }
                        BLACK_BISHOP_U32 => {
                            *&mut self.black_pawns &= start;
                            *&mut self.black_bishops |= end
                        }
                        BLACK_KNIGHT_U32 => {
                            *&mut self.black_pawns &= start;
                            *&mut self.black_knights |= end
                        }
                        _ => unreachable!(),
                    }
                };
            }
            None => {
                let piece_to_mutate: &mut u64 = if color == 8 {
                    match moving_piece {
                        WHITE_PAWN_U32 => &mut self.white_pawns,
                        WHITE_BISHOP_U32 => &mut self.white_bishops,
                        WHITE_KNIGHT_U32 => &mut self.white_knights,
                        WHITE_QUEEN_U32 => &mut self.white_queens,
                        WHITE_KING_U32 => &mut self.white_king,
                        WHITE_ROOK_U32 => &mut self.white_rooks,
                        _ => unreachable!("moving piece: {moving_piece}"),
                    }
                } else {
                    match moving_piece {
                        BLACK_PAWN_U32 => &mut self.black_pawns,
                        BLACK_BISHOP_U32 => &mut self.black_bishops,
                        BLACK_KNIGHT_U32 => &mut self.black_knights,
                        BLACK_QUEEN_U32 => &mut self.black_queens,
                        BLACK_KING_U32 => &mut self.black_king,
                        BLACK_ROOK_U32 => &mut self.black_rooks,
                        _ => unreachable!(),
                    }
                };
                *piece_to_mutate |= end;
                *piece_to_mutate &= start;
            }
        };
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(&mut self, state: &mut GameState, color: u32) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            self.material -= previous_move.material_difference;

            let m: u32 = previous_move.moved_piece;
            let (start, end, main_piece, promotion, captured_piece, castling, en_passant) = (
                from_square(m),
                to_square(m),
                moving_piece(m),
                promotion(m),
                captured_piece(m),
                castling(m),
                en_passant(m),
            );
            let (moved_piece_bitboard, color_occupancy): (&mut u64, &mut u64) = if color == 8 {
                (
                    match main_piece {
                        WHITE_PAWN_U32 => &mut self.white_pawns,
                        WHITE_KNIGHT_U32 => &mut self.white_knights,
                        WHITE_BISHOP_U32 => &mut self.white_bishops,
                        WHITE_QUEEN_U32 => &mut self.white_queens,
                        WHITE_KING_U32 => &mut self.white_king,
                        WHITE_ROOK_U32 => &mut self.white_rooks,
                        other => unreachable!("piece {other}"),
                    },
                    &mut self.white_occupancy,
                )
            } else {
                (
                    match main_piece {
                        BLACK_PAWN_U32 => &mut self.black_pawns,
                        BLACK_KNIGHT_U32 => &mut self.black_knights,
                        BLACK_BISHOP_U32 => &mut self.black_bishops,
                        BLACK_QUEEN_U32 => &mut self.black_queens,
                        BLACK_KING_U32 => &mut self.black_king,
                        BLACK_ROOK_U32 => &mut self.black_rooks,
                        other => unreachable!("piece {other}"),
                    },
                    &mut self.black_occupancy,
                )
            };
            let start_bb: u64 = 1 << start;
            let not_end_bb: u64 = !(1 << end);
            *moved_piece_bitboard &= not_end_bb;
            if promotion == 0 {
                *moved_piece_bitboard |= start_bb;
            } else {
                *match color {
                    8 => &mut self.white_pawns,
                    16 => &mut self.black_pawns,
                    _ => unreachable!(),
                } |= start_bb
            }

            *color_occupancy |= start_bb;
            *color_occupancy &= not_end_bb;
            self.total_occupancy |= start_bb;
            if captured_piece != 0 {
                let end_bb: u64 = 1 << end;
                *if color == 8 {
                    self.black_occupancy |= end_bb;
                    match captured_piece {
                        BLACK_PAWN_U32 => &mut self.black_pawns,
                        BLACK_KNIGHT_U32 => &mut self.black_knights,
                        BLACK_BISHOP_U32 => &mut self.black_bishops,
                        BLACK_QUEEN_U32 => &mut self.black_queens,
                        BLACK_ROOK_U32 => &mut self.black_rooks,
                        _ => unreachable!(),
                    }
                } else {
                    self.white_occupancy |= end_bb;
                    match captured_piece {
                        WHITE_PAWN_U32 => &mut self.white_pawns,
                        WHITE_KNIGHT_U32 => &mut self.white_knights,
                        WHITE_BISHOP_U32 => &mut self.white_bishops,
                        WHITE_QUEEN_U32 => &mut self.white_queens,
                        WHITE_ROOK_U32 => &mut self.white_rooks,
                        _ => unreachable!(),
                    }
                } |= end_bb;

                self.total_occupancy |= end_bb;
            } else {
                self.total_occupancy &= not_end_bb;
            }

            if en_passant != 0 {
                let (enemy_pawns, enemy_occupancy, taken_pawn_square): (&mut u64, &mut u64, u64) =
                    match color {
                        8 => (
                            &mut self.black_pawns,
                            &mut self.black_occupancy,
                            1 << (end - 8),
                        ),
                        16 => (
                            &mut self.white_pawns,
                            &mut self.white_occupancy,
                            1 << (end + 8),
                        ),
                        _ => unreachable!(),
                    };
                *enemy_pawns |= taken_pawn_square;
                self.total_occupancy |= taken_pawn_square;
                *enemy_occupancy |= taken_pawn_square;
            }

            if castling != 0 {
                let (rooks, rook_start, rook_end) = match (start, end) {
                    (4, 2) => (&mut self.white_rooks, 0, 3),
                    (4, 6) => (&mut self.white_rooks, 7, 5),
                    (60, 58) => (&mut self.black_rooks, 56, 59),
                    (60, 62) => (&mut self.black_rooks, 63, 61),
                    _ => unreachable!(),
                };
                *rooks |= 1 << rook_start;
                *rooks &= !(1 << rook_end);
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

    pub fn is_capture(&self, m: u32) -> bool {
        return self.piece_at((m & TO_MASK) >> TO_SHIFT).is_some();
    }
}
