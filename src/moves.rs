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
        let mut moves: Bitboard = 0;
        let blockers: &Bitboard = &self.total_occupancy.unwrap();

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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }

    pub fn diagonal_destinations(&self, square: u8) -> Bitboard {
        let mut moves: Bitboard = 0;
        let blockers: &Bitboard = &self.total_occupancy.unwrap();

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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
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
                PieceColor::Black => self.black_occupancy.unwrap(),
                PieceColor::White => self.white_occupancy.unwrap(),
            };
    }
    pub fn knight_destinations(&self, color: &PieceColor) -> Bitboard {
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

    pub fn knight_moves(&self, state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }
        let mut moves: Vec<PieceMove> = Vec::new();
        let (mut knights_bitboard, pinned_pieces) = match color {
            PieceColor::Black => (self.black_knights, &state.pin_info.black_pinned_pieces),
            PieceColor::White => (self.white_knights, &state.pin_info.white_pinned_pieces),
        };

        while knights_bitboard != 0 {
            let initial_pos: u8 = knights_bitboard.trailing_zeros() as u8;

            if pinned_pieces.contains(&initial_pos) {
                continue;
            }

            let attacks: Bitboard = KNIGHT_ATTACKS[initial_pos as usize];
            let mut dest_bitboard: Bitboard = attacks
                & !match color {
                    PieceColor::White => self.white_occupancy.unwrap(),
                    PieceColor::Black => self.black_occupancy.unwrap(),
                };
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }

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

    pub fn pawn_moves(&self, state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }
        let mut moves: Vec<PieceMove> = Vec::new();
        let mut pawns_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_pawns,
            PieceColor::White => self.white_pawns,
        };

        let mut enemy_occupancy: Bitboard = match color {
            PieceColor::White => self.black_occupancy.unwrap(),
            PieceColor::Black => self.white_occupancy.unwrap(),
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
            let mut dest_bitboard: Bitboard = attacks & enemy_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
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

    pub fn king_moves(&self, _state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        let mut moves: Vec<PieceMove> = Vec::new();
        let king_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_king,
            PieceColor::White => self.white_king,
        };

        let initial_pos: u8 = king_bitboard.trailing_zeros() as u8;
        let opponent_attacks: Bitboard = match color {
            PieceColor::Black => {
                self.bishop_destinations(&PieceColor::White)
                    | self.rook_destinations(&PieceColor::White)
                    | self.knight_destinations(&PieceColor::White)
                    | self.pawn_destintions(&PieceColor::White)
                    | self.king_destinations(&PieceColor::White)
            }
            PieceColor::White => {
                self.bishop_destinations(&PieceColor::Black)
                    | self.rook_destinations(&PieceColor::Black)
                    | self.knight_destinations(&PieceColor::Black)
                    | self.pawn_destintions(&PieceColor::Black)
                    | self.king_destinations(&PieceColor::Black)
            }
        };
        let mut dest_bitboard: Bitboard = KING_ATTACKS[initial_pos as usize]
            & !match color {
                PieceColor::White => self.white_occupancy.unwrap(),
                PieceColor::Black => self.black_occupancy.unwrap(),
            }
            & !opponent_attacks;

        while dest_bitboard != 0 {
            let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
            moves.push(PieceMove {
                from: initial_pos,
                to: final_pos,
            });
            dest_bitboard &= dest_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn rook_moves(&self, state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
        if let Some(_checked_king) = state.check_info.checked_king {
            match (
                state.check_info.first_checker,
                state.check_info.second_checker,
            ) {
                (Some(_c), None) => (),
                (Some(_), Some(_)) => return None,
                _ => unreachable!(),
            };
        }

        let mut moves: Vec<PieceMove> = Vec::new();
        let mut rooks_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_rooks,
            PieceColor::White => self.white_rooks,
        };

        let occupancy: Bitboard = self.total_occupancy.unwrap();
        let friendly_occupancy: Bitboard = match color {
            PieceColor::White => self.white_occupancy.unwrap(),
            PieceColor::Black => self.black_occupancy.unwrap(),
        };

        while rooks_bitboard != 0 {
            let initial_pos: usize = rooks_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard = rook_attacks(initial_pos, occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }

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

    pub fn bishop_moves(&self, state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
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
        let mut bishops_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_bishops,
            PieceColor::White => self.white_bishops,
        };

        let occupancy: Bitboard = self.total_occupancy.unwrap();
        let friendly_occupancy: Bitboard = match color {
            PieceColor::White => self.white_occupancy.unwrap(),
            PieceColor::Black => self.black_occupancy.unwrap(),
        };

        while bishops_bitboard != 0 {
            let initial_pos: usize = bishops_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard = bishop_attacks(initial_pos, occupancy);
            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos as u8,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }

        return Some(moves);
    }

    pub fn queen_moves(&self, state: &GameState, color: &PieceColor) -> Option<Vec<PieceMove>> {
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
        let mut queens_bitboard: Bitboard = match color {
            PieceColor::Black => self.black_queens,
            PieceColor::White => self.white_queens,
        };

        let occupancy: Bitboard = self.total_occupancy.unwrap();
        let friendly_occupancy: Bitboard = match color {
            PieceColor::White => self.white_occupancy.unwrap(),
            PieceColor::Black => self.black_occupancy.unwrap(),
        };

        while queens_bitboard != 0 {
            let initial_pos: usize = queens_bitboard.trailing_zeros() as usize;
            let attacks: Bitboard =
                bishop_attacks(initial_pos, occupancy) | rook_attacks(initial_pos, occupancy);

            let mut dest_bitboard: Bitboard = attacks & !friendly_occupancy;
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                moves.push(PieceMove {
                    from: initial_pos as u8,
                    to: final_pos,
                });
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }

        return Some(moves);
    }
}
