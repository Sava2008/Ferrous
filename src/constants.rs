use crate::{
    game_logic::state_enums::{DiagonalDirection, LinearDirection},
    helper_functions::generate_coords,
};

pub const WIN_SCALES: (f32, f32) = (900., 700.);
pub const ROOK_VALUE: u16 = 50;
pub const KNIGHT_VALUE: u16 = 25;
pub const BISHOP_VALUE: u16 = 30;
pub const PAWN_VALUE: u16 = 10;
pub const QUUEN_VALUE: u16 = 100;
pub const KING_VALUE: u16 = 1000;

pub const BOARD_SIDE: u8 = 8;
pub const BOARD_AREA: usize = 64;
pub const SQUARE_SIDE: f32 = 75.;

pub const COORDS: [(u8, u8); BOARD_AREA] = generate_coords();

pub const LINEAR_DIRECTIONS: [LinearDirection; 4] = [
    LinearDirection::FileUp,
    LinearDirection::FileDown,
    LinearDirection::RankLeft,
    LinearDirection::RankRight,
];

pub const DIAGONAL_DIRECTIONS: [DiagonalDirection; 4] = [
    DiagonalDirection::UpRight,
    DiagonalDirection::UpLeft,
    DiagonalDirection::DownRight,
    DiagonalDirection::DownLeft,
];

pub const KNIGHT_DELTAS: [(i8, i8); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];
pub const KING_DELTAS: [(i8, i8); 8] = [
    (1, 1),
    (1, 0),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (1, -1),
];

pub const PAWN_HEURISTICS: [i16; 64] = [
    8, 10, 10, 10, 10, 10, 10, 8, 7, 9, 9, 9, 9, 9, 9, 7, 0, 4, 4, 4, 4, 4, 4, 0, 2, 2, 5, 6, 6, 5,
    2, 2, -2, -3, 9, 10, 10, 9, -3, -2, -3, -2, -5, 1, 1, -5, -2, -3, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0,
    0, 0, 0, 0, 0, 0,
];

pub const KNIGHT_HEURISTICS: [i16; 64] = [
    -10, -9, -7, -7, -7, -7, -9, -10, -9, -7, 5, 2, 2, 5, -7, -9, -7, 5, 5, 2, 2, 5, 5, -7, -7, 2,
    6, 10, 10, 6, 2, -7, -7, 2, 7, 10, 10, 8, 2, -7, -7, 0, 5, 2, 2, 5, 0, -7, -9, -7, 1, 2, 2, 1,
    -7, -9, -10, -9, -7, -7, -7, -7, -9, -10,
];

pub const BISHOP_HEURISTICS: [i16; 64] = [
    -10, -15, -15, -5, -5, -15, -15, -10, -10, 10, 0, 5, 5, 0, 10, -10, 5, 5, 10, 10, 10, 10, 5, 5,
    -5, 5, 7, 15, 15, 7, 5, -5, -5, 5, 10, 15, 15, 10, 5, -5, 5, 0, -5, 10, 10, -5, 0, 5, -15, 10,
    -10, 10, 10, -10, 10, -15, -10, -15, -15, -10, -10, -15, -15, -10,
];

pub const QUEEN_HEURISTICS: [i16; 64] = [
    -12, -10, -5, 0, 0, -5, -10, -12, -10, -5, 0, 0, 0, 0, -5, -10, -5, 0, 0, 5, 5, 0, 0, -5, 0, 0,
    5, 7, 7, 5, 0, -5, 0, 0, 5, 10, 10, 5, 0, -5, -5, 0, 0, 10, 10, 5, 0, -5, -10, -5, 0, 2, 2, 0,
    -5, -10, -12, -10, -5, 0, 0, -5, -10, -12,
];

pub const ROOK_HEURISTICS: [i16; 64] = [
    0, 0, 5, 10, 10, 5, 0, 0, 12, 15, 15, 15, 15, 15, 15, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, -5, 10,
    10, 10, 10, -5, 5,
];

pub const OPENING_MIDDLEGAME_KING_HEURISTICS: [i16; 64] = [
    -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300,
    -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300, -300,
    -200, -200, -200, -200, -200, -200, -200, -200, -150, -150, -150, -150, -150, -150, -150, -150,
    -10, -10, -15, -20, -20, -15, -10, -10, -5, 10, -10, -5, -5, -10, 10, -5,
];

pub const ENDGAME_KING_HEURISTICS: [i16; 64] = [
    -20, -15, -10, -5, -5, -10, -15, -20, -15, -10, -5, 0, 0, -5, -10, -15, -10, 20, 25, 30, 30,
    25, 20, -10, 0, 20, 30, 40, 40, 30, 20, 0, 0, 20, 30, 40, 40, 30, 20, 0, -10, 20, 25, 30, 30,
    25, 20, -10, -15, -10, -5, 0, 0, -5, -10, -15, -20, -15, -10, -5, -5, -10, -15, -20,
];
