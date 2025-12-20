use crate::{board_geometry_templates::Bitboard, enums::PieceColor};

const KNIGHT_ATTACKS: [Bitboard; 64] = knight_attacks();

#[allow(unused)]
const fn knight_attacks() -> [Bitboard; 64] {
    let mut table: [u64; 64] = [0; 64];
    const OFFSETS: [(i8, i8); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    let mut square: usize = 0;
    while square < 64 {
        let file: i8 = (square % 8) as i8;
        let rank: i8 = (square / 8) as i8;
        let mut attacks: u64 = 0;

        let mut i: usize = 0;
        while i < 8 {
            let (file_offset, rank_offset) = OFFSETS[i];
            let new_file: i8 = file + file_offset;
            let new_rank: i8 = rank + rank_offset;

            if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                let target_sq: usize = (new_rank * 8 + new_file) as usize;
                attacks |= 1 << target_sq;
            }
            i += 1;
        }

        table[square] = attacks;
        square += 1;
    }

    return table;
}

const KING_ATTACKS: [Bitboard; 64] = king_attacks();

#[allow(unused)]
const fn king_attacks() -> [Bitboard; 64] {
    let mut table: [u64; 64] = [0; 64];
    const OFFSETS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut square: usize = 0;
    while square < 64 {
        let file: i8 = (square % 8) as i8;
        let rank: i8 = (square / 8) as i8;
        let mut attacks: u64 = 0;

        let mut i: usize = 0;
        while i < 8 {
            let (file_offset, rank_offset) = OFFSETS[i];
            let new_file: i8 = file + file_offset;
            let new_rank: i8 = rank + rank_offset;

            if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
                let target_sq: usize = (new_rank * 8 + new_file) as usize;
                attacks |= 1 << target_sq;
            }
            i += 1;
        }

        table[square] = attacks;
        square += 1;
    }

    return table;
}

const WHITE_PAWN_ATTACKS: [Bitboard; 64] = pawn_attacks(PieceColor::White);
const BLACK_PAWN_ATTACKS: [Bitboard; 64] = pawn_attacks(PieceColor::Black);

#[allow(unused)]
const fn pawn_attacks(color: PieceColor) -> [Bitboard; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut square: usize = 0;
    while square < 64 {
        let (file, rank) = (square % 8, square / 8);
        let mut attacks: u64 = 0;

        if match color {
            PieceColor::White => rank < 7,
            PieceColor::Black => rank > 1,
        } {
            if file > 0 {
                attacks |= match color {
                    PieceColor::White => 1 << ((rank + 1) * 8 + (file - 1)),
                    PieceColor::Black => 1 >> ((rank - 1) * 8 + (file - 1)),
                };
            }
            if file < 7 {
                attacks |= match color {
                    PieceColor::White => 1 << ((rank + 1) * 8 + (file + 1)),
                    PieceColor::Black => 1 >> ((rank - 1) * 8 + (file + 1)),
                };
            }
        }
        table[square] = attacks;
        square += 1;
    }
    return table;
}
