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
    pub fn en_passant_exposes_king(
        &self,
        from: u16,
        e_p_pawn: u16,
        king_color: u16,
        king_square: u8,
    ) -> (bool, u64) {
        let (king_sq, from_sq) = (king_square as usize, from as usize);
        let king_pawn_line: u64 = unsafe { TWO_SQUARES_LINE[king_sq][from_sq] };
        if king_pawn_line == 0 {
            return (false, 0);
        }
        let en_passantless_occupancy: u64 = self.total_occupancy & !(1 << e_p_pawn);
        if unsafe { RAYS_BETWEEN[king_sq][from_sq] } & en_passantless_occupancy != 0 {
            return (false, 0);
        }
        let pinners_area: u64 = unsafe { RAYS_FROM[king_sq][from_sq] };
        let is_line: bool = (from_sq % 8 == king_sq % 8) || (from_sq / 8 == king_sq / 8);
        let attackers: u64 = if king_color == 8 {
            if is_line {
                (self.bitboards[9] | self.bitboards[10])
                    & (rook_attacks(from_sq, en_passantless_occupancy) & pinners_area)
            } else {
                (self.bitboards[8] | self.bitboards[10])
                    & (bishop_attacks(from_sq, en_passantless_occupancy) & pinners_area)
            }
        } else {
            if is_line {
                (self.bitboards[3] | self.bitboards[4])
                    & (rook_attacks(from_sq, en_passantless_occupancy) & pinners_area)
            } else {
                (self.bitboards[2] | self.bitboards[4])
                    & (bishop_attacks(from_sq, en_passantless_occupancy) & pinners_area)
            }
        };
        if attackers == 0 {
            return (false, 0);
        }

        return (true, attackers);
    }
    #[inline(always)]
    pub fn exposes_king(&self, from: u16, king_square: u8, king_color: u16) -> (bool, u64) {
        let king_sq: usize = king_square as usize;
        let from_sq: usize = from as usize;

        if unsafe { TWO_SQUARES_LINE[king_sq][from_sq] == 0 } {
            return (false, 0);
        }

        let occ_no_moving_piece: u64 = self.total_occupancy & !(1 << from);
        if occ_no_moving_piece & unsafe { RAYS_BETWEEN[king_sq][from_sq] } != 0 {
            return (false, 0);
        }

        let (linear_enemies, diagonal_enemies) = if king_color == 16 {
            (
                (self.bitboards[3] | self.bitboards[4])
                    & rook_attacks(king_sq, occ_no_moving_piece),
                (self.bitboards[2] | self.bitboards[4])
                    & bishop_attacks(king_sq, occ_no_moving_piece),
            )
        } else {
            (
                (self.bitboards[9] | self.bitboards[10])
                    & rook_attacks(king_sq, occ_no_moving_piece),
                (self.bitboards[8] | self.bitboards[10])
                    & bishop_attacks(king_sq, occ_no_moving_piece),
            )
        };

        if diagonal_enemies.count_ones() > 1 || linear_enemies.count_ones() > 1 {
            return (true, 0);
        }
        let diagonal_attacker: usize = diagonal_enemies.trailing_zeros() as usize;
        let linear_attacker: usize = linear_enemies.trailing_zeros() as usize;

        return match (diagonal_attacker, linear_attacker) {
            (64, 64) => (false, 0),
            (_, 64) => {
                if unsafe {
                    TWO_SQUARES_LINE[king_sq][from as usize]
                        != TWO_SQUARES_LINE[king_sq][diagonal_attacker]
                } {
                    return (false, 0);
                }
                (
                    true,
                    unsafe { RAYS_BETWEEN[king_sq][diagonal_attacker] } | diagonal_enemies,
                )
            }
            (64, _) => {
                if unsafe {
                    TWO_SQUARES_LINE[king_sq][from as usize]
                        != TWO_SQUARES_LINE[king_sq][linear_attacker]
                } {
                    return (false, 0);
                }
                (
                    true,
                    unsafe { RAYS_BETWEEN[king_sq][linear_attacker] } | linear_enemies,
                )
            }
            _ => (true, 0),
        };
    }

    #[inline(always)]
    fn check_info(
        &self,
        from: u16,
        to: u16,
        flag: u16,
        king_square: usize,
        king_color: u16,
        mut moving_piece: u16,
        check_squares: &[u64; 5],
    ) -> u16 {
        let total_occ: u64 = self.total_occupancy & !(1 << from);
        let to_sq_bb: u64 = 1 << to;
        if moving_piece > 6 {
            moving_piece -= 6;
        }

        let (queen_idx, rook_idx, bishop_idx) = if king_color == 8 {
            (10, 9, 8)
        } else {
            (4, 3, 2)
        };
        let direct_attacks: u64 = if flag == 0 || flag == 1 {
            if moving_piece == 6 {
                0
            } else {
                check_squares[moving_piece as usize - 1]
            }
        } else {
            check_squares[flag.saturating_sub(2) as usize]
        };
        let diag_discovery_attacks: usize = ((bishop_attacks(king_square, total_occ) & !to_sq_bb)
            & (self.bitboards[queen_idx] | self.bitboards[bishop_idx]))
            .trailing_zeros() as usize;
        let line_discovery_attacks: usize = ((rook_attacks(king_square, total_occ) & !to_sq_bb)
            & (self.bitboards[queen_idx] | self.bitboards[rook_idx]))
            .trailing_zeros() as usize;

        let squares: u64 = unsafe {
            if diag_discovery_attacks != 64 {
                RAYS_BETWEEN[king_square][diag_discovery_attacks] | (1 << diag_discovery_attacks)
            } else if line_discovery_attacks != 64 {
                RAYS_BETWEEN[king_square][line_discovery_attacks] | (1 << line_discovery_attacks)
            } else {
                64
            }
        };
        let is_check: bool = direct_attacks & to_sq_bb != 0 || squares != 64;
        if is_check {
            return flag + 7;
        }
        return flag;
    }

    #[inline(always)]
    pub fn knight_moves(
        &self,
        color: u16,
        moves: &mut MoveList,
        state: &GameState,
        captures_checks_only: bool,
    ) -> () {
        let (
            mut knights_bitboard,
            excluded_occupancy,
            king_sq,
            check_restrictions,
            opposite_color,
            moving_piece,
        ): (u64, u64, u8, &u64, u16, u16) = match color {
            8 => (
                self.bitboards[1],
                !self.occupancies[0],
                self.white_king_square,
                &state.white_legal_squares_mask,
                16,
                WHITE_KNIGHT_U16,
            ),
            _ => (
                self.bitboards[7],
                !self.occupancies[1],
                self.black_king_square,
                &state.black_legal_squares_mask,
                8,
                BLACK_KNIGHT_U16,
            ),
        };
        let king_sq_idx: usize = king_sq as usize;

        while knights_bitboard != 0 {
            let initial_pos: u16 = knights_bitboard.trailing_zeros() as u16;
            if self.exposes_king(initial_pos, king_sq, color).0 {
                knights_bitboard &= knights_bitboard - 1; // pinned knights cannot move at all
                continue;
            }
            let attacks: u64 = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: u64 = attacks & excluded_occupancy & check_restrictions;

            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let check_flag: u16 = self.check_info(
                    initial_pos,
                    final_pos,
                    0,
                    king_sq_idx,
                    opposite_color,
                    moving_piece,
                    &state.check_squares,
                );
                if captures_checks_only
                    && (self.cached_pieces[final_pos as usize] == 0 && check_flag == 0)
                {
                    dest_bitboard &= dest_bitboard - 1;
                    continue;
                }
                moves.push(initial_pos | (final_pos << TO_SHIFT) | (check_flag << MARK_SHIFT));

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
        captures_checks_only: bool,
    ) -> () {
        let en_passant: u16 = if let Some(e_p) = state.en_passant_target {
            e_p as u16
        } else {
            64
        };
        let (
            mut pawns_bitboard,
            enemy_occupancy,
            promo_rank,
            e_p_rank,
            king_sq,
            en_passant_pawn,
            check_restrictions,
            opposite_color,
            moving_piece,
        ): (u64, u64, &u64, &u64, u8, u16, &u64, u16, u16) = match color {
            8 => (
                self.bitboards[0],
                self.occupancies[1],
                &RANK_8,
                if en_passant < 64 { &RANK_5 } else { &0 },
                self.white_king_square,
                en_passant - 8,
                &state.white_legal_squares_mask,
                16,
                WHITE_PAWN_U16,
            ),
            _ => (
                self.bitboards[6],
                self.occupancies[0],
                &RANK_1,
                if en_passant < 64 { &RANK_4 } else { &0 },
                self.black_king_square,
                en_passant + 8,
                &state.black_legal_squares_mask,
                8,
                BLACK_PAWN_U16,
            ),
        };
        let king_sq_idx: usize = king_sq as usize;
        let e_p_bitboard: u64 = if en_passant < 64 { 1 << en_passant } else { 0 };

        while pawns_bitboard != 0 {
            let initial_pos: u16 = pawns_bitboard.trailing_zeros() as u16;

            let attacks: u64 = match color {
                8 => WHITE_PAWN_ATTACKS[initial_pos as usize],
                _ => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: u64 = attacks & enemy_occupancy;
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

            let (exposes_king, allowed_squares) = self.exposes_king(initial_pos, king_sq, color);
            dest_bitboard &= check_restrictions;
            if (1 << initial_pos) & e_p_rank != 0 && attacks & e_p_bitboard != 0 {
                if !self
                    .en_passant_exposes_king(initial_pos, en_passant_pawn, color, king_sq)
                    .0
                    && check_restrictions & (1 << en_passant_pawn) != 0
                {
                    dest_bitboard |= e_p_bitboard;
                }
            }
            if exposes_king {
                dest_bitboard &= allowed_squares;
            }
            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let piece_move: u16 = initial_pos | (final_pos << TO_SHIFT);

                if promo_rank & (1 << final_pos) != 0 {
                    let queen_promo_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        0b0110,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    let rook_promo_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        0b0101,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    let bishop_promo_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        0b0100,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    let knight_promo_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        0b0011,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    if captures_checks_only {
                        if self.cached_pieces[final_pos as usize] != 0 {
                            if queen_promo_check > 6 {
                                moves.push(piece_move | (queen_promo_check << MARK_SHIFT));
                            }
                            if rook_promo_check > 5 {
                                moves.push(piece_move | (rook_promo_check << MARK_SHIFT));
                            }
                            if bishop_promo_check > 4 {
                                moves.push(piece_move | (bishop_promo_check << MARK_SHIFT));
                            }
                            if knight_promo_check > 3 {
                                moves.push(piece_move | (knight_promo_check << MARK_SHIFT));
                            }
                        }
                        dest_bitboard &= dest_bitboard - 1;
                        continue;
                    }
                    moves.push(piece_move | (queen_promo_check << MARK_SHIFT));
                    moves.push(piece_move | (rook_promo_check << MARK_SHIFT));
                    moves.push(piece_move | (bishop_promo_check << MARK_SHIFT));
                    moves.push(piece_move | (knight_promo_check << MARK_SHIFT));
                } else if final_pos == en_passant {
                    let ep_promo_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        2,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    if captures_checks_only {
                        if self.cached_pieces[final_pos as usize] == 0 && ep_promo_check == 2 {
                            dest_bitboard &= dest_bitboard - 1;
                            continue;
                        }
                    }
                    moves.push(piece_move | (ep_promo_check << MARK_SHIFT));
                } else {
                    let regular_move_check: u16 = self.check_info(
                        initial_pos,
                        final_pos,
                        0,
                        king_sq_idx,
                        opposite_color,
                        moving_piece,
                        &state.check_squares,
                    );
                    if captures_checks_only {
                        if self.cached_pieces[final_pos as usize] == 0 && regular_move_check == 0 {
                            dest_bitboard &= dest_bitboard - 1;
                            continue;
                        }
                    }
                    moves.push(piece_move | (regular_move_check << MARK_SHIFT));
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
            defending_king,
        ) = match by {
            8 => (
                &self.bitboards[0],
                &BLACK_PAWN_ATTACKS[usize_square],
                &self.bitboards[1],
                &(self.bitboards[2] | w_q),
                &(self.bitboards[3] | w_q),
                &self.bitboards[5],
                self.bitboards[11],
            ),
            _ => (
                &self.bitboards[6],
                &WHITE_PAWN_ATTACKS[usize_square],
                &self.bitboards[7],
                &(self.bitboards[8] | b_q),
                &(self.bitboards[9] | b_q),
                &self.bitboards[11],
                self.bitboards[5],
            ),
        };
        let total_occ: u64 = self.total_occupancy & !defending_king; // KING IS NOT A DEFENDER!
        if (KNIGHT_ATTACKS[usize_square] & attacking_knights != 0)
            | (attacking_pawns & pawn_attacks != 0)
            | (bishop_attacks(usize_square, total_occ) & diagonal_attackers != 0)
            | (rook_attacks(usize_square, total_occ) & linear_attackers != 0)
            | (KING_ATTACKS[usize_square] & attacking_king != 0)
        {
            return true;
        }
        return false;
    }

    #[inline(always)]
    pub fn type_of_check(&self, from: u16, to: u16, king_square: u8, king_color: u16) -> u8 {
        // check in CURRENT position
        let king_sq = king_square as usize;
        let occ = (self.total_occupancy & !(1 << from)) & (1 << to);
        let (linear_enemies, diagonal_enemies) = if king_color == 16 {
            (
                (self.bitboards[3] | self.bitboards[4]) & rook_attacks(king_sq, occ),
                (self.bitboards[2] | self.bitboards[4]) & bishop_attacks(king_sq, occ),
            )
        } else {
            (
                (self.bitboards[9] | self.bitboards[10]) & rook_attacks(king_sq, occ),
                (self.bitboards[8] | self.bitboards[10]) & bishop_attacks(king_sq, occ),
            )
        };
        return match (
            diagonal_enemies.trailing_zeros(),
            linear_enemies.trailing_zeros(),
        ) {
            (64, 64) => 0,
            (_, 64) => 7,
            (64, _) => 7,
            _ => 8,
        };
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
            if self.is_square_attacked(final_pos as u8, opposite_color) {
                dest_bitboard &= dest_bitboard - 1;
                continue;
            }
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
    pub fn rook_moves(
        &self,
        color: u16,
        moves: &mut MoveList,
        state: &GameState,
        captures_checks_only: bool,
    ) -> () {
        let (
            mut rooks_bitboard,
            friendly_occupancy,
            king_sq,
            check_restrictions,
            opposite_color,
            moving_piece,
        ): (u64, u64, u8, &u64, u16, u16) = match color {
            8 => (
                self.bitboards[3],
                self.occupancies[0],
                self.white_king_square,
                &state.white_legal_squares_mask,
                16,
                WHITE_ROOK_U16,
            ),
            _ => (
                self.bitboards[9],
                self.occupancies[1],
                self.black_king_square,
                &state.black_legal_squares_mask,
                8,
                BLACK_ROOK_U16,
            ),
        };

        let king_sq_idx: usize = king_sq as usize;

        while rooks_bitboard != 0 {
            let initial_pos: u16 = rooks_bitboard.trailing_zeros() as u16;
            let attacks: u64 = rook_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: u64 = attacks & !friendly_occupancy & check_restrictions;
            let (exposes_king, allowed_squares) = self.exposes_king(initial_pos, king_sq, color);
            if exposes_king {
                dest_bitboard &= allowed_squares;
            }
            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let check_flag: u16 = self.check_info(
                    initial_pos,
                    final_pos,
                    0,
                    king_sq_idx,
                    opposite_color,
                    moving_piece,
                    &state.check_squares,
                );
                if captures_checks_only
                    && check_flag == 0
                    && self.cached_pieces[final_pos as usize] == 0
                {
                    dest_bitboard &= dest_bitboard - 1;
                    continue;
                }
                moves.push(initial_pos | (final_pos << TO_SHIFT) | (check_flag << MARK_SHIFT));

                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn bishop_moves(
        &self,
        color: u16,
        moves: &mut MoveList,
        state: &GameState,
        captures_checks_only: bool,
    ) -> () {
        let (
            mut bishops_bitboard,
            friendly_occupancy,
            king_sq,
            check_restrictions,
            opposite_color,
            moving_piece,
        ): (u64, u64, u8, &u64, u16, u16) = match color {
            8 => (
                self.bitboards[2],
                self.occupancies[0],
                self.white_king_square,
                &state.white_legal_squares_mask,
                16,
                WHITE_BISHOP_U16,
            ),
            _ => (
                self.bitboards[8],
                self.occupancies[1],
                self.black_king_square,
                &state.black_legal_squares_mask,
                8,
                BLACK_BISHOP_U16,
            ),
        };
        let king_sq_idx: usize = king_sq as usize;

        while bishops_bitboard != 0 {
            let initial_pos: u16 = bishops_bitboard.trailing_zeros() as u16;
            let attacks: u64 = bishop_attacks(initial_pos as usize, self.total_occupancy);
            let mut dest_bitboard: u64 = attacks & !friendly_occupancy & check_restrictions;
            let (exposes_king, allowed_squares) = self.exposes_king(initial_pos, king_sq, color);
            if exposes_king {
                dest_bitboard &= allowed_squares;
            }
            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let check_flag: u16 = self.check_info(
                    initial_pos,
                    final_pos,
                    0,
                    king_sq_idx,
                    opposite_color,
                    moving_piece,
                    &state.check_squares,
                );
                if captures_checks_only
                    && check_flag == 0
                    && self.cached_pieces[final_pos as usize] == 0
                {
                    dest_bitboard &= dest_bitboard - 1;
                    continue;
                }
                moves.push(initial_pos | (final_pos << TO_SHIFT) | (check_flag << MARK_SHIFT));
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }
    }

    #[inline(always)]
    pub fn queen_moves(
        &self,
        color: u16,
        moves: &mut MoveList,
        state: &GameState,
        captures_checks_only: bool,
    ) -> () {
        let (
            mut queens_bitboard,
            friendly_occupancy,
            king_sq,
            check_restrictions,
            opposite_color,
            moving_piece,
        ): (u64, u64, u8, &u64, u16, u16) = match color {
            8 => (
                self.bitboards[4],
                self.occupancies[0],
                self.white_king_square,
                &state.white_legal_squares_mask,
                16,
                WHITE_QUEEN_U16,
            ),
            _ => (
                self.bitboards[10],
                self.occupancies[1],
                self.black_king_square,
                &state.black_legal_squares_mask,
                8,
                BLACK_QUEEN_U16,
            ),
        };
        let king_sq_idx: usize = king_sq as usize;

        while queens_bitboard != 0 {
            let initial_pos: u16 = queens_bitboard.trailing_zeros() as u16;
            let initial_pos_as_index: usize = initial_pos as usize;
            let attacks: u64 = bishop_attacks(initial_pos_as_index, self.total_occupancy)
                | rook_attacks(initial_pos_as_index, self.total_occupancy);

            let mut dest_bitboard: u64 = attacks & !friendly_occupancy & check_restrictions;

            let (exposes_king, allowed_squares) = self.exposes_king(initial_pos, king_sq, color);
            if exposes_king {
                dest_bitboard &= allowed_squares;
            }

            while dest_bitboard != 0 {
                let final_pos: u16 = dest_bitboard.trailing_zeros() as u16;
                let check_flag: u16 = self.check_info(
                    initial_pos,
                    final_pos,
                    0,
                    king_sq_idx,
                    opposite_color,
                    moving_piece,
                    &state.check_squares,
                );
                if captures_checks_only
                    && check_flag == 0
                    && self.cached_pieces[final_pos as usize] == 0
                {
                    dest_bitboard &= dest_bitboard - 1;
                    continue;
                }
                moves.push(initial_pos | (final_pos << TO_SHIFT));
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }
    }
}
