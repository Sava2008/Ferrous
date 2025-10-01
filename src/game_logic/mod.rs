pub mod board;
pub mod pieces;
pub mod state_enums;

use crate::{
    constants::{COORDS, SQUARE_SIDE},
    game_logic::pieces::{ChessPiece, Void},
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
use std::collections::{HashMap, HashSet};

pub struct MainState {
    pub max_id: u8,
    pub gamemode: GameMode,
    pub board: Board,
    pub pieces_images: HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>,
    pub mouse_clicked: bool,
    pub mouse_pressed: bool,
    pub mouse_pos: Point2<f32>,
    pub selected: Option<usize>,
    pub destination: Option<usize>,
    pub en_peasant_susceptible: Option<usize>,
    pub legal_moves: Vec<usize>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        return Ok(MainState {
            max_id: 0,
            gamemode: GameMode::SelectionWhite,
            board: Board::new(ctx)?,
            pieces_images: load_images(ctx)?,
            mouse_clicked: false,
            mouse_pressed: false,
            mouse_pos: Point2 { x: 0., y: 0. },
            selected: None,
            destination: None,
            en_peasant_susceptible: None,
            legal_moves: Vec::new(),
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

        let selected_piece = &self.board.squares[selection_idx];
        match selected_piece {
            ChessPiece::Square(_) => (),
            piece => {
                self.legal_moves = piece.legal_moves(&self.board, self.en_peasant_susceptible)?;
            }
        };

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
        match self.gamemode {
            GameMode::MovementBlack => self.board.black_vision.clear(),
            GameMode::MovementWhite => self.board.white_vision.clear(),
            _ => unreachable!(),
        }
        self.destination = coords_to_index(self.mouse_pos);

        let destination_idx: usize = match self.destination {
            Some(i) => i,
            None => return Ok(()),
        };
        let selection_idx: usize = match self.selected {
            Some(i) => i,
            None => return Ok(()),
        };

        if !self
            .legal_moves
            .iter()
            .any(|idx: &usize| idx == &destination_idx)
        {
            (self.selected, self.destination, self.legal_moves) = (None, None, Vec::new());
            match self.gamemode {
                GameMode::MovementBlack => self.gamemode = GameMode::SelectionBlack,
                GameMode::MovementWhite => self.gamemode = GameMode::SelectionWhite,
                _ => (),
            }
            return Ok(());
        }

        if selection_idx >= 64 || destination_idx >= 64 {
            (self.selected, self.destination, self.legal_moves) = (None, None, Vec::new());
            return Ok(());
        }

        if &self.board.squares[selection_idx].color()
            != &self.board.squares[destination_idx].color()
        {
            self.board.squares[selection_idx].new_idx(destination_idx);
            if let ChessPiece::P(p) = &self.board.squares[selection_idx] {
                if p.moved_two_squares(selection_idx) {
                    self.en_peasant_susceptible = match p.key.0 {
                        PieceColor::Black => Some(destination_idx - 8),
                        PieceColor::White => Some(destination_idx + 8),
                    }
                } else {
                    self.en_peasant_susceptible = None;
                }
            } else {
                self.en_peasant_susceptible = None;
            }
            let moving_piece: ChessPiece = std::mem::replace(
                &mut self.board.squares[selection_idx],
                ChessPiece::Square(Void),
            );

            let _ = std::mem::replace(&mut self.board.squares[destination_idx], moving_piece);
            match self.gamemode {
                GameMode::MovementBlack => {
                    self.board
                        .black_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.black_vision(self.en_peasant_susceptible)?;
                    println!("black vision = {:?}", self.board.black_vision);
                    self.gamemode = GameMode::SelectionWhite;
                }
                GameMode::MovementWhite => {
                    self.board
                        .white_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.white_vision(self.en_peasant_susceptible)?;
                    println!("white vision = {:?}", self.board.white_vision);
                    self.gamemode = GameMode::SelectionBlack;
                    println!("white vision = {:?}", self.board.white_vision);
                }
                _ => (),
            }
        } else {
            match self.gamemode {
                GameMode::MovementBlack => self.gamemode = GameMode::SelectionBlack,
                GameMode::MovementWhite => self.gamemode = GameMode::SelectionWhite,
                _ => (),
            }
        }
        (self.selected, self.destination, self.legal_moves) = (None, None, Vec::new());
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
        clear(ctx, Color::from_rgb(198, 131, 70));

        draw(ctx, &self.board.board_mesh, DrawParam::default())?;
        self.draw_pieces(ctx)?;

        ggez::graphics::present(ctx)?;
        return Ok(());
    }
}
