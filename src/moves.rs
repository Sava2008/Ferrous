use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::enums::PieceColor;
use crate::gamestate::{GameState, PieceMove};

impl Board {
    pub fn pawn_destinations(&self, color: PieceColor) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();
        let enemies: &Bitboard = match color {
            PieceColor::Black => &self.white_occupancy.unwrap(),
            PieceColor::White => &self.black_occupancy.unwrap(),
        };
        return match color {
            PieceColor::Black => {
                ((!RANK_1 & self.black_pawns) >> 8 & empty)
                    | ((self.black_pawns & RANK_7) >> 16 & empty & (empty >> 8))
                    | ((!RANK_1 & self.black_pawns & !FILE_A) >> 9 & enemies)
                    | ((!RANK_1 & self.black_pawns & !FILE_H) >> 7 & enemies)
            }
            PieceColor::White => {
                ((!RANK_8 & self.white_pawns) << 8 & empty)
                    | ((self.white_pawns & RANK_2) << 16 & empty & (empty << 8))
                    | ((!RANK_8 & self.white_pawns & !FILE_A) << 9 & enemies)
                    | ((!RANK_8 & self.white_pawns & !FILE_H) << 7 & enemies)
            }
        };
    }

    pub fn knight_destinations(&self, color: PieceColor) -> Bitboard {
        let (not_teammates, map) = match color {
            PieceColor::White => (&!self.white_occupancy.unwrap(), &self.white_knights),
            PieceColor::Black => (&!self.black_occupancy.unwrap(), &self.black_knights),
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

    pub fn linear_moves(&self, square: u8) -> Bitboard {
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

    pub fn rook_destinations(&self, color: PieceColor) -> Bitboard {
        let mut all_moves: u64 = 0;
        let mut rooks: u64 = match color {
            PieceColor::White => self.white_rooks,
            PieceColor::Black => self.black_rooks,
        };
        while rooks != 0 {
            let square: u8 = rooks.trailing_zeros() as u8;
            all_moves |= self.linear_moves(square);
            rooks &= rooks - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn diagonal_moves(&self, square: u8) -> Bitboard {
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

    pub fn bishop_destinations(&self, color: PieceColor) -> Bitboard {
        let mut all_moves: u64 = 0;
        let mut bishops: u64 = match color {
            PieceColor::White => self.white_bishops,
            PieceColor::Black => self.black_bishops,
        };
        while bishops != 0 {
            let square: u8 = bishops.trailing_zeros() as u8;
            all_moves |= self.diagonal_moves(square);
            bishops &= bishops - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn queen_destinations(&self, color: PieceColor) -> Bitboard {
        let mut all_moves: u64 = 0;
        let mut queens: u64 = match color {
            PieceColor::White => self.white_queens,
            PieceColor::Black => self.black_queens,
        };
        while queens != 0 {
            let square: u8 = queens.trailing_zeros() as u8;
            all_moves |= self.linear_moves(square);
            all_moves |= self.diagonal_moves(square);
            queens &= queens - 1;
        }

        return all_moves
            & !match color {
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn king_destinations(&self, color: PieceColor) -> Bitboard {
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

    pub fn knight_moves(&self, state: GameState, color: PieceColor) -> Option<Vec<PieceMove>> {
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
        let mut knights_bitboard: u64 = match color {
            PieceColor::Black => self.black_knights,
            PieceColor::White => self.white_knights,
        };

        while knights_bitboard != 0 {
            let initial_pos: u8 = knights_bitboard.trailing_zeros() as u8;
            let attacks: u64 = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: u64 = attacks
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

    pub fn pawn_moves(&self, state: GameState, color: PieceColor) -> Option<Vec<PieceMove>> {
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
        let mut pawns_bitboard: u64 = match color {
            PieceColor::Black => self.black_pawns,
            PieceColor::White => self.white_pawns,
        };

        while pawns_bitboard != 0 {
            let initial_pos: u8 = pawns_bitboard.trailing_zeros() as u8;
            let attacks: u64 = match color {
                PieceColor::White => WHITE_PAWN_ATTACKS[initial_pos as usize],
                PieceColor::Black => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: u64 = attacks
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
            pawns_bitboard &= pawns_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn king_moves(&self, _state: GameState, color: PieceColor) -> Option<Vec<PieceMove>> {
        let mut moves: Vec<PieceMove> = Vec::new();
        let mut king_bitboard: u64 = match color {
            PieceColor::Black => self.black_king,
            PieceColor::White => self.white_king,
        };

        while king_bitboard != 0 {
            let initial_pos: u8 = king_bitboard.trailing_zeros() as u8;
            let attacks: u64 = KING_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: u64 = attacks
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
}
