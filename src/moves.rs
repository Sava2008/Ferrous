use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::enums::PieceColor;

impl Board {
    pub fn white_pawn_moves(&self) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();
        let enemies: &Bitboard = &self.black_occupancy.unwrap();
        return ((!RANK_8 & self.white_pawns) << 8 & empty)
            | ((self.white_pawns & RANK_2) << 16 & empty & (empty << 8))
            | ((!RANK_8 & self.white_pawns & !FILE_A) << 9 & enemies)
            | ((!RANK_8 & self.white_pawns & !FILE_H) << 7 & enemies);
    }

    pub fn black_pawn_moves(&self) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();
        let enemies: &Bitboard = &self.white_occupancy.unwrap();
        return ((!RANK_1 & self.black_pawns) >> 8 & empty)
            | ((self.black_pawns & RANK_7) >> 16 & empty & (empty >> 8))
            | ((!RANK_1 & self.black_pawns & !FILE_A) >> 9 & enemies)
            | ((!RANK_1 & self.black_pawns & !FILE_H) >> 7 & enemies);
    }

    pub fn knight_moves(&self, color: PieceColor) -> Bitboard {
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

    fn linear_moves(&self, square: u8) -> Bitboard {
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

    pub fn rook_moves(&self, color: PieceColor) -> Bitboard {
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

        return all_moves;
    }

    fn diagonal_moves(&self, square: u8) -> Bitboard {
        let mut moves: u64 = 0;
        let blockers: &u64 = &self.total_occupancy.unwrap();

        let mut ray: u64 = 1 << square;
        while ray & !FILE_H != 0 {
            ray <<= 9;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !FILE_A != 0 {
            ray <<= 7;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !FILE_A != 0 {
            ray >>= 9;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        ray = 1 << square;
        while ray & !FILE_H != 0 {
            ray >>= 7;
            moves |= ray;
            if ray & blockers != 0 {
                break;
            }
        }

        return moves;
    }

    pub fn bishop_moves(&self, color: PieceColor) -> Bitboard {
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

    pub fn queen_moves(&self, color: PieceColor) -> Bitboard {
        let mut all_moves: u64 = 0;
        let mut queens: u64 = match color {
            PieceColor::White => self.white_queens,
            PieceColor::Black => self.black_queens,
        };
        while queens != 0 {
            let square: u8 = queens.trailing_zeros() as u8;
            all_moves |= self.diagonal_moves(square);
            all_moves |= self.linear_moves(square);
            queens &= queens - 1;
        }

        return all_moves;
    }
}
