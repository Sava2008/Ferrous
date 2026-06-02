use crate::board_visual::{BoardVisual, SQUARE_SIZE};
use macroquad::prelude::*;

pub fn calculate_coords(point: u16) -> (f32, f32) {
    // x, y
    return ((point % 8) as f32, (7 - (point / 8)) as f32);
}
pub fn calculate_index(point1: u16, point2: u16) -> u16 {
    let side = SQUARE_SIZE as u16;
    let x = point1 / side;
    let y = point2 / side;

    let row = 7 - y;
    let index = row * 8 + x;

    return index;
}

const NORMAL_MOVE_HIGHLIGHT_COLOR_ALPHA1: Color = Color::from_hex(0x003300);
const CAPTURE_HIGHLIGHT_COLOR_ALPHA1: Color = Color::from_hex(0x990000);
const EP_HIGHLIGHT_COLOR_ALPHA1: Color = Color::from_hex(0xCC0066);
const CASTLING_HIGHLIGHT_COLOR_ALPHA1: Color = Color::from_hex(0x000066);
const SELECTION_COLOR_ALPHA1: Color = Color::from_hex(0x4500BC);

const NORMAL_MOVE_HIGHLIGHT_COLOR: Color = NORMAL_MOVE_HIGHLIGHT_COLOR_ALPHA1.with_alpha(0.9);
const CAPTURE_HIGHLIGHT_COLOR: Color = CAPTURE_HIGHLIGHT_COLOR_ALPHA1.with_alpha(0.9);
const EP_HIGHLIGHT_COLOR: Color = EP_HIGHLIGHT_COLOR_ALPHA1.with_alpha(0.95);
const CASTLING_HIGHLIGHT_COLOR: Color = CASTLING_HIGHLIGHT_COLOR_ALPHA1.with_alpha(0.95);
const SELECTION_COLOR: Color = SELECTION_COLOR_ALPHA1.with_alpha(0.95);

impl BoardVisual {
    pub fn highlight_legal_moves(&self) -> () {
        for (from, to) in &self.captures {
            if *from != self.selected_square {
                continue;
            }
            let (x, y) = calculate_coords(*to);
            draw_rectangle(
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                CAPTURE_HIGHLIGHT_COLOR,
            );
        }
        for (from, to) in &self.normal_moves {
            if *from != self.selected_square {
                continue;
            }
            let (x, y) = calculate_coords(*to);
            draw_rectangle(
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                NORMAL_MOVE_HIGHLIGHT_COLOR,
            );
        }
        for (from, to) in &self.castlings {
            if *from != self.selected_square {
                continue;
            }
            let (x, y) = calculate_coords(*to);
            draw_rectangle(
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                CASTLING_HIGHLIGHT_COLOR,
            );
        }
        if self.en_passant.1 != 64 && self.en_passant.0 == self.selected_square {
            let (x, y) = calculate_coords(self.en_passant.1);
            draw_rectangle(
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                EP_HIGHLIGHT_COLOR,
            );
        }
        if self.selected_square != 64 {
            let (x, y) = calculate_coords(self.selected_square);
            draw_rectangle(
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                SELECTION_COLOR,
            );
        }
    }
}
