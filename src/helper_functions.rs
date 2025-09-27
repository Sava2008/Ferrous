use crate::constants::{BOARD_AREA, BOARD_SIDE, SQUARE_SIDE};
use crate::game_logic::{
    pieces::{ChessPiece, Void},
    state_enums::{PieceColor, PieceVariant},
};

use ggez::mint::Point2;
use ggez::{
    Context, GameResult,
    graphics::{GlBackendSpec, Image, ImageGeneric},
};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub const fn generate_coords() -> [(u8, u8); BOARD_AREA] {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut idx: usize = 0;
    let mut coords: [(u8, u8); BOARD_AREA] = [(69, 69); BOARD_AREA];
    while y < BOARD_SIDE {
        coords[idx] = (y, x);
        idx += 1;
        x += 1;
        if x == BOARD_SIDE {
            y += 1;
            x = 0;
        }
    }

    return coords;
}

pub fn generate_empty_board() -> [ChessPiece; BOARD_AREA] {
    return std::array::from_fn(|_| ChessPiece::Square(Void {}));
}

pub fn load_images(
    ctx: &mut Context,
) -> GameResult<HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>> {
    return Ok(HashMap::from([
        (
            (PieceColor::Black, PieceVariant::B),
            Image::new(ctx, "/black_bishop.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::K),
            Image::new(ctx, "/black_king.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::N),
            Image::new(ctx, "/black_knight.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::P),
            Image::new(ctx, "/black_pawn.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::Q),
            Image::new(ctx, "/black_queen.png")?,
        ),
        (
            (PieceColor::Black, PieceVariant::R),
            Image::new(ctx, "/black_rook.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::B),
            Image::new(ctx, "/white_bishop.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::K),
            Image::new(ctx, "/white_king.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::N),
            Image::new(ctx, "/white_knight.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::P),
            Image::new(ctx, "/white_pawn.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::Q),
            Image::new(ctx, "/white_queen.png")?,
        ),
        (
            (PieceColor::White, PieceVariant::R),
            Image::new(ctx, "/white_rook.png")?,
        ),
    ]));
}

pub fn coords_to_index(coords: Point2<f32>) -> Option<usize> {
    return Some(
        coords.y as usize / SQUARE_SIDE as usize * 8 + coords.x as usize / SQUARE_SIDE as usize,
    );
}

pub fn index_to_coords<'a>(index: usize) -> (u8, u8) {
    return (index as u8 / 8, index as u8 % 8);
}

pub fn is_diagonal(idx1: usize, idx2: usize) -> bool {
    let coords1: (u8, u8) = index_to_coords(idx1);
    let coords2: (u8, u8) = index_to_coords(idx2);

    return max(coords1.0, coords2.0) - min(coords1.0, coords2.0)
        == max(coords1.1, coords2.1) - min(coords1.1, coords2.1);
}
