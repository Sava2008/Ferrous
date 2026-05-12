pub const FILE_A: u64 = 0b0000000100000001000000010000000100000001000000010000000100000001;
pub const FILE_B: u64 = 0b0000001000000010000000100000001000000010000000100000001000000010;
pub const FILE_C: u64 = 0b0000010000000100000001000000010000000100000001000000010000000100;
pub const FILE_D: u64 = 0b0000100000001000000010000000100000001000000010000000100000001000;
pub const FILE_E: u64 = 0b0001000000010000000100000001000000010000000100000001000000010000;
pub const FILE_F: u64 = 0b0010000000100000001000000010000000100000001000000010000000100000;
pub const FILE_G: u64 = 0b0100000001000000010000000100000001000000010000000100000001000000;
pub const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;

pub const RANK_1: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
pub const RANK_2: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
pub const RANK_3: u64 = 0b0000000000000000000000000000000000000000111111110000000000000000;
pub const RANK_4: u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
pub const RANK_5: u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
pub const RANK_6: u64 = 0b0000000000000000111111110000000000000000000000000000000000000000;
pub const RANK_7: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
pub const RANK_8: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

/* move encoding (leftmost to rightmost):

START INCLUSIVE, END INCLUSIVE
bit 0-5: from square
bit 6-11: to square
bit 12-14: promotion choice
bit 15-17: type of moving piece
bit 18-20: type of captured piece
bit 21: e.p. marker
bit 22: castling marker
 */

pub const TO_SHIFT: u32 = 6;
pub const PROMOTION_SHIFT: u32 = 12;
pub const FROM_MASK: u32 = 0b0000000000111111; // to access from info of a move, `data & FROM_MASK` to be applied
pub const TO_MASK: u32 = 0b0000111111000000; // to access from info of a move, `(data & TO_MASK) >> 6` to be applied
pub const PROMOTION_MASK: u32 = 0b0111000000000000; // to access from info of a move, `(data & PROMOTION_MASK) >> 12` to be applied

/* piece type encryption: 1 (0b1) - pawn, 2 (0b10) - knight, 3 (0b11) - bishop,
4 (0b100) - rook, 5 (0b101) - queen, 6 (0b110) - king

piece color encryption: 1 (0b1) - white, 2 (0b10) - black */

pub const MOVING_PIECE_TYPE_SHIFT: u32 = 15; // `(u32 & PIECE_COLOR_MASK) >> MOVING_PIECE_COLOR_SHIFT` to get the piece color
pub const CAPTURED_PIECE_TYPE_SHIFT: u32 = 18;
pub const CASTLING_SHIFT: u32 = 22;
pub const EN_PASSANT_SHIFT: u32 = 21;

pub const MOVING_PIECE_TYPE_MASK: u32 = 0b111000000000000000;
pub const CAPTURED_PIECE_TYPE_MASK: u32 = 0b111000000000000000000;
pub const CASTLING_MASK: u32 = 0b10000000000000000000000;
pub const EN_PASSANT_MASK: u32 = 0b1000000000000000000000;

pub const WHITE_PAWN_U32: u32 = 9;
pub const WHITE_KNIGHT_U32: u32 = 10;
pub const WHITE_BISHOP_U32: u32 = 11;
pub const WHITE_ROOK_U32: u32 = 12;
pub const WHITE_QUEEN_U32: u32 = 13;
pub const WHITE_KING_U32: u32 = 14;

pub const BLACK_PAWN_U32: u32 = 17;
pub const BLACK_KNIGHT_U32: u32 = 18;
pub const BLACK_BISHOP_U32: u32 = 19;
pub const BLACK_ROOK_U32: u32 = 20;
pub const BLACK_QUEEN_U32: u32 = 21;
pub const BLACK_KING_U32: u32 = 22;

pub const U32_PIECES_TABLE: [u32; 12] = [
    WHITE_PAWN_U32,
    WHITE_KNIGHT_U32,
    WHITE_BISHOP_U32,
    WHITE_ROOK_U32,
    WHITE_QUEEN_U32,
    WHITE_KING_U32,
    BLACK_PAWN_U32,
    BLACK_KNIGHT_U32,
    BLACK_BISHOP_U32,
    BLACK_ROOK_U32,
    BLACK_QUEEN_U32,
    BLACK_KING_U32,
];

pub const NO_PIECE_WHITE: u32 = 8;
pub const NO_PIECE_BLACK: u32 = 16;

pub const COLORLESS_PAWN: u32 = 1;
pub const COLORLESS_KNIGHT: u32 = 2;
pub const COLORLESS_BISHOP: u32 = 3;
pub const COLORLESS_ROOK: u32 = 4;
pub const COLORLESS_QUEEN: u32 = 5;
pub const COLORLESS_KING: u32 = 6;

pub const WHITE_SHORT: u8 = 1;
pub const WHITE_LONG: u8 = 2;
pub const BLACK_SHORT: u8 = 4;
pub const BLACK_LONG: u8 = 8; // all available = 15

pub const WHITE_SHORT_MASK: u8 = 0b1;
pub const WHITE_LONG_MASK: u8 = 0b10;
pub const BLACK_SHORT_MASK: u8 = 0b100;
pub const BLACK_LONG_MASK: u8 = 0b1000;

#[inline(always)]
pub fn from_square(m: u32) -> u8 {
    return (m & FROM_MASK) as u8;
}
#[inline(always)]
pub fn to_square(m: u32) -> u32 {
    return (m & TO_MASK) >> TO_SHIFT;
}
#[inline(always)]
pub fn promotion(m: u32) -> u8 {
    return ((m & PROMOTION_MASK) >> PROMOTION_SHIFT) as u8;
}
#[inline(always)]
pub fn moving_piece_type(m: u32) -> u32 {
    return (m & MOVING_PIECE_TYPE_MASK) >> MOVING_PIECE_TYPE_SHIFT;
}

#[inline(always)]
pub fn captured_piece_type(m: u32) -> u32 {
    return (m & CAPTURED_PIECE_TYPE_MASK) >> CAPTURED_PIECE_TYPE_SHIFT;
}

#[inline(always)]
pub fn castling(m: u32) -> u8 {
    return ((m & CASTLING_MASK) >> CASTLING_SHIFT) as u8;
}
#[inline(always)]
pub fn en_passant(m: u32) -> u8 {
    return ((m & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT) as u8;
}

#[inline(always)]
pub fn get_bb_index(mut piece_type: u32, color: &u32) -> (usize, usize) {
    // (bitboard_idx, occupancy_idx)
    let occupancy_idx: usize = if color == &16 {
        piece_type += 6;
        1
    } else {
        0
    };
    return ((piece_type as usize) - 1, occupancy_idx);
}
