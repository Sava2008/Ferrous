use crate::{board::Board, board_geometry_templates::FILES};

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
        let (
            mut white_pawn_on_previous,
            mut black_pawn_on_previous,
            mut white_pawn_on_current,
            mut black_pawn_on_current,
            mut white_pawn_on_next,
            mut black_pawn_on_next,
        ): (bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, false);
        let (mut white_isolated, mut black_isolated, mut white_doubled, mut black_doubled): (
            i32,
            i32,
            u32,
            u32,
        ) = (0, 0, 0, 0);

        for f in 0..9 {
            if f < 8 {
                let (white_file_count, black_file_count) = self.count_pawns_on_file(FILES[f]);
                if white_file_count > 0 {
                    white_pawn_on_next = true;
                }
                if black_file_count > 0 {
                    black_pawn_on_next = true;
                }
                white_doubled += white_file_count.saturating_sub(1);
                black_doubled += black_file_count.saturating_sub(1);
            } else {
                (white_pawn_on_next, black_pawn_on_next) = (false, false);
            }
            if !white_pawn_on_previous && white_pawn_on_current && !white_pawn_on_next {
                white_isolated += 1;
            }
            if !black_pawn_on_previous && black_pawn_on_current && !black_pawn_on_next {
                black_isolated += 1;
            }

            white_pawn_on_previous = white_pawn_on_current;
            black_pawn_on_previous = black_pawn_on_current;

            white_pawn_on_current = white_pawn_on_next;
            black_pawn_on_current = black_pawn_on_next;

            white_pawn_on_next = false;
            black_pawn_on_next = false;
        }
        (
            pawn_structure.isolated_white,
            pawn_structure.isolated_black,
            pawn_structure.doubled_white,
            pawn_structure.doubled_black,
        ) = (
            white_isolated * 3,
            black_isolated * 3,
            white_doubled as i32 * 3,
            black_doubled as i32 * 3,
        );
    }
}
