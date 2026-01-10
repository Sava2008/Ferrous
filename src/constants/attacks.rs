use crate::{board_geometry_templates::Bitboard, enums::PieceColor};

pub const KNIGHT_ATTACKS: [Bitboard; 64] = knight_attacks();

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

pub const KING_ATTACKS: [Bitboard; 64] = king_attacks();

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

pub const WHITE_PAWN_ATTACKS: [Bitboard; 64] = pawn_attacks(PieceColor::White);
pub const BLACK_PAWN_ATTACKS: [Bitboard; 64] = pawn_attacks(PieceColor::Black);

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
                    PieceColor::Black => 1 << ((rank - 1) * 8 + (file - 1)),
                };
            }
            if file < 7 {
                attacks |= match color {
                    PieceColor::White => 1 << ((rank + 1) * 8 + (file + 1)),
                    PieceColor::Black => 1 << ((rank - 1) * 8 + (file + 1)),
                };
            }
        }
        table[square] = attacks;
        square += 1;
    }
    return table;
}

// taken from another chess engine's source code
#[allow(unused)]
const ROOK_MAGICS: [Bitboard; 64] = [
    0xA180022080400230,
    0x0040100040022000,
    0x0080088020001002,
    0x0080080280841000,
    0x4200042010460008,
    0x04800A0003040080,
    0x0400110082041008,
    0x008000A041000880,
    0x10138001A080C010,
    0x0000804008200480,
    0x00010011012000C0,
    0x0022004128102200,
    0x000200081201200C,
    0x202A001048460004,
    0x0081000100420004,
    0x4000800380004500,
    0x0000208002904001,
    0x0090004040026008,
    0x0208808010002001,
    0x2002020020704940,
    0x8048010008110005,
    0x6820808004002200,
    0x0A80040008023011,
    0x00B1460000811044,
    0x4204400080008EA0,
    0xB002400180200184,
    0x2020200080100380,
    0x0010080080100080,
    0x2204080080800400,
    0x0000A40080360080,
    0x02040604002810B1,
    0x008C218600004104,
    0x8180004000402000,
    0x488C402000401001,
    0x4018A00080801004,
    0x1230002105001008,
    0x8904800800800400,
    0x0042000C42003810,
    0x008408110400B012,
    0x0018086182000401,
    0x2240088020C28000,
    0x001001201040C004,
    0x0A02008010420020,
    0x0010003009010060,
    0x0004008008008014,
    0x0080020004008080,
    0x0282020001008080,
    0x50000181204A0004,
    0x48FFFE99FECFAA00,
    0x48FFFE99FECFAA00,
    0x497FFFADFF9C2E00,
    0x613FFFDDFFCE9200,
    0xFFFFFFE9FFE7CE00,
    0xFFFFFFF5FFF3E600,
    0x0010301802830400,
    0x510FFFF5F63C96A0,
    0xEBFFFFB9FF9FC526,
    0x61FFFEDDFEEDAEAE,
    0x53BFFFEDFFDEB1A2,
    0x127FFFB9FFDFB5F6,
    0x411FFFDDFFDBF4D6,
    0x0801000804000603,
    0x0003FFEF27EEBE74,
    0x7645FFFECBFEA79E,
];

#[allow(unused)]
const BISHOP_MAGICS: [Bitboard; 64] = [
    0xFFEDF9FD7CFCFFFF,
    0xFC0962854A77F576,
    0x5822022042000000,
    0x2CA804A100200020,
    0x0204042200000900,
    0x2002121024000002,
    0xFC0A66C64A7EF576,
    0x7FFDFDFCBD79FFFF,
    0xFC0846A64A34FFF6,
    0xFC087A874A3CF7F6,
    0x1001080204002100,
    0x1810080489021800,
    0x0062040420010A00,
    0x5028043004300020,
    0xFC0864AE59B4FF76,
    0x3C0860AF4B35FF76,
    0x73C01AF56CF4CFFB,
    0x41A01CFAD64AAFFC,
    0x040C0422080A0598,
    0x4228020082004050,
    0x0200800400E00100,
    0x020B001230021040,
    0x7C0C028F5B34FF76,
    0xFC0A028E5AB4DF76,
    0x0020208050A42180,
    0x001004804B280200,
    0x2048020024040010,
    0x0102C04004010200,
    0x020408204C002010,
    0x02411100020080C1,
    0x102A008084042100,
    0x0941030000A09846,
    0x0244100800400200,
    0x4000901010080696,
    0x0000280404180020,
    0x0800042008240100,
    0x0220008400088020,
    0x04020182000904C9,
    0x0023010400020600,
    0x0041040020110302,
    0xDCEFD9B54BFCC09F,
    0xF95FFA765AFD602B,
    0x1401210240484800,
    0x0022244208010080,
    0x1105040104000210,
    0x2040088800C40081,
    0x43FF9A5CF4CA0C01,
    0x4BFFCD8E7C587601,
    0xFC0FF2865334F576,
    0xFC0BF6CE5924F576,
    0x80000B0401040402,
    0x0020004821880A00,
    0x8200002022440100,
    0x0009431801010068,
    0xC3FFB7DC36CA8C89,
    0xC3FF8A54F4CA2C89,
    0xFFFFFCFCFD79EDFF,
    0xFC0863FCCB147576,
    0x040C000022013020,
    0x2000104000420600,
    0x0400000260142410,
    0x0800633408100500,
    0xFC087E8E4BB2F736,
    0x43FF9E4EF4CA2C89,
];
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

const fn rook_masks() -> [Bitboard; 64] {
    let mut masks: [Bitboard; 64] = [69; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut mask: Bitboard = 0;
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

const fn bishop_mask() -> [Bitboard; 64] {
    let mut masks: [Bitboard; 64] = [69; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut mask: Bitboard = 0;
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

pub const ROOK_MASKS: [Bitboard; 64] = rook_masks();
pub const BISHOP_MASKS: [Bitboard; 64] = bishop_mask();

fn generate_blockers(mask: Bitboard) -> Vec<Bitboard> {
    let bits: Vec<u32> = (0..64).filter(|&i| (mask >> i) & 1 == 1).collect();

    let n: usize = bits.len();
    let total: usize = 1 << n;

    let mut blockers: Vec<u64> = Vec::with_capacity(total);

    for i in 0..total {
        let mut blocker_config: u64 = 0u64;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                blocker_config |= 1u64 << bits[j];
            }
        }
        blockers.push(blocker_config);
    }

    return blockers;
}

static mut ROOK_ATTACKS: [Bitboard; 64 * 4096] = [69; 64 * 4096];
static mut BISHOP_ATTACKS: [Bitboard; 64 * 512] = [69; 64 * 512];

fn rook_attacks_with_blockers(square: usize, blockers: Bitboard) -> Bitboard {
    let (rank, file) = (square / 8, square % 8);
    let mut attacks = 0u64;

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

fn bishop_attacks_with_blockers(square: usize, blockers: Bitboard) -> Bitboard {
    let (rank, file) = (square / 8, square % 8);
    let mut attacks = 0u64;

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

fn initialize_sliding_attack_tables() -> () {
    let mut square: usize = 0;
    unsafe {
        while square < 64 {
            let rook_mask: Bitboard = ROOK_MASKS[square];
            let rook_blockers: Vec<u64> = generate_blockers(rook_mask);

            let bishop_mask: Bitboard = BISHOP_MASKS[square];
            let bishop_blockers: Vec<u64> = generate_blockers(bishop_mask);

            let mut blockers_index: usize = 0;
            while blockers_index < rook_blockers.len() {
                let idx: usize = ((rook_blockers[blockers_index] * ROOK_MAGICS[square])
                    >> ROOK_SHIFTS[square]) as usize;
                ROOK_ATTACKS[square * 4096 + idx] =
                    rook_attacks_with_blockers(square, rook_blockers[blockers_index]);
                blockers_index += 1;
            }

            blockers_index = 0;
            while blockers_index < bishop_blockers.len() {
                let idx: usize = ((bishop_blockers[blockers_index] * BISHOP_MAGICS[square])
                    >> BISHOP_SHIFTS[square]) as usize;
                BISHOP_ATTACKS[square * 512 + idx] =
                    bishop_attacks_with_blockers(square, bishop_blockers[blockers_index]);
                blockers_index += 1;
            }
            square += 1;
        }
    }
}
