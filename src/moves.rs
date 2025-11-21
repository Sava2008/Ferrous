use crate::board::Board;
use crate::board_geometry_templates::*;
use crate::enums::PieceColor;

impl Board {
    pub fn white_pawn_moves(&self) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();
        let enemies: &Bitboard = &self.black_occupancy.unwrap();
        return ((!RANK_8 & self.white_pawns) >> 8 & empty)
            | ((self.white_pawns & RANK_2) >> 16 & empty & (empty >> 8))
            | ((!RANK_8 & self.white_pawns & !FILE_A) >> 9 & enemies)
            | ((!RANK_8 & self.white_pawns & !FILE_H) >> 7 & enemies);
    }

    pub fn black_pawn_moves(&self) -> Bitboard {
        let empty: &Bitboard = &!self.total_occupancy.unwrap();
        let enemies: &Bitboard = &self.white_occupancy.unwrap();
        return ((!RANK_1 & self.black_pawns) << 8 & empty)
            | ((self.black_pawns & RANK_7) << 16 & empty & (empty << 8))
            | ((!RANK_1 & self.black_pawns & !FILE_A) << 9 & enemies)
            | ((!RANK_1 & self.black_pawns & !FILE_H) << 7 & enemies);
    }

    pub fn knight_moves(&self, color: PieceColor) -> Bitboard {
        let (not_teammates, map) = match color {
            PieceColor::White => (&!self.white_occupancy.unwrap(), &self.white_knights),
            PieceColor::Black => (&!self.black_occupancy.unwrap(), &self.black_knights),
        };

        return ((map & !(FILE_G | FILE_H | RANK_1)) << 6 & not_teammates)
            | ((map & !(FILE_A | FILE_B | RANK_8)) >> 6 & not_teammates)
            | ((map & !(FILE_A | FILE_B | RANK_1)) << 10 & not_teammates)
            | ((map & !(FILE_G | FILE_H | RANK_8)) >> 10 & not_teammates)
            | ((map & !(FILE_H | RANK_1 | RANK_2)) << 15 & not_teammates)
            | ((map & !(FILE_A | RANK_7 | RANK_8)) >> 15 & not_teammates)
            | ((map & !(FILE_A | RANK_1 | RANK_2)) << 17 & not_teammates)
            | ((map & !(FILE_H | RANK_7 | RANK_8)) >> 17 & not_teammates);
    }

    pub fn bishop_moves(&self) -> Bitboard {
        todo!();
    }

    pub fn rook_moves(&self) -> Bitboard {
        todo!();
    }
}
