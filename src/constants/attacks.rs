use crate::{
    board_geometry_templates::{
        FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, NO_PIECE_BLACK,
        NO_PIECE_WHITE, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
    },
    constants::magic_bitboards::*,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const KNIGHT_ATTACKS: [u64; 64] = knight_attacks();
pub const COORDS_TO_INDICES: Lazy<HashMap<String, u8>> = Lazy::new(|| {
    (0..=63)
        .map(|i: u8| {
            let (col, row) = (i as u32 / 8 + 1, i as u32 % 8);
            {
                (
                    format!("{}{}", std::char::from_u32(row + 97).unwrap(), col),
                    i,
                )
            }
        })
        .collect()
});
pub const INDICES_TO_COORDS: Lazy<HashMap<u8, String>> = Lazy::new(|| {
    (0..=63)
        .map(|i: u8| {
            let (col, row) = (i as u32 / 8 + 1, i as u32 % 8);
            {
                (
                    i,
                    format!("{}{}", std::char::from_u32(row + 97).unwrap(), col),
                )
            }
        })
        .collect()
});

#[allow(unused)]
const fn knight_attacks() -> [u64; 64] {
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

pub const KING_ATTACKS: [u64; 64] = king_attacks();

#[allow(unused)]
const fn king_attacks() -> [u64; 64] {
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

pub const WHITE_PAWN_ATTACKS: [u64; 64] = pawn_attacks(NO_PIECE_WHITE as u8);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = pawn_attacks(NO_PIECE_BLACK as u8);

#[allow(unused)]
const fn pawn_attacks(color: u8) -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut square: usize = 0;
    while square < 64 {
        let (file, rank) = (square % 8, square / 8);
        let mut attacks: u64 = 0;

        if match color {
            8 => rank < 7,
            _ => rank > 1,
        } {
            if file > 0 {
                attacks |= match color {
                    8 => 1 << ((rank + 1) * 8 + (file - 1)),
                    _ => 1 << ((rank - 1) * 8 + (file - 1)),
                };
            }
            if file < 7 {
                attacks |= match color {
                    8 => 1 << ((rank + 1) * 8 + (file + 1)),
                    _ => 1 << ((rank - 1) * 8 + (file + 1)),
                };
            }
        }
        table[square] = attacks;
        square += 1;
    }
    return table;
}

pub const ROOK_SHIFTS: [u32; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
];

pub const BISHOP_SHIFTS: [u32; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 58,
];

const fn rook_masks() -> [u64; 64] {
    let mut masks: [u64; 64] = [69; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut mask: u64 = 0;
        let (rank, file) = (i / 8, i % 8);

        let (mut r, mut f) = (1, 1);

        while r < 7 {
            if r != rank {
                mask |= 1 << (r * 8 + file);
            }
            r += 1;
        }

        while f < 7 {
            if f != file {
                mask |= 1 << (rank * 8 + f);
            }
            f += 1;
        }

        mask &= !(1 << i);
        masks[i] = mask;
        i += 1;
    }
    return masks;
}

const fn bishop_mask() -> [u64; 64] {
    let mut masks: [u64; 64] = [69; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut mask: u64 = 0;
        let (rank, file) = (i / 8, i % 8);

        let (mut r, mut f) = (rank, file);
        while r < 7 && f < 7 {
            mask |= 1 << (r * 8 + f);
            r += 1;
            f += 1;
        }

        (r, f) = (rank, file);
        while r < 7 && f > 0 {
            mask |= 1 << (r * 8 + f);
            r += 1;
            f -= 1;
        }

        (r, f) = (rank, file);
        while r > 0 && f > 0 {
            mask |= 1 << (r * 8 + f);
            r -= 1;
            f -= 1;
        }

        (r, f) = (rank, file);
        while r > 0 && f < 7 {
            mask |= 1 << (r * 8 + f);
            r -= 1;
            f += 1;
        }

        mask &= !(1 << i);
        masks[i] = mask;
        i += 1;
    }
    return masks;
}

pub const ROOK_MASKS: [u64; 64] = rook_masks();
pub const BISHOP_MASKS: [u64; 64] = bishop_mask();

pub fn generate_blockers(mask: u64) -> Vec<u64> {
    let bits: Vec<u32> = (0..64).filter(|&i| (mask >> i) & 1 == 1).collect();

    let n: usize = bits.len();
    let total: usize = 1 << n;

    let mut blockers: Vec<u64> = Vec::with_capacity(total);

    for i in 0..total {
        let mut blocker_config: u64 = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                blocker_config |= 1 << bits[j];
            }
        }
        blockers.push(blocker_config);
    }

    return blockers;
}
pub static mut ROOK_ATTACKS: [u64; 64 * 4096] = [0; 64 * 4096];
pub static mut BISHOP_ATTACKS: [u64; 64 * 512] = [0; 64 * 512];

pub const ROOK_OFFSETS: [usize; 64] = {
    let mut offsets: [usize; 64] = [0; 64];
    let mut idx: usize = 0;
    while idx < 64 {
        offsets[idx] = idx * 4096;
        idx += 1;
    }
    offsets
};

pub const BISHOP_OFFSETS: [usize; 64] = {
    let mut offsets: [usize; 64] = [0; 64];
    let mut idx: usize = 0;
    while idx < 64 {
        offsets[idx] = idx * 512;
        idx += 1;
    }
    offsets
};

pub fn rook_attacks_with_blockers(square: usize, blockers: u64) -> u64 {
    let (rank, file) = (square / 8, square % 8);
    let mut attacks: u64 = 0;

    let mut r: usize = rank + 1;
    while r < 8 {
        let pos: usize = r * 8 + file;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r += 1;
    }

    let mut r: i32 = rank as i32 - 1;
    while r >= 0 {
        let pos: usize = r as usize * 8 + file;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r -= 1;
    }

    let mut f: usize = file + 1;
    while f < 8 {
        let pos: usize = rank * 8 + f;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        f += 1;
    }

    let mut f: i32 = file as i32 - 1;
    while f >= 0 {
        let pos: usize = rank * 8 + f as usize;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        f -= 1;
    }

    return attacks;
}

pub fn bishop_attacks_with_blockers(square: usize, blockers: u64) -> u64 {
    let (rank, file) = (square / 8, square % 8);
    let mut attacks: u64 = 0;

    let (mut r, mut f) = (rank as i32 + 1, file as i32 + 1);
    while r < 8 && f < 8 {
        let pos: usize = r as usize * 8 + f as usize;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r += 1;
        f += 1;
    }

    let (mut r, mut f) = (rank as i32 + 1, file as i32 - 1);
    while r < 8 && f >= 0 {
        let pos: usize = r as usize * 8 + f as usize;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r += 1;
        f -= 1;
    }

    let (mut r, mut f) = (rank as i32 - 1, file as i32 + 1);
    while r >= 0 && f < 8 {
        let pos = r as usize * 8 + f as usize;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r -= 1;
        f += 1;
    }

    let (mut r, mut f) = (rank as i32 - 1, file as i32 - 1);
    while r >= 0 && f >= 0 {
        let pos: usize = r as usize * 8 + f as usize;
        attacks |= 1 << pos;
        if (blockers >> pos) & 1 == 1 {
            break;
        }
        r -= 1;
        f -= 1;
    }

    return attacks;
}

pub fn initialize_sliding_attack_tables() -> () {
    let mut square: usize = 0;
    unsafe {
        while square < 64 {
            let rook_mask: u64 = ROOK_MASKS[square];
            let rook_blockers: Vec<u64> = generate_blockers(rook_mask);

            let bishop_mask: u64 = BISHOP_MASKS[square];
            let bishop_blockers: Vec<u64> = generate_blockers(bishop_mask);

            let mut blockers_index: usize = 0;
            while blockers_index < rook_blockers.len() {
                let idx: usize = ((rook_blockers[blockers_index].wrapping_mul(ROOK_MAGICS[square]))
                    >> ROOK_SHIFTS[square]) as usize;
                ROOK_ATTACKS[ROOK_OFFSETS[square] + idx] =
                    rook_attacks_with_blockers(square, rook_blockers[blockers_index]);
                blockers_index += 1;
            }

            blockers_index = 0;
            while blockers_index < bishop_blockers.len() {
                let idx: usize = ((bishop_blockers[blockers_index]
                    .wrapping_mul(BISHOP_MAGICS[square]))
                    >> BISHOP_SHIFTS[square]) as usize;
                BISHOP_ATTACKS[BISHOP_OFFSETS[square] + idx] =
                    bishop_attacks_with_blockers(square, bishop_blockers[blockers_index]);
                blockers_index += 1;
            }
            square += 1;
        }
    }
}

#[inline(always)]
pub fn rook_attacks(initial_pos: usize, occupancy: u64) -> u64 {
    let masked_blockers: u64 = occupancy & ROOK_MASKS[initial_pos];
    let idx: usize = ((masked_blockers.wrapping_mul(ROOK_MAGICS[initial_pos]))
        >> ROOK_SHIFTS[initial_pos]) as usize;
    let offset: usize = ROOK_OFFSETS[initial_pos];

    return unsafe { ROOK_ATTACKS[offset + idx] };
}

#[inline(always)]
pub fn bishop_attacks(initial_pos: usize, occupancy: u64) -> u64 {
    let masked_blockers: u64 = occupancy & BISHOP_MASKS[initial_pos];
    let idx: usize = ((masked_blockers.wrapping_mul(BISHOP_MAGICS[initial_pos]))
        >> BISHOP_SHIFTS[initial_pos]) as usize;
    let offset: usize = BISHOP_OFFSETS[initial_pos];

    return unsafe { BISHOP_ATTACKS[offset + idx] };
}

pub static mut RAYS_BETWEEN: [[u64; 64]; 64] = [[0; 64]; 64];
pub static mut RAYS_FROM: [[u64; 64]; 64] = [[0; 64]; 64];

pub fn compute_all_rays() -> () {
    let mut rays: [[u64; 64]; 64] = [[0u64; 64]; 64];
    for sq1 in 0..64 {
        for sq2 in 0..64 {
            rays[sq1][sq2] = compute_ray_between(sq1 as u8, sq2 as u8);
        }
    }
    unsafe {
        RAYS_BETWEEN = rays;
    }
}

pub fn compute_all_rays_from() -> () {
    let mut rays: [[u64; 64]; 64] = [[0; 64]; 64];
    for sq1 in 0..64 {
        for sq2 in 0..64 {
            rays[sq1][sq2] = compute_ray_from(sq1 as u8, sq2 as u8);
        }
    }
    unsafe {
        RAYS_FROM = rays;
    }
}

fn compute_ray_between(sq1: u8, sq2: u8) -> u64 {
    if sq1 == sq2 {
        return 0;
    }

    let (rank1, file1) = (sq1 / 8, sq1 % 8);
    let (rank2, file2) = (sq2 / 8, sq2 % 8);

    let rank_diff = (rank2 as i8 - rank1 as i8).abs();
    let file_diff: i8 = (file2 as i8 - file1 as i8).abs();

    if rank_diff != 0 && file_diff != 0 && rank_diff != file_diff {
        return 0;
    }

    let rank_step: i8 = if rank2 > rank1 {
        1
    } else if rank2 < rank1 {
        -1
    } else {
        0
    };
    let file_step: i8 = if file2 > file1 {
        1
    } else if file2 < file1 {
        -1
    } else {
        0
    };

    let mut ray: u64 = 0;
    let mut r: i8 = rank1 as i8 + rank_step;
    let mut f: i8 = file1 as i8 + file_step;

    while r != rank2 as i8 || f != file2 as i8 {
        ray |= 1 << (r * 8 + f);
        r += rank_step;
        f += file_step;
    }

    return ray;
}

// Compute ray from sq1 through sq2 and beyond
fn compute_ray_from(sq1: u8, sq2: u8) -> u64 {
    if sq1 == sq2 {
        return 0;
    }

    let (rank1, file1) = (sq1 / 8, sq1 % 8);
    let (rank2, file2) = (sq2 / 8, sq2 % 8);

    let rank_diff: i8 = (rank2 as i8 - rank1 as i8).abs();
    let file_diff: i8 = (file2 as i8 - file1 as i8).abs();

    if rank_diff != 0 && file_diff != 0 && rank_diff != file_diff {
        return 0;
    }

    let rank_step: i8 = if rank2 > rank1 {
        1
    } else if rank2 < rank1 {
        -1
    } else {
        0
    };
    let file_step: i8 = if file2 > file1 {
        1
    } else if file2 < file1 {
        -1
    } else {
        0
    };

    let mut ray: u64 = 0;
    let mut r: i8 = rank2 as i8 + rank_step;
    let mut f: i8 = file2 as i8 + file_step;

    while r >= 0 && r < 8 && f >= 0 && f < 8 {
        ray |= 1 << (r * 8 + f);
        r += rank_step;
        f += file_step;
    }

    return ray;
}

pub static mut TWO_SQUARES_LINE: [[u64; 64]; 64] = [[0; 64]; 64];

pub fn compute_all_lines() -> () {
    let mut lines: [[u64; 64]; 64] = [[0; 64]; 64];

    for sq1 in 0..64 {
        for sq2 in 0..64 {
            if sq1 == sq2 {
                continue;
            }

            let (rank1, file1) = (sq1 / 8, sq1 % 8);
            let (rank2, file2) = (sq2 / 8, sq2 % 8);

            let rank_diff: i8 = (rank2 as i8 - rank1 as i8).abs();
            let file_diff: i8 = (file2 as i8 - file1 as i8).abs();

            if rank_diff == 0 {
                lines[sq1][sq2] |= match rank1 {
                    0 => RANK_1,
                    1 => RANK_2,
                    2 => RANK_3,
                    3 => RANK_4,
                    4 => RANK_5,
                    5 => RANK_6,
                    6 => RANK_7,
                    7 => RANK_8,
                    _ => unreachable!(),
                };
            } else if file_diff == 0 {
                lines[sq1][sq2] |= match file1 {
                    0 => FILE_A,
                    1 => FILE_B,
                    2 => FILE_C,
                    3 => FILE_D,
                    4 => FILE_E,
                    5 => FILE_F,
                    6 => FILE_G,
                    7 => FILE_H,
                    _ => unreachable!(),
                };
            } else if rank_diff == file_diff {
                let step_r: i8 = if rank2 > rank1 { 1 } else { -1 };
                let step_f: i8 = if file2 > file1 { 1 } else { -1 };

                let mut r: i8 = rank1 as i8;
                let mut f: i8 = file1 as i8;

                while r > 0 && f > 0 && r < 7 && f < 7 {
                    r -= step_r;
                    f -= step_f;
                }

                while r >= 0 && r < 8 && f >= 0 && f < 8 {
                    lines[sq1][sq2] |= 1 << (r as usize * 8 + f as usize);
                    r += step_r;
                    f += step_f;
                }
            }
        }
    }

    unsafe {
        TWO_SQUARES_LINE = lines;
    }
}

pub static mut MVV_LVA: [[i16; 6]; 6] = [[69; 6]; 6]; // MVV_LVA[victim][attacker]
pub fn compute_mvvlva() -> () {
    for x in 0..6 {
        for y in 0..6 {
            unsafe {
                MVV_LVA[x][y] = (x * 6 + (5 - y)) as i16;
            }
        }
    }
}
