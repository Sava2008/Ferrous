pub mod board;
pub mod pieces;
pub mod state_enums;

use crate::{
    constants::{COORDS, SQUARE_SIDE},
    game_logic::pieces::{ChessPiece, Piece, Void},
    helper_functions::{coords_to_index, load_images},
};
pub use board::Board;
pub use ggez::{
    Context, GameResult,
    event::EventHandler,
    graphics::{Color, DrawParam, GlBackendSpec, ImageGeneric, Mesh, MeshBuilder, clear, draw},
    input::mouse,
    mint::Point2,
};
use state_enums::{GameMode, PieceColor, PieceVariant};
use std::collections::HashMap;

pub struct MainState {
    pub gamemode: GameMode,
    pub board: Board,
    pub pieces_images: HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>,
    pub mouse_clicked: bool,
    pub mouse_pressed: bool,
    pub mouse_pos: Point2<f32>,
    pub selected: Option<usize>,
    pub destination: Option<usize>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        return Ok(MainState {
            gamemode: GameMode::SelectionWhite,
            board: Board::new(ctx)?,
            pieces_images: load_images(ctx)?,
            mouse_clicked: false,
            mouse_pressed: false,
            mouse_pos: Point2 { x: 0., y: 0. },
            selected: None,
            destination: None,
        });
    }

    fn draw_pieces(&mut self, ctx: &mut Context) -> GameResult {
        for (i, square) in self.board.squares.iter().enumerate() {
            if square.is_piece() {
                draw(
                    ctx,
                    self.pieces_images
                        .get(
                            &(if let Ok(x) = square.key() {
                                x
                            } else {
                                continue;
                            }),
                        )
                        .unwrap(),
                    DrawParam::default()
                        .dest(Point2 {
                            x: COORDS[i].1 as f32 * SQUARE_SIDE,
                            y: COORDS[i].0 as f32 * SQUARE_SIDE,
                        })
                        .scale([0.05, 0.05]),
                )?;
            }
        }
        return Ok(());
    }

    fn handle_selection(&mut self) -> GameResult {
        self.selected = coords_to_index(self.mouse_pos);
        let selection_idx: usize = if let Some(x) = self.selected {
            x
        } else {
            return Ok(());
        };

        let selected_piece: &ChessPiece = &self.board.squares[selection_idx];

        match (&self.gamemode, selected_piece.is_piece(), selected_piece) {
            (GameMode::SelectionWhite, true, piece) => {
                if piece.color() != Some(PieceColor::White) {
                    self.selected = None;
                } else {
                    self.gamemode = GameMode::MovementWhite;
                }
            }
            (GameMode::SelectionBlack, true, piece) => {
                if piece.color() != Some(PieceColor::Black) {
                    self.selected = None;
                } else {
                    self.gamemode = GameMode::MovementBlack;
                }
            }
            _ => {
                self.selected = None;
            }
        };

        return Ok(());
    }

    fn handle_movement(&mut self) -> GameResult {
        self.destination = coords_to_index(self.mouse_pos);

        let destination_idx: usize = match self.destination {
            Some(i) => i,
            None => return Ok(()),
        };
        let selection_idx: usize = match self.selected {
            Some(i) => i,
            None => return Ok(()),
        };
        match &self.board.squares[selection_idx] {
            ChessPiece::P(p) => {
                if !p
                    .legal_moves(&self.board)
                    .iter()
                    .any(|i: &usize| i == &destination_idx)
                {
                    (self.selected, self.destination) = (None, None);
                    match self.gamemode {
                        GameMode::MovementBlack => self.gamemode = GameMode::SelectionBlack,
                        GameMode::MovementWhite => self.gamemode = GameMode::SelectionWhite,
                        _ => (),
                    }
                    return Ok(());
                }
            }
            _ => (),
        };

        if selection_idx >= 64 || destination_idx >= 64 {
            (self.selected, self.destination) = (None, None);
            return Ok(());
        }

        if self.board.squares[selection_idx].color() != self.board.squares[destination_idx].color()
        {
            self.board.squares[selection_idx].new_idx(destination_idx);
            let moving_piece: ChessPiece = std::mem::replace(
                &mut self.board.squares[selection_idx],
                ChessPiece::Square(Void),
            );

            let _ = std::mem::replace(&mut self.board.squares[destination_idx], moving_piece);
            match self.gamemode {
                GameMode::MovementBlack => self.gamemode = GameMode::SelectionWhite,
                GameMode::MovementWhite => self.gamemode = GameMode::SelectionBlack,
                _ => (),
            }
        } else {
            match self.gamemode {
                GameMode::MovementBlack => self.gamemode = GameMode::SelectionBlack,
                GameMode::MovementWhite => self.gamemode = GameMode::SelectionWhite,
                _ => (),
            }
        }

        (self.selected, self.destination) = (None, None);
        return Ok(());
    }

    fn handle_input(&mut self, ctx: &mut Context) -> GameResult {
        self.mouse_pressed = mouse::button_pressed(ctx, mouse::MouseButton::Left);
        if self.mouse_pressed && !self.mouse_clicked {
            self.mouse_pos = mouse::position(ctx);
            if self.mouse_pos.x > 8. * SQUARE_SIDE || self.mouse_pos.y > 8. * SQUARE_SIDE {
                return Ok(());
            }
            match self.gamemode {
                GameMode::MovementBlack | GameMode::MovementWhite => {
                    self.destination = coords_to_index(self.mouse_pos)
                }
                GameMode::SelectionBlack | GameMode::SelectionWhite => {
                    self.selected = coords_to_index(self.mouse_pos);
                }
            };

            match self.gamemode {
                GameMode::SelectionWhite | GameMode::SelectionBlack => self.handle_selection(),
                GameMode::MovementWhite | GameMode::MovementBlack => self.handle_movement(),
            }?;
        }

        self.mouse_clicked = self.mouse_pressed;
        return Ok(());
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx)?;
        return Ok(());
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::YELLOW);

        draw(ctx, &self.board.board_mesh, DrawParam::default())?;
        self.draw_pieces(ctx)?;

        ggez::graphics::present(ctx)?;
        return Ok(());
    }
}
