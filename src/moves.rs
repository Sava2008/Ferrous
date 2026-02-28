use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::constants::attacks::*;
use crate::enums::PieceColor;
use crate::gamestate::GameState;

impl Board {
    pub fn knight_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();

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
        let (enemy_king, mut knights_bitboard): (u8, Bitboard) = match color {
            &PieceColor::White => (state.pin_info.black_king, self.white_knights),
            &PieceColor::Black => (state.pin_info.white_king, self.black_knights),
        };

        while knights_bitboard != 0 {
            let initial_pos: u16 = knights_bitboard.trailing_zeros() as u16;

            if &(state.pin_info.pinned_pieces) & (1 << initial_pos) != 0 {
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
                    moves.push((initial_pos as u16) | ((final_pos as u16) << TO_SHIFT));
                }
                dest_bitboard &= dest_bitboard - 1;
            }
            knights_bitboard &= knights_bitboard - 1;
        }
        return moves;
    }

    pub fn pawn_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();
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

        let (enemy_king, friendly_king, mut pawns_bitboard, mut enemy_occupancy): (
            u8,
            usize,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                state.pin_info.white_king as usize,
                self.white_pawns,
                self.black_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                state.pin_info.black_king as usize,
                self.black_pawns,
                self.white_occupancy,
            ),
        };

        /*if let Some(e_p) = state.en_passant_target {
            enemy_occupancy |= 1 << e_p
        } // for testing*/

        while pawns_bitboard != 0 {
            let initial_pos: u16 = pawns_bitboard.trailing_zeros() as u16;
            let forward_square: u16 = match color {
                &PieceColor::Black => initial_pos.wrapping_sub(8),
                &PieceColor::White => initial_pos + 8,
            };
            let attacks: Bitboard = match color {
                PieceColor::White => WHITE_PAWN_ATTACKS[initial_pos as usize],
                PieceColor::Black => BLACK_PAWN_ATTACKS[initial_pos as usize],
            };
            let mut dest_bitboard: Bitboard = attacks & enemy_occupancy;

            if forward_square < 64 && (self.total_occupancy >> forward_square) & 1 == 0 {
                dest_bitboard |= 1 << forward_square;
                let second_forward_square: u16 = match color {
                    &PieceColor::Black => initial_pos.wrapping_sub(16),
                    &PieceColor::White => initial_pos + 16,
                };
                if match color {
                    PieceColor::Black => (1 << initial_pos) & RANK_7 != 0,
                    PieceColor::White => (1 << initial_pos) & RANK_2 != 0,
                } && (self.total_occupancy >> second_forward_square) & 1 == 0
                {
                    dest_bitboard |= 1 << second_forward_square;
                }
            }
            if state.check_contraints != 0 {
                dest_bitboard &= &state.check_contraints;
            }
            if (1 << initial_pos) & &state.pin_info.pinned_pieces != 0 {
                dest_bitboard &= unsafe { TWO_SQUARES_LINE[initial_pos as usize][friendly_king] };
            }
            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    let piece_move: u16 = (initial_pos as u16) | ((final_pos as u16) << TO_SHIFT);
                    if final_pos > 55 || final_pos < 8 {
                        moves.push(piece_move | (0b001 << PROMOTION_SHIFT));
                        moves.push(piece_move | (0b010 << PROMOTION_SHIFT));
                        moves.push(piece_move | (0b011 << PROMOTION_SHIFT));
                        moves.push(piece_move | (0b100 << PROMOTION_SHIFT));
                    } else {
                        moves.push(piece_move)
                    }
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
        let occupancy: Bitboard = self.total_occupancy & !(self.white_king | self.black_king);

        if bishop_attacks(square as usize, occupancy)
            & match by {
                PieceColor::White => self.white_bishops | self.white_queens,
                PieceColor::Black => self.black_bishops | self.black_queens,
            }
            != 0
        {
            return true;
        }

        if rook_attacks(square as usize, occupancy)
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

    pub fn king_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();
        let initial_pos: u16 = match color {
            &PieceColor::White => self.white_king,
            &PieceColor::Black => self.black_king,
        }
        .trailing_zeros() as u16;
        let mut dest_bitboard: Bitboard = KING_ATTACKS[initial_pos as usize]
            & !match color {
                PieceColor::White => self.white_occupancy,
                PieceColor::Black => self.black_occupancy,
            };

        while dest_bitboard != 0 {
            let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
            if !self.is_square_attacked(final_pos, &!color.clone()) {
                moves.push((initial_pos as u16) | ((final_pos as u16) << TO_SHIFT));
            }
            dest_bitboard &= dest_bitboard - 1;
        }
        if Some(initial_pos as u8) == state.check_info.checked_king {
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
                &state.castling_rights.black_two_zeros,
                &state.castling_rights.black_three_zeros,
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
            let mut finished_fully: bool = true;
            while left_path != 0 {
                let square: u8 = left_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, &!color.clone()) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                left_path &= left_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.1 {
                moves.push((initial_pos as u16) | ((sq as u16) << TO_SHIFT));
            }
        }
        if right_path != 0 && (right_path & self.total_occupancy == 0) {
            let mut finished_fully: bool = true;
            while right_path != 0 {
                let square: u8 = right_path.trailing_zeros() as u8;
                if self.is_square_attacked(square, &!color.clone()) && ((1 << square) & FILE_B == 0)
                {
                    finished_fully = false;
                    break;
                }
                right_path &= right_path - 1;
            }
            if finished_fully && let Some(sq) = castling_squares.0 {
                moves.push((initial_pos as u16) | ((sq as u16) << TO_SHIFT));
            }
        }

        return moves;
    }

    pub fn rook_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();
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

        let (enemy_king, friendly_king, mut rooks_bitboard, friendly_occupancy): (
            u8,
            usize,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                state.pin_info.white_king as usize,
                self.white_rooks,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                state.pin_info.black_king as usize,
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

            if (1 << initial_pos) & &state.pin_info.pinned_pieces != 0 {
                dest_bitboard &= unsafe { TWO_SQUARES_LINE[initial_pos as usize][friendly_king] };
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push((initial_pos as u16) | ((final_pos as u16) << TO_SHIFT));
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            rooks_bitboard &= rooks_bitboard - 1;
        }

        return moves;
    }

    pub fn bishop_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();
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
        let (enemy_king, friendly_king, mut bishops_bitboard, friendly_occupancy): (
            u8,
            usize,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                state.pin_info.white_king as usize,
                self.white_bishops,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                state.pin_info.black_king as usize,
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

            if (1 << initial_pos) & &state.pin_info.pinned_pieces != 0 {
                dest_bitboard &= unsafe { TWO_SQUARES_LINE[initial_pos as usize][friendly_king] };
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push((initial_pos as u16) | ((final_pos as u16) << TO_SHIFT));
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            bishops_bitboard &= bishops_bitboard - 1;
        }
        return moves;
    }

    pub fn queen_moves(&self, state: &GameState, color: &PieceColor) -> Vec<u16> {
        let mut moves: Vec<u16> = Vec::new();
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

        let (enemy_king, friendly_king, mut queens_bitboard, friendly_occupancy): (
            u8,
            usize,
            Bitboard,
            Bitboard,
        ) = match color {
            &PieceColor::White => (
                state.pin_info.black_king,
                state.pin_info.white_king as usize,
                self.white_queens,
                self.white_occupancy,
            ),
            &PieceColor::Black => (
                state.pin_info.white_king,
                state.pin_info.black_king as usize,
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

            if (1 << initial_pos) & &state.pin_info.pinned_pieces != 0 {
                dest_bitboard &= unsafe { TWO_SQUARES_LINE[initial_pos as usize][friendly_king] };
            }

            while dest_bitboard != 0 {
                let final_pos: u8 = dest_bitboard.trailing_zeros() as u8;
                if final_pos != enemy_king {
                    moves.push((initial_pos as u16) | ((final_pos as u16) << TO_SHIFT));
                }
                dest_bitboard &= dest_bitboard - 1;
            }

            queens_bitboard &= queens_bitboard - 1;
        }
        return moves;
    }
}
