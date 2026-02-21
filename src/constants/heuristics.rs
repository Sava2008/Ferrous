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

pub const BLACK_PAWN_HEURISTICS: [i32; 64] = {
    let mut temp: [i32; 64] = WHITE_PAWN_HEURISTICS;
    temp.reverse();
    temp
};

pub const WHITE_KNIGHT_HEURISTICS: [i32; 64] = [
    -10, -7, -5, -2, -2, -5, -7, -10, // Rank1
    -7, -3, 2, 3, 3, 2, -3, -7, // Rank2
    -4, 4, 5, 2, 2, 6, 4, -4, // Rank3
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
    5, 10, -10, -5, -5, -10, 10, 5, // Rank 1
    -40, -50, -50, -50, -50, -50, -50, -40, // Rank 2
    -50, -40, -40, -30, -30, -40, -40, -50, // Rank 3
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

pub const ENDGAME_KING_HEURISTICS: [i32; 64] = [
    -20, -15, -10, -5, -5, -10, -15, -20, -15, -10, -5, 0, 0, -5, -10, -15, -10, 20, 25, 30, 30,
    25, 20, -10, 0, 20, 30, 40, 40, 30, 20, 0, 0, 20, 30, 40, 40, 30, 20, 0, -10, 20, 25, 30, 30,
    25, 20, -10, -15, -10, -5, 0, 0, -5, -10, -15, -20, -15, -10, -5, -5, -10, -15, -20,
];
