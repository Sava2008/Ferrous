pub const WHITE_PAWN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    1, 1, 1, 0, 0, 1, 1, 1, // Rank2
    -1, 0, 2, 1, 1, -7, 0, -1, // Rank3
    1, -3, 4, 6, 6, 4, -5, 1, // Rank4
    -1, 1, 3, 4, 4, 3, 1, -1, // Rank5
    0, 2, 2, 3, 3, 2, 2, 0, // Rank6
    6, 7, 7, 7, 7, 7, 7, 6, // Rank7
    9, 10, 10, 10, 10, 10, 10, 9, // Rank8
];

pub const ENDGAME_WHITE_PAWN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    -1, -1, -1, -1, -1, -1, -1, -1, // Rank2
    0, 0, 0, 0, 0, 0, 0, 0, // Rank3
    1, 1, 1, 1, 1, 1, 1, 1, // Rank4
    2, 2, 2, 2, 2, 2, 2, 2, // Rank5
    5, 5, 5, 5, 5, 5, 5, 5, // Rank6
    10, 10, 10, 10, 10, 10, 10, 10, // Rank7
    10, 10, 10, 10, 10, 10, 10, 10, // Rank8
];

pub const BLACK_PAWN_HEURISTICS: [i32; 64] = [
    9, 10, 10, 10, 10, 10, 10, 9, // Rank1
    6, 7, 7, 7, 7, 7, 7, 6, // Rank2
    0, 2, 2, 3, 3, 2, 2, 0, // Rank3
    -1, 1, 3, 4, 4, 3, 1, -1, // Rank4
    1, -5, 4, 6, 6, 4, -3, 1, // Rank5
    -1, 0, -7, 1, 1, 2, 0, -1, // Rank6
    1, 1, 1, 0, 0, 1, 1, 1, // Rank7
    0, 0, 0, 0, 0, 0, 0, 0, // Rank8
];

pub const ENDGAME_BLACK_PAWN_HEURISTICS: [i32; 64] = [
    10, 10, 10, 10, 10, 10, 10, 10, // Rank1
    10, 10, 10, 10, 10, 10, 10, 10, // Rank2
    5, 5, 5, 5, 5, 5, 5, 5, // Rank3
    2, 2, 2, 2, 2, 2, 2, 2, // Rank4
    1, 1, 1, 1, 1, 1, 1, 1, // Rank5
    0, 0, 0, 0, 0, 0, 0, 0, // Rank6
    -1, -1, -1, -1, -1, -1, -1, -1, // Rank7
    0, 0, 0, 0, 0, 0, 0, 0, // Rank8
];

pub const WHITE_KNIGHT_HEURISTICS: [i32; 64] = [
    -10, -7, -5, -2, -2, -5, -7, -10, // Rank1
    -7, -3, 2, 3, 3, 2, -3, -7, // Rank2
    -4, 4, 5, 2, 2, 5, 4, -4, // Rank3
    -3, 2, 4, 7, 7, 4, 2, -3, // Rank4
    -3, 1, 2, 3, 3, 2, 1, -3, // Rank5
    -4, 0, 4, 2, 2, 4, 0, -4, // Rank6
    -7, -3, 1, 2, 2, 1, -3, -7, // Rank7
    -10, -7, -5, -2, -2, -5, -7, -10, // Rank8
];

pub const BLACK_KNIGHT_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_KNIGHT_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_BISHOP_HEURISTICS: [i32; 64] = [
    -5, -10, -5, -3, -3, -5, -10, -5, // Rank1
    -1, 7, 0, 2, 2, 0, 7, -1, // Rank2
    2, 3, 2, 5, 5, 2, 3, 2, // Rank3
    -2, 2, 5, 7, 7, 5, 2, -2, // Rank4
    -2, 2, 4, 6, 6, 4, 2, -2, // Rank5
    1, 0, -1, 3, 3, -1, 0, 1, // Rank6
    -6, 5, -3, -1, -1, -3, 5, -6, // Rank7
    -5, -10, -5, -4, -4, -5, -10, -5, // Rank8
];

pub const BLACK_BISHOP_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_BISHOP_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_QUEEN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    -8, -5, -2, 0, 0, -2, -5, -8, // Rank2
    -8, -5, -2, 2, 2, -2, -5, -8, // Rank3
    -10, -8, -2, 3, 3, -2, -8, -10, // Rank4
    -10, -8, -2, 2, 2, -2, -8, -10, // Rank5
    -8, -5, -2, 0, 0, -2, -5, -8, // Rank6
    -5, -3, 0, 0, 0, 0, -3, -5, // Rank7
    -5, -5, -5, -5, -5, -5, -5, -5, // Rank8
];

pub const BLACK_QUEEN_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_QUEEN_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_ROOK_HEURISTICS: [i32; 64] = [
    1, -3, 2, 4, 4, 2, -3, 1, // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 2
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 3
    0, 0, 0, 3, 3, 0, 0, 0, // Rank 4
    0, 0, 0, 3, 3, 0, 0, 0, // Rank 5
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 6
    3, 4, 4, 4, 4, 4, 4, 3, // Rank 7
    3, 3, 3, 3, 3, 3, 3, 3, // Rank 8
];

pub const BLACK_ROOK_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_ROOK_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_KING_HEURISTICS: [i32; 64] = [
    2, 3, -2, -2, -2, -2, 3, 2, // Rank 1
    -7, -7, -7, -7, -7, -7, -7, -7, // Rank 2
    -15, -15, -15, -15, -15, -15, -15, -15, // Rank 3
    -30, -25, -20, -15, -15, -20, -25, -30, // Rank 4
    -40, -35, -30, -25, -25, -30, -35, -40, // Rank 5
    -50, -45, -40, -35, -35, -40, -45, -50, // Rank 6
    -60, -55, -50, -45, -45, -50, -55, -60, // Rank 7
    -70, -65, -60, -30, -30, -60, -65, -70, // Rank 8
];

pub const BLACK_KING_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_KING_HEURISTICS;
    temp.reverse();
    temp
};

pub static mut HEURISTICS_TABLE: [[i32; 64]; 12] = [
    WHITE_PAWN_HEURISTICS,
    WHITE_KNIGHT_HEURISTICS,
    WHITE_BISHOP_HEURISTICS,
    WHITE_ROOK_HEURISTICS,
    WHITE_QUEEN_HEURISTICS,
    WHITE_KING_HEURISTICS,
    BLACK_PAWN_HEURISTICS,
    BLACK_KNIGHT_HEURISTICS,
    BLACK_BISHOP_HEURISTICS,
    BLACK_ROOK_HEURISTICS,
    BLACK_QUEEN_HEURISTICS,
    BLACK_KING_HEURISTICS,
];

pub const ENDGAME_WHITE_KING_HEURISTICS: [i32; 64] = [
    -9, -8, -7, -7, -7, -7, -8, -9, // Rank 1
    -8, -7, -6, -5, -5, -6, -7, -8, // Rank 2
    -7, -6, -4, 0, 0, -4, -6, -7, // Rank 3
    -7, -5, 0, 2, 2, 0, -5, -7, // Rank 4
    -7, -5, 0, 2, 2, 0, -5, -7, // Rank 5
    -7, -6, -4, 0, 0, -4, -6, -7, // Rank 6
    -8, -7, -6, -5, -5, -8, -7, -8, // Rank 7
    -9, -8, -9, -9, -9, -9, -8, -9, // Rank 8
];

pub const ENDGAME_BLACK_KING_HEURISTICS: [i32; 64] = WHITE_KING_HEURISTICS;

// used to punish bad pawn structure
pub const OPENING_BAD_STRUCTURE_ANTIBONUS: i32 = 3;
pub const MIDDLEGAME_BAD_STRUCTURE_ANTIBONUS: i32 = 4;
pub const ENDGAME_BAD_STRUCTURE_ANTIBONUS: i32 = 5;
