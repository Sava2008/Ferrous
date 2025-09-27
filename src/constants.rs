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
