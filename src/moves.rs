use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::gamestate::GameState;

#[derive(Clone, Copy)]
pub struct MoveList {
    pub pseudo_moves: [u32; 192],
    pub first_not_occupied: usize,
}

impl MoveList {
    fn push(&mut self, item: u32) -> () {
        self.pseudo_moves[self.first_not_occupied] = item;
        self.first_not_occupied += 1;
    }
}

impl Board {
    #[inline(always)]
    pub fn knight_moves(&self, color: u32, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut knights_bitboard, excluded_occupancy, moving_piece, enemy_bitboard): (
            u64,
            u64,
            u32,
            &u64,
        ) = match color {
            8 => (
                self.white_knights,
                !self.white_occupancy,
                WHITE_KNIGHT_U32,
                &self.black_occupancy,
            ),
            16 => (
                self.black_knights,
                !self.black_occupancy,
                BLACK_KNIGHT_U32,
                &self.white_occupancy,
            ),
            _ => unreachable!(),
        };

        while knights_bitboard != 0 {
            let initial_pos: u32 = knights_bitboard.trailing_zeros();
            let attacks: Bitboard = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: Bitboard = attacks & excluded_occupancy;
            if captures_only {
                dest_bitboard &= enemy_bitboard;
            }

            while dest_bitboard != 0 {
                let final_pos: u32 = dest_bitboard.trailing_zeros();
                let mut pseudo_move: u32 = initial_pos
                    | (final_pos << TO_SHIFT)
                    | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
                if enemy_bitboard & (1 << final_pos) != 0 {
                    pseudo_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                        << CAPTURED_PIECE_TYPE_SHIFT;
                }
                moves.push(pseudo_move);

                dest_bitboard &= dest_bitboard - 1;
            }
            knights_bitboard &= knights_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn pawn_moves(
        &self,
        state: &GameState,
        color: u32,
        moves: &mut MoveList,
        captures_only: bool,
    ) -> () {
        let (mut pawns_bitboard, enemy_occupancy, moving_piece, promo_rank, e_p_rank): (
            u64,
            u64,
            u32,
            &u64,
            &u64,
        ) = match color {
            8 => (
                self.white_pawns,
                self.black_occupancy,
                WHITE_PAWN_U32,
                &RANK_8,
                &RANK_5,
            ),
            16 => (
                self.black_pawns,
                self.white_occupancy,
                BLACK_PAWN_U32,
                &RANK_1,
                &RANK_4,
            ),
            _ => unreachable!(),
        };
        let en_passant: u32 = if let Some(e_p) = state.en_passant_target {
            e_p as u32
        } else {
            64
        };

        while pawns_bitboard != 0 {
            let initial_pos: u32 = pawns_bitboard.trailing_zeros();
            let attacks: Bitboard = match color {
                8 => WHITE_PAWN_ATTACKS[initial_pos as usize],
                16 => BLACK_PAWN_ATTACKS[initial_pos as usize],
                _ => unreachable!(),
            };
            let mut dest_bitboard: Bitboard = attacks & enemy_occupancy;
            if en_passant < 64 && (1 << initial_pos) & e_p_rank != 0 {
                dest_bitboard |= 1 << en_passant;
            }
            if !captures_only {
                let forward_square: u32 = match color {
                    8 => initial_pos + 8,
                    16 => initial_pos.wrapping_sub(8),
                    _ => unreachable!(),
                };
                if forward_square < 64 && (self.total_occupancy >> forward_square) & 1 == 0 {
                    dest_bitboard |= 1 << forward_square;
                    let second_forward_square: u32 = match color {
                        8 => initial_pos + 16,
                        16 => initial_pos.wrapping_sub(16),
                        _ => unreachable!(),
                    };
                    if match color {
                        8 => (1 << initial_pos) & RANK_2 != 0,
                        16 => (1 << initial_pos) & RANK_7 != 0,
                        _ => unreachable!(),
                    } && (self.total_occupancy >> second_forward_square) & 1 == 0
                    {
                        dest_bitboard |= 1 << second_forward_square;
                    }
                }
            }
            while dest_bitboard != 0 {
                let final_pos: u32 = dest_bitboard.trailing_zeros();
                let mut piece_move: u32 = initial_pos
                    | (final_pos << TO_SHIFT)
                    | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
                if final_pos == en_passant {
                    piece_move |= 1 << EN_PASSANT_SHIFT;
                }
                if enemy_occupancy & (1 << final_pos) != 0 {
                    piece_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                        << CAPTURED_PIECE_TYPE_SHIFT;
                }
                if promo_rank & (1 << final_pos) != 0 {
                    moves.push(piece_move | (0b100 << PROMOTION_SHIFT));
                    moves.push(piece_move | (0b011 << PROMOTION_SHIFT));
                    moves.push(piece_move | (0b010 << PROMOTION_SHIFT));
                    moves.push(piece_move | (0b001 << PROMOTION_SHIFT));
                } else {
                    moves.push(piece_move);
                }
                dest_bitboard &= dest_bitboard - 1;
            }
            pawns_bitboard &= pawns_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, square: u8, by: u32) -> bool {
        let usize_square: usize = square as usize;
        if KNIGHT_ATTACKS[usize_square]
            & match by {
                8 => self.white_knights,
                16 => self.black_knights,
                _ => unreachable!(),
            }
            != 0
        {
            return true;
        }

        let pawn_attacks: Bitboard = match by {
            8 => BLACK_PAWN_ATTACKS[usize_square],
            16 => WHITE_PAWN_ATTACKS[usize_square],
            _ => unreachable!(),
        };
        if pawn_attacks
            & match by {
                8 => self.white_pawns,
                16 => self.black_pawns,
                _ => unreachable!(),
            }
            != 0
        {
            return true;
        }

        if bishop_attacks(usize_square, self.total_occupancy)
            & match by {
                8 => self.white_bishops | self.white_queens,
                16 => self.black_bishops | self.black_queens,
                _ => unreachable!(),
            }
            != 0
        {
            return true;
        }

        if rook_attacks(usize_square, self.total_occupancy)
            & match by {
                8 => self.white_rooks | self.white_queens,
                16 => self.black_rooks | self.black_queens,
                _ => unreachable!(),
            }
            != 0
        {
            return true;
        }
        if KING_ATTACKS[usize_square]
            & match by {
                8 => self.white_king,
                16 => self.black_king,
                _ => unreachable!(),
            }
            != 0
        {
            return true;
        }

        return false;
    }

    #[inline(always)]
    pub fn king_moves(
        &self,
        state: &GameState,
        color: u32,
        moves: &mut MoveList,
        captures_only: bool,
    ) -> () {
        let (initial_pos, opposite_color, moving_piece, enemy_occupancy): (u32, u32, u32, &u64) =
            match color {
                8 => (
                    self.white_king_square as u32,
                    16,
                    WHITE_KING_U32,
                    &self.black_occupancy,
                ),
                16 => (
                    self.black_king_square as u32,
                    8,
                    BLACK_KING_U32,
                    &self.white_occupancy,
                ),
                _ => unreachable!(),
            };
        let mut dest_bitboard: Bitboard = KING_ATTACKS[initial_pos as usize]
            & !match color {
                8 => self.white_occupancy,
                16 => self.black_occupancy,
                _ => unreachable!(),
            };
        if captures_only {
            dest_bitboard &= enemy_occupancy;
        }

        while dest_bitboard != 0 {
            let final_pos: u32 = dest_bitboard.trailing_zeros();
            let mut piece_move: u32 =
                initial_pos | (final_pos << TO_SHIFT) | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
            if enemy_occupancy & (1 << final_pos) != 0 {
                piece_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                    << CAPTURED_PIECE_TYPE_SHIFT;
            }
            moves.push(piece_move);
            dest_bitboard &= dest_bitboard - 1;
        }

        if self.is_square_attacked(initial_pos as u8, opposite_color) | captures_only {
            return;
        }
        let (castling_squares, mut right_path, mut left_path): (
            (Option<u8>, Option<u8>),
            Bitboard,
            Bitboard,
        ) = match color {
            8 => match (
                &state.castling_rights & WHITE_LONG_MASK != 0,
                &state.castling_rights & WHITE_SHORT_MASK != 0,
            ) {
                (true, true) => (
                    (Some(2), Some(6)),
                    0b0000000000000000000000000000000000000000000000000000000000001110,
                    0b0000000000000000000000000000000000000000000000000000000001100000,
                ),
                (false, false) => return,
                (true, false) => (
                    (Some(2), None),
                    0b0000000000000000000000000000000000000000000000000000000000001110,
                    0,
                ),
                (false, true) => (
                    (None, Some(6)),
                    0,
                    0b0000000000000000000000000000000000000000000000000000000001100000,
                ),
            },
            16 => match (
                &state.castling_rights & BLACK_SHORT_MASK != 0,
                &state.castling_rights & BLACK_LONG_MASK != 0,
            ) {
                (true, true) => (
                    (Some(62), Some(58)),
                    0b0110000000000000000000000000000000000000000000000000000000000000,
                    0b0000111000000000000000000000000000000000000000000000000000000000,
                ),
                (false, false) => return,
                (true, false) => (
                    (Some(62), None),
                    0b0110000000000000000000000000000000000000000000000000000000000000,
                    0,
                ),
                (false, true) => (
                    (None, Some(58)),
                    0,
                    0b0000111000000000000000000000000000000000000000000000000000000000,
                ),
            },
            _ => unreachable!(),
        };

        if left_path != 0 && (left_path & self.total_occupancy == 0) {
            let mut finished_fully: bool = true;
            while left_path != 0 {
                let square: u8 = left_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, opposite_color) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                left_path &= left_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.1 {
                moves.push(
                    initial_pos
                        | ((sq as u32) << TO_SHIFT)
                        | (moving_piece << MOVING_PIECE_TYPE_SHIFT)
                        | (1 << CASTLING_SHIFT),
                );
            }
        }
        if right_path != 0 && (right_path & self.total_occupancy == 0) {
            let mut finished_fully: bool = true;
            while right_path != 0 {
                let square: u8 = right_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, opposite_color) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                right_path &= right_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.0 {
                moves.push(
                    initial_pos
                        | ((sq as u32) << TO_SHIFT)
                        | (moving_piece << MOVING_PIECE_TYPE_SHIFT)
                        | (1 << CASTLING_SHIFT),
                );
            }
        }
    }

    #[inline(always)]
    pub fn rook_moves(&self, color: u32, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut rooks_bitboard, friendly_occupancy, moving_piece, enemy_occupancy): (
            u64,
            u64,
            u32,
            &u64,
        ) = match color {
            8 => (
                self.white_rooks,
                self.white_occupancy,
                WHITE_ROOK_U32,
                &self.black_occupancy,
            ),
            16 => (
                self.black_rooks,
                self.black_occupancy,
                BLACK_ROOK_U32,
                &self.white_occupancy,
            ),
            _ => unreachable!(),
        };

        while rooks_bitboard != 0 {
            let initial_pos: u32 = rooks_bitboard.trailing_zeros();
            let attacks: Bitboard = rook_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }
            while dest_bitboard != 0 {
                let final_pos: u32 = dest_bitboard.trailing_zeros();
                let mut piece_move: u32 = initial_pos
                    | (final_pos << TO_SHIFT)
                    | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
                if enemy_occupancy & (1 << final_pos) != 0 {
                    piece_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                        << CAPTURED_PIECE_TYPE_SHIFT;
                }
                moves.push(piece_move);

                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn bishop_moves(&self, color: u32, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut bishops_bitboard, friendly_occupancy, moving_piece, enemy_occupancy): (
            u64,
            u64,
            u32,
            &u64,
        ) = match color {
            8 => (
                self.white_bishops,
                self.white_occupancy,
                WHITE_BISHOP_U32,
                &self.black_occupancy,
            ),
            16 => (
                self.black_bishops,
                self.black_occupancy,
                BLACK_BISHOP_U32,
                &self.white_occupancy,
            ),
            _ => unreachable!(),
        };

        while bishops_bitboard != 0 {
            let initial_pos: u32 = bishops_bitboard.trailing_zeros();
            let attacks: Bitboard = bishop_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }

            while dest_bitboard != 0 {
                let final_pos: u32 = dest_bitboard.trailing_zeros();
                let mut piece_move: u32 = initial_pos
                    | (final_pos << TO_SHIFT)
                    | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
                if enemy_occupancy & (1 << final_pos) != 0 {
                    piece_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                        << CAPTURED_PIECE_TYPE_SHIFT;
                }
                moves.push(piece_move);
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn queen_moves(&self, color: u32, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut queens_bitboard, friendly_occupancy, moving_piece, enemy_occupancy): (
            u64,
            u64,
            u32,
            &u64,
        ) = match color {
            8 => (
                self.white_queens,
                self.white_occupancy,
                WHITE_QUEEN_U32,
                &self.black_occupancy,
            ),
            16 => (
                self.black_queens,
                self.black_occupancy,
                BLACK_QUEEN_U32,
                &self.white_occupancy,
            ),
            _ => unreachable!(),
        };

        while queens_bitboard != 0 {
            let initial_pos: u32 = queens_bitboard.trailing_zeros();
            let initial_pos_as_index: usize = initial_pos as usize;
            let attacks: Bitboard = bishop_attacks(initial_pos_as_index, self.total_occupancy)
                | rook_attacks(initial_pos_as_index, self.total_occupancy);

            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }

            while dest_bitboard != 0 {
                let final_pos: u32 = dest_bitboard.trailing_zeros();
                let mut piece_move: u32 = initial_pos
                    | (final_pos << TO_SHIFT)
                    | (moving_piece << MOVING_PIECE_TYPE_SHIFT);
                if enemy_occupancy & (1 << final_pos) != 0 {
                    piece_move |= unsafe { self.piece_at(final_pos).unwrap_unchecked() }
                        << CAPTURED_PIECE_TYPE_SHIFT;
                }
                moves.push(piece_move);
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }
    }
}
