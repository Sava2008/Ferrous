use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::enums::PieceColor;
use crate::gamestate::{GameState, PieceMove, PinnedPiece};

impl Board {
    pub fn pawn_destintions(&self, color: &PieceColor) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy;

        return match color {
            PieceColor::White => {
                ((!RANK_8 & self.white_pawns) >> 8 & empty)
                    | ((self.white_pawns & RANK_2) >> 16 & empty & (empty >> 8))
            }
            PieceColor::Black => {
                ((!RANK_1 & self.black_pawns) << 8 & empty)
                    | ((self.black_pawns & RANK_2) << 16 & empty & (empty << 8))
            }
        };
    }

    pub fn linear_destinations(&self, square: u8) -> Bitboard {
        let mut moves: Bitboard = 0;
        let blockers: &Bitboard = &self.total_occupancy;

        let mut ray: Bitboard = 1 << square;
        while ray & !RANK_8 != 0 {
            ray <<= 8;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: Bitboard = 1 << square;
        while ray & !RANK_1 != 0 {
            ray >>= 8;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: Bitboard = 1 << square;
        while ray & !FILE_H != 0 {
            ray <<= 1;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: Bitboard = 1 << square;
        while ray & !FILE_A != 0 {
            ray >>= 1;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        return moves;
    }

    pub fn rook_destinations(&self, color: &PieceColor) -> Bitboard {
        let mut all_moves: Bitboard = 0;
        let mut rooks: Bitboard = match color {
            PieceColor::White => self.white_rooks,
            PieceColor::Black => self.black_rooks,
        };
        while rooks != 0 {
            let square: u8 = rooks.trailing_zeros() as u8;
            all_moves |= self.linear_destinations(square);
            rooks &= rooks - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy,
                PieceColor::White => self.white_occupancy,
            };
    }

    pub fn diagonal_destinations(&self, square: u8) -> Bitboard {
        let mut moves: Bitboard = 0;
        let blockers: &Bitboard = &self.total_occupancy;

        let mut ray: Bitboard = 1 << square;
        while ray & !(FILE_H | RANK_8) != 0 {
            ray <<= 9;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !(FILE_A | RANK_8) != 0 {
            ray <<= 7;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !(FILE_A | RANK_1) != 0 {
            ray >>= 9;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !(FILE_H | RANK_1) != 0 {
            ray >>= 7;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        return moves;
    }

    pub fn bishop_destinations(&self, color: &PieceColor) -> Bitboard {
        let mut all_moves: Bitboard = 0;
        let mut bishops: Bitboard = match color {
            PieceColor::White => self.white_bishops,
            PieceColor::Black => self.black_bishops,
        };
        while bishops != 0 {
            let square: u8 = bishops.trailing_zeros() as u8;
            all_moves |= self.diagonal_destinations(square);
            bishops &= bishops - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy,
                PieceColor::White => self.white_occupancy,
            };
    }

    pub fn queen_destinations(&self, color: &PieceColor) -> Bitboard {
        let mut all_moves: Bitboard = 0;
        let mut queens: Bitboard = match color {
            PieceColor::White => self.white_queens,
            PieceColor::Black => self.black_queens,
        };
        while queens != 0 {
            let square: u8 = queens.trailing_zeros() as u8;
            all_moves |= self.linear_destinations(square);
            all_moves |= self.diagonal_destinations(square);
            queens &= queens - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy,
                PieceColor::White => self.white_occupancy,
            };
    }
    pub fn knight_destinations(&self, color: &PieceColor) -> Bitboard {
        let (not_teammates, map) = match color {
            PieceColor::White => (&!self.white_occupancy, &self.white_knights),
            PieceColor::Black => (&!self.black_occupancy, &self.black_knights),
        };

        return ((map & !(FILE_G | FILE_H | RANK_1)) >> 6 & not_teammates)
            | ((map & !(FILE_A | FILE_B | RANK_8)) << 6 & not_teammates)
            | ((map & !(FILE_A | FILE_B | RANK_1)) >> 10 & not_teammates)
            | ((map & !(FILE_G | FILE_H | RANK_8)) << 10 & not_teammates)
            | ((map & !(FILE_H | RANK_1 | RANK_2)) >> 15 & not_teammates)
            | ((map & !(FILE_A | RANK_7 | RANK_8)) << 15 & not_teammates)
            | ((map & !(FILE_A | RANK_1 | RANK_2)) >> 17 & not_teammates)
            | ((map & !(FILE_H | RANK_7 | RANK_8)) << 17 & not_teammates);
    }

    pub fn king_destinations(&self, color: &PieceColor) -> Bitboard {
        let (not_teammates, map) = match color {
            PieceColor::White => (&!self.white_occupancy, &self.white_king),
            PieceColor::Black => (&!self.black_occupancy, &self.black_king),
        };

        return ((map & !FILE_H) << 1 & not_teammates)
            | ((map & !RANK_8) << 8 & not_teammates)
            | ((map & !RANK_1) >> 8 & not_teammates)
            | ((map & !FILE_A) >> 1 & not_teammates)
            | ((map & !(FILE_H | RANK_8)) << 9 & not_teammates)
            | ((map & !(FILE_A | RANK_1)) >> 9 & not_teammates)
            | ((map & !(FILE_A | RANK_8)) << 7 & not_teammates)
            | ((map & !(FILE_H | RANK_1)) >> 7 & not_teammates);
    }

    pub fn knight_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        let enemy_king: u8 = match color {
            &PieceColor::White => state.pin_info.black_king,
            &PieceColor::Black => state.pin_info.white_king,
        };

        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return moves,
                _ => unreachable!(),
            };
        }

        let (mut knights_bitboard, pinned_pieces): (Bitboard, &Vec<PinnedPiece>) = match color {
            PieceColor::Black => (self.black_knights, &state.pin_info.black_pinned_pieces),
            PieceColor::White => (self.white_knights, &state.pin_info.white_pinned_pieces),
        };

        while knights_bitboard != 0 {
            let initial_pos: u8 = knights_bitboard.trailing_zeros() as u8;

            if pinned_pieces
                .iter()
                .any(|p_p: &PinnedPiece| p_p.square == initial_pos)
            {
                knights_bitboard &= knights_bitboard - 1;
                continue;
            }

            let attacks: Bitboard = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: Bitboard = attacks
                & !match color {
                    PieceColor::White => self.white_occupancy,
                    PieceColor::Black => self.black_occupancy,
                };
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push(PieceMove {
                        from: initial_pos,
                        to: final_pos,
                    });
                }
                dest_bitboard &= dest_bitboard - 1;
            }
            knights_bitboard &= knights_bitboard - 1;
        }
        return moves;
    }

    pub fn pawn_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return moves,
                _ => unreachable!(),
            };
        }

        let (enemy_king, pinned_pieces, mut pawns_bitboard, mut enemy_occupancy): (
            u8,
            &Vec<PinnedPiece>,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                &state.pin_info.white_pinned_pieces,
                self.white_pawns,
                self.black_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                &state.pin_info.black_pinned_pieces,
                self.black_pawns,
                self.white_occupancy,
            ),
        };

        if let Some(e_p) = state.en_passant_target {
            enemy_occupancy |= 1 << e_p
        }

        while pawns_bitboard != 0 {
            let initial_pos: u8 = pawns_bitboard.trailing_zeros() as u8;
            let forward_square: u8 = match color {
                &PieceColor::Black => initial_pos.wrapping_sub(8),
                &PieceColor::White => initial_pos + 8,
            };
            let attacks: Bitboard = match color {
                PieceColor::White => WHITE_PAWN_ATTACKS[initial_pos as usize],
                PieceColor::Black => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: Bitboard = attacks & enemy_occupancy;
            let pin_ray: Option<u64> = pinned_pieces
                .iter()
                .find(|p: &&PinnedPiece| p.square == initial_pos)
                .map(|p: &PinnedPiece| p.pin_ray);

            if forward_square < 64 && (self.total_occupancy >> forward_square) & 1 == 0 {
                dest_bitboard |= 1 << forward_square;
                let second_forward_square: u8 = match color {
                    &PieceColor::Black => initial_pos.wrapping_sub(16),
                    &PieceColor::White => initial_pos + 16,
                };
                if match color {
                    PieceColor::Black => (1 << initial_pos) & RANK_7 != 0,
                    PieceColor::White => (1 << initial_pos) & RANK_2 != 0,
                } && second_forward_square < 64
                    && (self.total_occupancy >> second_forward_square) & 1 == 0
                {
                    dest_bitboard |= 1 << second_forward_square;
                }
            }
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
            if let Some(ray) = pin_ray {
                dest_bitboard &= ray;
            }
            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push(PieceMove {
                        from: initial_pos,
                        to: final_pos,
                    });
                }
                dest_bitboard &= dest_bitboard - 1;
            }
            pawns_bitboard &= pawns_bitboard - 1;
        }
        return moves;
    }

    pub fn is_square_attacked(&self, square: u8, by: &PieceColor) -> bool {
        if KNIGHT_ATTACKS[square as usize]
            & match by {
                PieceColor::White => self.white_knights,
                PieceColor::Black => self.black_knights,
            }
            != 0
        {
            return true;
        }

        let pawn_attacks: Bitboard = match by {
            PieceColor::White => BLACK_PAWN_ATTACKS[square as usize],
            PieceColor::Black => WHITE_PAWN_ATTACKS[square as usize],
        };
        if pawn_attacks
            & match by {
                PieceColor::White => self.white_pawns,
                PieceColor::Black => self.black_pawns,
            }
            != 0
        {
            return true;
        }

        if bishop_attacks(square as usize, self.total_occupancy)
            & match by {
                PieceColor::White => self.white_bishops | self.white_queens,
                PieceColor::Black => self.black_bishops | self.black_queens,
            }
            != 0
        {
            return true;
        }

        if rook_attacks(square as usize, self.total_occupancy)
            & match by {
                PieceColor::White => self.white_rooks | self.white_queens,
                PieceColor::Black => self.black_rooks | self.black_queens,
            }
            != 0
        {
            return true;
        }

        if KING_ATTACKS[square as usize]
            & match by {
                PieceColor::White => self.white_king,
                PieceColor::Black => self.black_king,
            }
            != 0
        {
            return true;
        }

        return false;
    }

    pub fn king_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        let initial_pos: u8 = match color {
            &PieceColor::White => self.white_king,
            &PieceColor::Black => self.black_king,
        }
        .trailing_zeros() as u8;
        let mut dest_bitboard: Bitboard = KING_ATTACKS[initial_pos as usize]
            & !match color {
                PieceColor::White => self.white_occupancy,
                PieceColor::Black => self.black_occupancy,
            };

        while dest_bitboard != 0 {
            let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
            if !self.is_square_attacked(final_pos, &!color.clone()) {
                moves.push(PieceMove {
                    from: initial_pos,
                    to: final_pos,
                });
            }
            dest_bitboard &= dest_bitboard - 1;
        }
        if Some(initial_pos) == state.check_info.checked_king {
            return moves;
        }
        let (castling_squares, mut right_path, mut left_path): (
            (Option<u8>, Option<u8>),
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => match (
                &state.castling_rights.white_three_zeros,
                &state.castling_rights.white_two_zeros,
            ) {
                (true, true) => (
                    (Some(2), Some(6)),
                    0b0000000000000000000000000000000000000000000000000000000000001110,
                    0b0000000000000000000000000000000000000000000000000000000001100000,
                ),
                (false, false) => return moves,
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
            &PieceColor::Black => match (
                &state.castling_rights.black_three_zeros,
                &state.castling_rights.black_two_zeros,
            ) {
                (true, true) => (
                    (Some(62), Some(58)),
                    0b0110000000000000000000000000000000000000000000000000000000000000,
                    0b0000111000000000000000000000000000000000000000000000000000000000,
                ),
                (false, false) => return moves,
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
        if left_path != 0 && (left_path & self.total_occupancy == 0) {
            while left_path != 0 {
                if self.is_square_attacked(left_path.trailing_zeros() as u8, &!color.clone()) {
                    break;
                }
                left_path &= left_path - 1;
            }
            if left_path == 0
                && let Some(sq) = castling_squares.0
            {
                moves.push(PieceMove {
                    from: initial_pos,
                    to: sq,
                })
            }
        }
        if right_path != 0 && (right_path & self.total_occupancy == 0) {
            while right_path != 0 {
                if self.is_square_attacked(right_path.trailing_zeros() as u8, &!color.clone()) {
                    break;
                }
                right_path &= right_path - 1;
            }
            if right_path == 0
                && let Some(sq) = castling_squares.1
            {
                moves.push(PieceMove {
                    from: initial_pos,
                    to: sq,
                })
            }
        }

        return moves;
    }

    pub fn rook_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return moves,
                _ => unreachable!(),
            };
        }

        let (enemy_king, pinned_pieces, mut rooks_bitboard, friendly_occupancy): (
            u8,
            &Vec<PinnedPiece>,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                &state.pin_info.white_pinned_pieces,
                self.white_rooks,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                &state.pin_info.black_pinned_pieces,
                self.black_rooks,
                self.black_occupancy,
            ),
        };

        let occupancy: Bitboard = self.total_occupancy;

        while rooks_bitboard != 0 {
            let initial_pos: usize = rooks_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard = rook_attacks(initial_pos, occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
            let pin_ray: Option<u64> = pinned_pieces
                .iter()
                .find(|p: &&PinnedPiece| p.square as usize == initial_pos)
                .map(|p: &PinnedPiece| p.pin_ray);

            if let Some(ray) = pin_ray {
                dest_bitboard &= ray;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push(PieceMove {
                        from: initial_pos as u8,
                        to: final_pos,
                    });
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }

        return moves;
    }

    pub fn bishop_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return moves,
                _ => unreachable!(),
            };
        }

        let (enemy_king, pinned_pieces, mut bishops_bitboard, friendly_occupancy): (
            u8,
            &Vec<PinnedPiece>,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                &state.pin_info.white_pinned_pieces,
                self.white_bishops,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                &state.pin_info.black_pinned_pieces,
                self.black_bishops,
                self.black_occupancy,
            ),
        };
        let occupancy: Bitboard = self.total_occupancy;

        while bishops_bitboard != 0 {
            let initial_pos: usize = bishops_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard = bishop_attacks(initial_pos, occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
            let pin_ray: Option<u64> = pinned_pieces
                .iter()
                .find(|p: &&PinnedPiece| p.square as usize == initial_pos)
                .map(|p: &PinnedPiece| p.pin_ray);

            if let Some(ray) = pin_ray {
                dest_bitboard &= ray;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push(PieceMove {
                        from: initial_pos as u8,
                        to: final_pos,
                    });
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }
        return moves;
    }

    pub fn queen_moves(&self, state: &GameState, color: &PieceColor) -> Vec<PieceMove> {
        let mut moves: Vec<PieceMove> = Vec::new();
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return moves,
                _ => unreachable!(),
            };
        }

        let (enemy_king, pinned_pieces, mut queens_bitboard, friendly_occupancy): (
            u8,
            &Vec<PinnedPiece>,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                &state.pin_info.white_pinned_pieces,
                self.white_queens,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                &state.pin_info.black_pinned_pieces,
                self.black_queens,
                self.black_occupancy,
            ),
        };

        let occupancy: Bitboard = self.total_occupancy;

        while queens_bitboard != 0 {
            let initial_pos: usize = queens_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard =
                bishop_attacks(initial_pos, occupancy) | rook_attacks(initial_pos, occupancy);

            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
            let pin_ray: Option<u64> = pinned_pieces
                .iter()
                .find(|p: &&PinnedPiece| p.square as usize == initial_pos)
                .map(|p: &PinnedPiece| p.pin_ray);

            if let Some(ray) = pin_ray {
                dest_bitboard &= ray;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push(PieceMove {
                        from: initial_pos as u8,
                        to: final_pos,
                    });
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }
        return moves;
    }
}
