use crate::{board::Board, board_geometry_templates::FILES, constants::masks::ISOLATED_PAWNS};

pub fn get_adjacent_files(sq: usize) -> (u64, u64) {
    let (adjacent_left, adjacent_right) = if sq % 8 == 0 {
        (64, sq + 1)
    } else if (sq + 1) % 8 == 0 {
        (sq - 1, 64)
    } else {
        (sq - 1, sq + 1)
    };
    return (
        if adjacent_left < 64 {
            FILES[adjacent_left % 8]
        } else {
            0
        },
        if adjacent_right < 64 {
            FILES[adjacent_right % 8]
        } else {
            0
        },
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct PawnStructureFeatures {
    pub isolated_white: i32,
    pub isolated_black: i32,
    pub doubled_white: i32,
    pub doubled_black: i32,
    pub white_passers: i32,
    pub black_passers: i32,
}

impl PawnStructureFeatures {
    pub fn new() -> Self {
        return Self {
            isolated_white: 0,
            isolated_black: 0,
            doubled_white: 0,
            doubled_black: 0,
            white_passers: 0,
            black_passers: 0,
        };
    }

    #[inline(always)]
    pub fn sum(&self) -> i32 {
        return self.isolated_white + self.doubled_white
            - self.isolated_black
            - self.isolated_white;
    }
}
#[inline(always)]
fn file_occupancy_parallel(mut pawns: u64) -> usize {
    pawns |= pawns >> 8;
    pawns |= pawns >> 16;
    pawns |= pawns >> 24;
    pawns |= pawns >> 32;
    pawns |= pawns >> 40;
    pawns |= pawns >> 48;
    pawns |= pawns >> 56;
    return (pawns & 255) as usize;
}

// to be applied after a move for simplicity
impl Board {
    #[inline(always)]
    fn count_pawns_on_file(&self, f: u64) -> (u32, u32) {
        return (
            (self.bitboards[0] & f).count_ones(),
            (self.bitboards[6] & f).count_ones(),
        );
    }

    #[inline(always)]
    pub fn modify_pawn_structure(&self, pawn_structure: &mut PawnStructureFeatures) -> () {
        let (mut white_doubled, mut black_doubled): (u32, u32) = (0, 0);
        let white_pawns: u64 = self.bitboards[0];
        let black_pawns: u64 = self.bitboards[6];

        for f in FILES {
            let (white_file_count, black_file_count) = self.count_pawns_on_file(f);

            white_doubled += white_file_count.saturating_sub(1);
            black_doubled += black_file_count.saturating_sub(1);
        }
        (
            pawn_structure.isolated_white,
            pawn_structure.isolated_black,
            pawn_structure.doubled_white,
            pawn_structure.doubled_black,
        ) = (
            ISOLATED_PAWNS[file_occupancy_parallel(white_pawns)] * 3,
            ISOLATED_PAWNS[file_occupancy_parallel(black_pawns)] * 3,
            white_doubled as i32 * 3,
            black_doubled as i32 * 3,
        );
    }
}
