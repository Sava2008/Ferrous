use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::gamestate::GameState;

#[derive(Clone, Copy)]
pub struct MoveList {
    pub pseudo_moves: [u16; 192],
    pub first_not_occupied: usize,
}

impl MoveList {
    fn push(&mut self, item: u16) -> () {
        self.pseudo_moves[self.first_not_occupied] = item;
        self.first_not_occupied += 1;
    }
}

impl Board {
    #[inline(always)]
    pub fn knight_moves(&self, color: u16, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut knights_bitboard, excluded_occupancy, enemy_bitboard): (u64, u64, &u64) =
            match color {
                8 => (
                    self.bitboards[1],
                    !self.occupancies[0],
                    &self.occupancies[1],
                ),
                _ => (
                    self.bitboards[7],
                    !self.occupancies[1],
                    &self.occupancies[0],
                ),
            };

        while knights_bitboard != 0 {
            let initial_pos: u16 = knights_bitboard.trailing_zeros() as u16;
            let attacks: u64 = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: u64 = attacks & excluded_occupancy;
            if captures_only {
                dest_bitboard &= enemy_bitboard;
            }

            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                moves.push(initial_pos | (final_pos << TO_SHIFT));

                dest_bitboard &= dest_bitboard - 1;
            }
            knights_bitboard &= knights_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn pawn_moves(
        &self,
        state: &GameState,
        color: u16,
        moves: &mut MoveList,
        captures_only: bool,
    ) -> () {
        let en_passant: u16 = if let Some(e_p) = state.en_passant_target {
            e_p as u16
        } else {
            64
        };
        let (mut pawns_bitboard, enemy_occupancy, promo_rank, e_p_rank): (u64, u64, &u64, &u64) =
            match color {
                8 => (
                    self.bitboards[0],
                    self.occupancies[1],
                    &RANK_8,
                    if en_passant < 64 { &RANK_5 } else { &0 },
                ),
                _ => (
                    self.bitboards[6],
                    self.occupancies[0],
                    &RANK_1,
                    if en_passant < 64 { &RANK_4 } else { &0 },
                ),
            };
        let e_p_bitboard = if en_passant < 64 { 1 << en_passant } else { 0 };

        while pawns_bitboard != 0 {
            let initial_pos: u16 = pawns_bitboard.trailing_zeros() as u16;

            let attacks: u64 = match color {
                8 => WHITE_PAWN_ATTACKS[initial_pos as usize],
                _ => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: u64 = attacks & enemy_occupancy;
            if (1 << initial_pos) & e_p_rank != 0 && attacks & e_p_bitboard != 0 {
                dest_bitboard |= e_p_bitboard;
            }
            if !captures_only {
                let forward_square: u16 = match color {
                    8 => initial_pos + 8,
                    _ => initial_pos.saturating_sub(8),
                };
                if forward_square < 64 && (self.total_occupancy >> forward_square) & 1 == 0 {
                    dest_bitboard |= 1 << forward_square;
                    let second_forward_square: u16 = match color {
                        8 => initial_pos + 16,
                        _ => initial_pos.saturating_sub(16),
                    };
                    if match color {
                        8 => (1 << initial_pos) & RANK_2 != 0,
                        _ => (1 << initial_pos) & RANK_7 != 0,
                    } && (self.total_occupancy >> second_forward_square) & 1 == 0
                    {
                        dest_bitboard |= 1 << second_forward_square;
                    }
                }
            }
            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let mut piece_move: u16 = initial_pos | (final_pos << TO_SHIFT);
                if final_pos == en_passant {
                    piece_move |= 2 << MARK_SHIFT;
                }
                if promo_rank & (1 << final_pos) != 0 {
                    moves.push(piece_move | (0b0110 << MARK_SHIFT));
                    moves.push(piece_move | (0b0101 << MARK_SHIFT));
                    moves.push(piece_move | (0b0100 << MARK_SHIFT));
                    moves.push(piece_move | (0b0011 << MARK_SHIFT));
                } else {
                    moves.push(piece_move);
                }
                dest_bitboard &= dest_bitboard - 1;
            }
            pawns_bitboard &= pawns_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, square: u8, by: u16) -> bool {
        let usize_square: usize = square as usize;
        let (w_q, b_q) = (&self.bitboards[4], &self.bitboards[10]);
        let (
            attacking_pawns,
            pawn_attacks,
            attacking_knights,
            diagonal_attackers,
            linear_attackers,
            attacking_king,
        ) = match by {
            8 => (
                &self.bitboards[0],
                &BLACK_PAWN_ATTACKS[usize_square],
                &self.bitboards[1],
                &(self.bitboards[2] | w_q),
                &(self.bitboards[3] | w_q),
                &self.bitboards[5],
            ),
            _ => (
                &self.bitboards[6],
                &WHITE_PAWN_ATTACKS[usize_square],
                &self.bitboards[7],
                &(self.bitboards[8] | b_q),
                &(self.bitboards[9] | b_q),
                &self.bitboards[11],
            ),
        };
        if (KNIGHT_ATTACKS[usize_square] & attacking_knights != 0)
            | (attacking_pawns & pawn_attacks != 0)
            | (bishop_attacks(usize_square, self.total_occupancy) & diagonal_attackers != 0)
            | (rook_attacks(usize_square, self.total_occupancy) & linear_attackers != 0)
            | (KING_ATTACKS[usize_square] & attacking_king != 0)
        {
            return true;
        }
        return false;
    }

    #[inline(always)]
    pub fn king_moves(
        &self,
        state: &GameState,
        color: u16,
        moves: &mut MoveList,
        captures_only: bool,
    ) -> () {
        let (black_occ, white_occ) = (self.occupancies[1], self.occupancies[0]);
        let (initial_pos, opposite_color, enemy_occupancy): (u16, u16, &u64) = match color {
            8 => (self.white_king_square as u16, 16, &black_occ),
            _ => (self.black_king_square as u16, 8, &white_occ),
        };
        let mut dest_bitboard: u64 = KING_ATTACKS[initial_pos as usize]
            & !match color {
                8 => white_occ,
                _ => black_occ,
            };
        if captures_only {
            dest_bitboard &= enemy_occupancy;
        }

        while dest_bitboard != 0 {
            let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
            moves.push(initial_pos | (final_pos << TO_SHIFT));
            dest_bitboard &= dest_bitboard - 1;
        }

        if self.is_square_attacked(initial_pos as u8, opposite_color) || captures_only {
            return;
        }
        let (castling_squares, mut kingside_path, mut queenside_path): (
            (Option<u8>, Option<u8>),
            u64,
            u64,
        ) = match color {
            8 => match (
                &state.castling_rights & WHITE_SHORT_MASK != 0,
                &state.castling_rights & WHITE_LONG_MASK != 0,
            ) {
                (true, true) => (
                    (Some(6), Some(2)),
                    0b0000000000000000000000000000000000000000000000000000000001100000,
                    0b0000000000000000000000000000000000000000000000000000000000001110,
                ),
                (false, false) => return,
                (false, true) => (
                    (None, Some(2)),
                    0,
                    0b0000000000000000000000000000000000000000000000000000000000001110,
                ),
                (true, false) => (
                    (Some(6), None),
                    0b0000000000000000000000000000000000000000000000000000000001100000,
                    0,
                ),
            },
            _ => match (
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
        };

        if queenside_path != 0 && (queenside_path & self.total_occupancy == 0) {
            let mut finished_fully: bool = true;
            while queenside_path != 0 {
                let square: u8 = queenside_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, opposite_color) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                queenside_path &= queenside_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.1 {
                moves.push(initial_pos | ((sq as u16) << TO_SHIFT) | (0b0001 << MARK_SHIFT));
            }
        }
        if kingside_path != 0 && (kingside_path & self.total_occupancy == 0) {
            let mut finished_fully: bool = true;
            while kingside_path != 0 {
                let square: u8 = kingside_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, opposite_color) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                kingside_path &= kingside_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.0 {
                moves.push(initial_pos | ((sq as u16) << TO_SHIFT) | (0b0001 << MARK_SHIFT));
            }
        }
    }

    #[inline(always)]
    pub fn rook_moves(&self, color: u16, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut rooks_bitboard, friendly_occupancy, enemy_occupancy): (u64, u64, &u64) =
            match color {
                8 => (self.bitboards[3], self.occupancies[0], &self.occupancies[1]),
                _ => (self.bitboards[9], self.occupancies[1], &self.occupancies[0]),
            };

        while rooks_bitboard != 0 {
            let initial_pos: u16 = rooks_bitboard.trailing_zeros() as u16;
            let attacks: u64 = rook_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: u64 = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }
            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                moves.push(initial_pos | (final_pos << TO_SHIFT));

                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn bishop_moves(&self, color: u16, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut bishops_bitboard, friendly_occupancy, enemy_occupancy): (u64, u64, &u64) =
            match color {
                8 => (self.bitboards[2], self.occupancies[0], &self.occupancies[1]),
                _ => (self.bitboards[8], self.occupancies[1], &self.occupancies[0]),
            };

        while bishops_bitboard != 0 {
            let initial_pos: u16 = bishops_bitboard.trailing_zeros() as u16;
            let attacks: u64 = bishop_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: u64 = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }

            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                moves.push(initial_pos | (final_pos << TO_SHIFT));
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn queen_moves(&self, color: u16, moves: &mut MoveList, captures_only: bool) -> () {
        let (mut queens_bitboard, friendly_occupancy, enemy_occupancy): (u64, u64, &u64) =
            match color {
                8 => (self.bitboards[4], self.occupancies[0], &self.occupancies[1]),
                _ => (
                    self.bitboards[10],
                    self.occupancies[1],
                    &self.occupancies[0],
                ),
            };

        while queens_bitboard != 0 {
            let initial_pos: u16 = queens_bitboard.trailing_zeros() as u16;
            let initial_pos_as_index: usize = initial_pos as usize;
            let attacks: u64 = bishop_attacks(initial_pos_as_index, self.total_occupancy)
                | rook_attacks(initial_pos_as_index, self.total_occupancy);

            let mut dest_bitboard: u64 = attacks & !friendly_occupancy;
            if captures_only {
                dest_bitboard &= enemy_occupancy;
            }

            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                moves.push(initial_pos | (final_pos << TO_SHIFT));
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }
    }
}
