use crate::game_logic::state_enums::{DiagonalDirection, LinearDirection};
use crate::helper_functions::generate_coords;

pub const WIN_SCALES: (f32, f32) = (900., 700.);
pub const ROOK_VALUE: u16 = 50;
pub const KNIGHT_VALUE: u16 = 25;
pub const BISHOP_VALUE: u16 = 30;
pub const PAWN_VALUE: u16 = 10;
pub const QUUEN_VALUE: u16 = 90;
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
