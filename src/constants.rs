use crate::{
    game_logic::state_enums::{DiagonalDirection, LinearDirection},
    helper_functions::generate_coords,
};

pub const WIN_SCALES: (f32, f32) = (900., 700.);
pub const ROOK_VALUE: u16 = 70;
pub const KNIGHT_VALUE: u16 = 40;
pub const BISHOP_VALUE: u16 = 42;
pub const PAWN_VALUE: u16 = 13;
pub const QUUEN_VALUE: u16 = 140;
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
    3, 4, 4, 4, 4, 4, 4, 3, 2, 3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2, 0, 2, 1, 2, 3, 3, 2, 1, 2,
    -1, -3, 3, 4, 4, 3, -3, -1, -2, -1, -4, 1, 1, -4, -1, -2, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 0,
];

pub const KNIGHT_HEURISTICS: [i16; 64] = [
    -5, -3, -2, -2, -2, -2, -3, -5, -3, -2, 2, 2, 2, 2, -2, -3, -2, 1, 2, 2, 2, 2, 1, -2, -2, 2, 3,
    4, 4, 3, 2, -2, -2, 2, 3, 5, 5, 3, 2, -2, -2, 0, 4, 2, 2, 4, 0, -2, -3, -2, 1, 2, 2, 1, -2, -3,
    -5, -3, -2, -2, -2, -2, -3, -5,
];

pub const BISHOP_HEURISTICS: [i16; 64] = [
    -4, -5, -5, -3, -3, -5, -5, -4, -3, 3, 0, 5, 5, 0, 3, -3, 2, 5, 2, 3, 3, 2, 5, 2, -2, 5, 3, 5,
    5, 3, 5, -2, -2, 5, 4, 5, 5, 4, 5, -2, 2, 0, -5, 4, 4, -5, 0, 2, -5, 4, -4, 4, 4, -4, 4, -5,
    -4, -5, -4, -5, -5, -4, -5, -4,
];

pub const QUEEN_HEURISTICS: [i16; 64] = [
    -10, -8, -5, 0, 0, -5, -8, -8, -8, -5, 0, 0, 0, 0, -5, -8, -5, 0, 0, 5, 5, 0, 0, -5, 0, 0, 5,
    5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -5, 0, 0, 5, 5, 5, 0, -5, -10, -5, 0, 2, 2, 0, -5, -8,
    -8, -8, -5, 0, 0, -5, -8, -10,
];

pub const ROOK_HEURISTICS: [i16; 64] = [
    0, 0, 5, 6, 6, 5, 0, 0, 7, 8, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 6, 6, 6, 5, -5,
    5,
];

pub const OPENING_MIDDLEGAME_KING_HEURISTICS: [i16; 64] = [
    -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100,
    -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100,
    -60, -60, -60, -60, -60, -60, -60, -60, -40, -40, -40, -40, -40, -40, -40, -40, -10, -10, -15,
    -20, -20, -15, -10, -10, 5, 10, -10, -5, -5, -10, 10, 5,
];

pub const ENDGAME_KING_HEURISTICS: [i16; 64] = [
    -20, -15, -10, -5, -5, -10, -15, -20, -15, -10, -5, 0, 0, -5, -10, -15, -10, 20, 25, 30, 30,
    25, 20, -10, 0, 20, 30, 40, 40, 30, 20, 0, 0, 20, 30, 40, 40, 30, 20, 0, -10, 20, 25, 30, 30,
    25, 20, -10, -15, -10, -5, 0, 0, -5, -10, -15, -20, -15, -10, -5, -5, -10, -15, -20,
];
