pub const WHITE_PAWN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    10, 10, 10, 0, 0, 10, 10, 10, // Rank2
    -5, 0, -35, 5, 5, 10, 0, -5, // Rank3
    5, -25, 20, 30, 30, 20, -3, 5, // Rank4
    -5, 5, 15, 20, 20, 15, 5, -5, // Rank5
    8, 10, 10, 15, 15, 10, 10, 8, // Rank6
    30, 35, 35, 35, 35, 35, 35, 30, // Rank7
    36, 40, 40, 40, 40, 40, 40, 36, // Rank8
];

pub const ENDGAME_WHITE_PAWN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    -5, -5, -5, -5, -5, -5, -5, -5, // Rank2
    0, 0, 0, 0, 0, 0, 0, 0, // Rank3
    5, 5, 5, 5, 5, 5, 5, 5, // Rank4
    10, 10, 10, 10, 10, 10, 10, 10, // Rank5
    20, 20, 20, 20, 20, 20, 20, 20, // Rank6
    40, 40, 40, 40, 40, 40, 40, 40, // Rank7
    40, 40, 40, 40, 40, 40, 40, 40, // Rank8
];

pub const BLACK_PAWN_HEURISTICS: [i32; 64] = [
    36, 40, 40, 40, 40, 40, 40, 36, // Rank1
    30, 35, 35, 35, 35, 35, 35, 30, // Rank2
    8, 10, 10, 15, 15, 10, 10, 8, // Rank3
    -5, 5, 15, 20, 20, 15, 5, -5, // Rank4
    5, -25, 20, 30, 30, 20, -3, 5, // Rank5
    -5, 0, -35, 5, 5, 10, 0, -5, // Rank6
    10, 10, 10, 0, 0, 10, 10, 10, // Rank7
    0, 0, 0, 0, 0, 0, 0, 0, // Rank8
];

pub const ENDGAME_BLACK_PAWN_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = ENDGAME_WHITE_PAWN_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_KNIGHT_HEURISTICS: [i32; 64] = [
    -30, -21, -15, -6, -6, -15, -21, -30, // Rank1
    -21, -9, 6, 9, 9, 6, -9, -21, // Rank2
    -12, 9, 15, 6, 6, 15, 9, -12, // Rank3
    -9, 6, 12, 21, 21, 12, 6, -9, // Rank4
    -9, 6, 10, 18, 18, 10, 6, -9, // Rank5
    -12, 9, 15, 6, 6, 15, 9, -12, // Rank6
    -21, -9, 6, 9, 9, 6, -9, -21, // Rank7
    -30, -21, -15, -6, -6, -15, -21, -30, // Rank8
];

pub const BLACK_KNIGHT_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_KNIGHT_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_BISHOP_HEURISTICS: [i32; 64] = [
    -10, -15, -10, -5, -5, -10, -15, -10, // Rank1
    -3, 20, 0, 7, 7, 0, 20, -3, // Rank2
    7, 9, 7, 12, 12, 7, 9, 7, // Rank3
    -6, 6, 14, 7, 7, 14, 6, -6, // Rank4
    -8, 10, 7, 6, 6, 7, 10, -8, // Rank5
    6, 0, -6, 7, 7, -6, 0, 6, // Rank6
    -10, 5, -7, -1, -1, -7, 5, -10, // Rank7
    -12, -20, -12, -7, -7, -12, -20, -12, // Rank8
];

pub const BLACK_BISHOP_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_BISHOP_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_QUEEN_HEURISTICS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, // Rank1
    -15, -10, -7, 0, 0, -7, -10, -15, // Rank2
    -15, -5, -5, 5, 5, -5, -5, -15, // Rank3
    -20, -15, -5, 8, 8, -5, -15, -20, // Rank4
    -20, -15, -5, 10, 10, -5, -15, -20, // Rank5
    -15, -5, -5, 0, 0, -5, -5, -15, // Rank6
    -10, -3, 0, 0, 0, 0, -3, -10, // Rank7
    -10, -10, -10, -10, -10, -10, -10, -10, // Rank8
];

pub const BLACK_QUEEN_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_QUEEN_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_ROOK_HEURISTICS: [i32; 64] = [
    5, -8, 7, 20, 20, 7, -8, 5, // Rank 1
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 2
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 3
    0, 0, 0, 8, 8, 0, 0, 0, // Rank 4
    0, 0, 0, 8, 8, 0, 0, 0, // Rank 5
    0, 0, 0, 0, 0, 0, 0, 0, // Rank 6
    20, 30, 30, 30, 30, 30, 30, 20, // Rank 7
    10, 10, 10, 10, 10, 10, 10, 10, // Rank 8
];

pub const BLACK_ROOK_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_ROOK_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_KING_HEURISTICS: [i32; 64] = [
    10, 15, -10, -10, -10, -10, 15, 10, // Rank 1
    -50, -50, -50, -50, -50, -50, -50, -50, // Rank 2
    -75, -75, -75, -75, -75, -75, -75, -75, // Rank 3
    -100, -90, -80, -75, -75, -80, -90, -100, // Rank 4
    -40, -35, -100, -90, -90, -100, -35, -40, // Rank 5
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
    -50, -40, -30, -30, -30, -30, -40, -50, // Rank 1
    -40, -30, -15, -8, -8, -15, -30, -40, // Rank 2
    -30, -15, -4, 0, 0, -4, -15, -30, // Rank 3
    -30, -8, 0, 2, 2, 0, -8, -30, // Rank 4
    -30, -8, 0, 2, 2, 0, -8, -30, // Rank 5
    -30, -15, -4, 0, 0, -4, -15, -30, // Rank 6
    -40, -30, -15, -8, -8, -40, -30, -40, // Rank 7
    -50, -40, -50, -50, -50, -50, -40, -50, // Rank 8
];

pub const ENDGAME_BLACK_KING_HEURISTICS: [i32; 64] = WHITE_KING_HEURISTICS;

// used to punish bad pawn structure
pub const OPENING_BAD_STRUCTURE_ANTIBONUS: i32 = 3;
pub const MIDDLEGAME_BAD_STRUCTURE_ANTIBONUS: i32 = 4;
pub const ENDGAME_BAD_STRUCTURE_ANTIBONUS: i32 = 5;
