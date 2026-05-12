pub const ROOK_VALUE: i32 = 500;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 310;
pub const PAWN_VALUE: i32 = 100;
pub const QUEEN_VALUE: i32 = 1000;

pub const VALUE_TABLE: [i32; 12] = [
    PAWN_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    QUEEN_VALUE,
    0,
    -PAWN_VALUE,
    -KNIGHT_VALUE,
    -BISHOP_VALUE,
    -ROOK_VALUE,
    -QUEEN_VALUE,
    0,
];
