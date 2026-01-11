use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::enums::PieceColor;
use crate::gamestate::{GameState, PieceMove};

impl Board {
    pub fn pawn_destintions(&self, color: &PieceColor) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();

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
        let mut moves: u64 = 0;
        let blockers: &u64 = &self.total_occupancy.unwrap();

        let mut ray: u64 = 1 << square;
        while ray & !RANK_8 != 0 {
            ray <<= 8;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: u64 = 1 << square;
        while ray & !RANK_1 != 0 {
            ray >>= 8;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: u64 = 1 << square;
        while ray & !FILE_H != 0 {
            ray <<= 1;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        let mut ray: u64 = 1 << square;
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
        let mut all_moves: u64 = 0;
        let mut rooks: u64 = match color {
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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn diagonal_destinations(&self, square: u8) -> Bitboard {
        let mut moves: u64 = 0;
        let blockers: &u64 = &self.total_occupancy.unwrap();

        let mut ray: u64 = 1 << square;
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
        let mut all_moves: u64 = 0;
        let mut bishops: u64 = match color {
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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn queen_destinations(&self, color: &PieceColor) -> Bitboard {
        let mut all_moves: u64 = 0;
        let mut queens: u64 = match color {
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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn king_destinations(&self, color: &PieceColor) -> Bitboard {
        let (not_teammates, map) = match color {
            PieceColor::White => (&!self.white_occupancy.unwrap(), &self.white_king),
            PieceColor::Black => (&!self.black_occupancy.unwrap(), &self.black_king),
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

    pub fn knight_moves(&self, state: GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => return None, // temporary solution
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }
        let mut moves: Vec<PieceMove> = Vec::new();
        let mut knights_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_knights,
            PieceColor::White => self.white_knights,
        };

        while knights_bitboard != 0 {
            let initial_pos: u8 = knights_bitboard.trailing_zeros() as u8;
            let attacks: Bitboard = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: Bitboard = attacks
                & !match color {
                    PieceColor::White => self.white_occupancy.unwrap(),
                    PieceColor::Black => self.black_occupancy.unwrap(),
                };

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }
            knights_bitboard &= knights_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn pawn_moves(&self, state: GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => return None, // temporary solution
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }
        let mut moves: Vec<PieceMove> = Vec::new();
        let mut pawns_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_pawns,
            PieceColor::White => self.white_pawns,
        };

        while pawns_bitboard != 0 {
            let initial_pos: u8 = pawns_bitboard.trailing_zeros() as u8;
            let forward_square: u8 = match color {
                &PieceColor::Black => initial_pos.wrapping_sub(8),
                &PieceColor::White => initial_pos + 8,
            };
            if forward_square < 64 && (self.total_occupancy.unwrap() >> forward_square) & 1 == 0 {
                moves.push(PieceMove {
                    from: initial_pos,
                    to: forward_square,
                });
                let second_forward_square: u8 = match color {
                    &PieceColor::Black => initial_pos.wrapping_sub(16),
                    &PieceColor::White => initial_pos + 16,
                };
                if match color {
                    PieceColor::Black => (initial_pos + 4) / 50,
                    PieceColor::White => (initial_pos + 4) / 10,
                } == 1
                    && second_forward_square < 64
                    && (self.total_occupancy.unwrap() >> second_forward_square) & 1 == 0
                {
                    moves.push(PieceMove {
                        from: initial_pos,
                        to: second_forward_square,
                    });
                }
            }
            let attacks: Bitboard = match color {
                PieceColor::White => WHITE_PAWN_ATTACKS[initial_pos as usize],
                PieceColor::Black => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: Bitboard = attacks
                & match color {
                    PieceColor::White => self.black_occupancy.unwrap(),
                    PieceColor::Black => self.white_occupancy.unwrap(),
                };
            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }
            pawns_bitboard &= pawns_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn king_moves(&self, _state: GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        let mut moves: Vec<PieceMove> = Vec::new();
        let mut king_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_king,
            PieceColor::White => self.white_king,
        };

        while king_bitboard != 0 {
            let initial_pos: u8 = king_bitboard.trailing_zeros() as u8;
            let attacks: Bitboard = KING_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: Bitboard = attacks
                & !match color {
                    PieceColor::White => self.white_occupancy.unwrap(),
                    PieceColor::Black => self.black_occupancy.unwrap(),
                };

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }
            king_bitboard &= king_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn rook_moves(&self, state: GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => return None, // temporary solution
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }

        let mut moves: Vec<PieceMove> = Vec::new();
        let mut rooks_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_rooks,
            PieceColor::White => self.white_rooks,
        };

        let occupancy = self.total_occupancy.unwrap();
        let friendly_occupancy = match color {
            PieceColor::White => self.white_occupancy.unwrap(),
            PieceColor::Black => self.black_occupancy.unwrap(),
        };

        while rooks_bitboard != 0 {
            let initial_pos: usize = rooks_bitboard.trailing_zeros() as usize;

            let masked_blockers = occupancy & ROOK_MASKS[initial_pos];
            let idx: usize = ((masked_blockers.wrapping_mul(ROOK_MAGICS[initial_pos]))
                >> ROOK_SHIFTS[initial_pos]) as usize;
            let offset: usize = ROOK_OFFSETS[initial_pos];

            let attacks: Bitboard = unsafe { ROOK_ATTACKS[offset + idx] };

            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos as u8,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }

        return Some(moves);
    }
}
