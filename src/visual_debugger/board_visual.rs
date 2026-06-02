use crate::{
    board::Board, board_geometry_templates::*, board_highlight::calculate_coords,
    gamestate::GameState, moves::MoveList,
};
use macroquad::prelude::*;

pub const SQUARE_SIZE: f32 = 100.;
pub const LIGHT_SQUARE_COLOR: Color = Color::from_hex(0xD8AE9F);
pub const DARK_SQUARE_COLOR: Color = Color::from_hex(0xA22B00);

#[derive(Debug)]
pub struct BoardVisual {
    pub squares: [u16; 64],
    pub normal_moves: Vec<(u16, u16)>,
    pub captures: Vec<(u16, u16)>,
    pub en_passant: (u16, u16),
    pub castlings: Vec<(u16, u16)>,
    pub selected_square: u16,
}

impl BoardVisual {
    pub fn set_pieces(&mut self, regular_board: &Board, state: &GameState) -> () {
        self.squares = regular_board.cached_pieces.clone();
        if let Some(ep) = state.en_passant_target {
            self.en_passant.1 = ep as u16;
        }
    }

    pub fn get_moves(&mut self, regular_board: &Board, game_state: &GameState, color: u16) -> () {
        let mut legal_moves: MoveList = MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        };
        if color == 8 {
            regular_board.pawn_moves(&game_state, 8, &mut legal_moves, false);
            regular_board.knight_moves(8, &mut legal_moves, &game_state, false);
            regular_board.bishop_moves(8, &mut legal_moves, &game_state, false);
            regular_board.rook_moves(8, &mut legal_moves, &game_state, false);
            regular_board.queen_moves(8, &mut legal_moves, &game_state, false);
            regular_board.king_moves(&game_state, 8, &mut legal_moves, false);
        } else {
            regular_board.pawn_moves(&game_state, 16, &mut legal_moves, false);
            regular_board.knight_moves(16, &mut legal_moves, &game_state, false);
            regular_board.bishop_moves(16, &mut legal_moves, &game_state, false);
            regular_board.rook_moves(16, &mut legal_moves, &game_state, false);
            regular_board.queen_moves(16, &mut legal_moves, &game_state, false);
            regular_board.king_moves(&game_state, 16, &mut legal_moves, false);
        };
        for i in 0..legal_moves.first_not_occupied {
            let m: u16 = legal_moves.pseudo_moves[i];
            let (from, to, flag) = (
                from_square(m) as u16,
                to_square(m) as u16,
                (m & MARK_MASK) >> MARK_SHIFT,
            );
            if flag == 1 {
                self.castlings.push((from, to));
                continue;
            }
            if flag == 2 {
                self.en_passant = (from, to);
                continue;
            }
            if self.squares[to as usize] != 0 {
                self.captures.push((from, to));
                continue;
            }
            self.normal_moves.push((from, to));
        }
    }
    pub fn draw_pieces(&self, images: &[Texture2D; 12]) -> () {
        for (idx, piece) in self.squares.iter().enumerate() {
            if *piece == 0 {
                continue;
            }
            let (x, y) = calculate_coords(idx as u16);
            draw_texture_ex(
                &images[*piece as usize - 1],
                x * SQUARE_SIZE,
                y * SQUARE_SIZE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SQUARE_SIZE, SQUARE_SIZE)),
                    ..Default::default()
                },
            );
        }
    }
}

pub fn draw_board() -> () {
    for sq in 0..64 {
        let (col, row) = calculate_coords(sq);
        let square_color = match (sq % 2, (row as u8) % 2) {
            (0, 0) | (1, 1) => LIGHT_SQUARE_COLOR,
            (1, 0) | (0, 1) => DARK_SQUARE_COLOR,
            _ => unreachable!(),
        };
        draw_rectangle(
            col * SQUARE_SIZE,
            row * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            square_color,
        );
    }
}
