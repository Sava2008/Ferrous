pub mod board;
pub mod pieces;
pub mod state_enums;

use crate::{
    constants::{COORDS, SQUARE_SIDE},
    engine::Engine,
    helper_functions::{coords_to_index, load_images},
};
pub use board::Board;
use ggez::graphics;
pub use ggez::{
    Context, GameResult,
    event::EventHandler,
    graphics::{
        Color, DrawMode, DrawParam, GlBackendSpec, ImageGeneric, Mesh, MeshBuilder, Rect, clear,
        draw,
    },
    input::mouse,
    mint::Point2,
};
use state_enums::{GameMode, PieceColor, PieceVariant};
use std::collections::HashMap;

pub struct MainState {
    pub engine: Engine,
    pub legal_move_mesh: Mesh,
    pub board_mesh: Mesh,
    pub board: Board,
    pub pieces_images: HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>,
    pub mouse_clicked: bool,
    pub mouse_pressed: bool,
    pub mouse_pos: Point2<f32>,
    pub player_side: PieceColor,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut mesh_builder: MeshBuilder = MeshBuilder::new();
        for (x, y) in COORDS {
            mesh_builder.rectangle(
                DrawMode::fill(),
                Rect::new(
                    x as f32 * SQUARE_SIDE,
                    y as f32 * SQUARE_SIDE,
                    SQUARE_SIDE as f32,
                    SQUARE_SIDE as f32,
                ),
                match x % 2 == y % 2 {
                    true => Color::from_rgb(250, 230, 250),
                    false => Color::from_rgb(70, 50, 130),
                },
            )?;
        }
        return Ok(MainState {
            engine: Engine::new(PieceColor::Black),
            legal_move_mesh: Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: 0., y: 0. },
                10.,
                0.4,
                graphics::Color::RED,
            )?,
            board_mesh: mesh_builder.build(ctx)?,
            board: Board::new()?,
            pieces_images: load_images(ctx)?,
            mouse_clicked: false,
            mouse_pressed: false,
            mouse_pos: Point2 { x: -5., y: -5. },
            player_side: PieceColor::White,
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

    fn legit_move(&self, selection_idx: usize, destination_idx: usize) -> bool {
        if selection_idx > 63 || destination_idx > 63 {
            return false;
        }
        if !self
            .board
            .legal_moves
            .iter()
            .any(|idx: &usize| idx == &destination_idx)
        {
            return false;
        }
        return true;
    }

    fn reset_mainstate(&mut self, destination_idx: usize, successful_move: bool) -> GameResult {
        if successful_move {
            match self.board.gamemode {
                GameMode::MovementBlack => {
                    self.board
                        .black_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.black_vision()?;
                    self.board.gamemode = GameMode::SelectionWhite;
                }
                GameMode::MovementWhite => {
                    self.board
                        .white_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.white_vision()?;
                    self.board.gamemode = GameMode::SelectionBlack;
                }
                _ => (),
            }
        } else {
            match self.board.gamemode {
                GameMode::MovementBlack => self.board.gamemode = GameMode::SelectionBlack,
                GameMode::MovementWhite => self.board.gamemode = GameMode::SelectionWhite,
                _ => (),
            }
        }
        return Ok(());
    }

    fn handle_movement(&mut self) -> GameResult {
        match self.board.gamemode {
            GameMode::MovementBlack => self.board.black_vision.clear(),
            GameMode::MovementWhite => self.board.white_vision.clear(),
            _ => unreachable!(),
        };
        self.board.dest_square = match self.board.gamemode {
            GameMode::MovementWhite => coords_to_index(self.mouse_pos),
            GameMode::MovementBlack => Some(self.board.engine_move.unwrap().1),
            _ => unreachable!(),
        };

        let destination_idx: usize = match self.board.dest_square {
            Some(i) => i,
            None => return Ok(()),
        };
        let selection_idx: usize = match self.board.chosen_piece {
            Some(i) => i,
            None => return Ok(()),
        };

        if !self.legit_move(selection_idx, destination_idx) {
            self.board.legal_moves.clear();
            self.reset_mainstate(destination_idx, false)?;
            return Ok(());
        }

        if &self.board.squares[selection_idx].color()
            != &self.board.squares[destination_idx].color()
        {
            self.board.perform_move(
                selection_idx,
                destination_idx,
                self.board.en_peasant_susceptible,
                match self.board.gamemode {
                    GameMode::MovementBlack => PieceColor::Black,
                    GameMode::MovementWhite => PieceColor::White,
                    _ => unreachable!(),
                },
            )?;
            self.reset_mainstate(destination_idx, true)?;
            self.board
                .is_check(self.board.squares[destination_idx].color().unwrap());
        } else {
            self.reset_mainstate(destination_idx, false)?;
        }
        (
            self.board.chosen_piece,
            self.board.dest_square,
            self.board.legal_moves,
        ) = (None, None, Vec::new());
        return Ok(());
    }

    fn handle_selection(&mut self) -> GameResult {
        match self.board.gamemode {
            GameMode::SelectionBlack => {
                (self.board.engine_move) = Some(
                    self.engine
                        .find_best_move(
                            &self.board,
                            &&self.board.check,
                            self.board.en_peasant_susceptible,
                        )
                        .unwrap(),
                )
            }
            _ => (),
        };

        self.board.chosen_piece = match self.board.gamemode {
            GameMode::SelectionBlack => Some(self.board.engine_move.unwrap().0),
            GameMode::SelectionWhite => coords_to_index(self.mouse_pos),
            _ => unreachable!(),
        };
        self.board.handle_selection()?;
        return Ok(());
    }

    fn handle_input(&mut self, ctx: &mut Context) -> GameResult {
        self.mouse_pressed = mouse::button_pressed(ctx, mouse::MouseButton::Left);
        match self.board.gamemode {
            GameMode::SelectionWhite => Ok(()),
            GameMode::SelectionBlack => self.handle_selection(),
            GameMode::MovementWhite => Ok(()),
            GameMode::MovementBlack => self.handle_movement(),
            GameMode::BlackWin | GameMode::Draw | GameMode::WhiteWin => return Ok(()),
        }?;
        if self.mouse_pressed && !self.mouse_clicked {
            self.mouse_pos = mouse::position(ctx);
            if self.mouse_pos.x > 8. * SQUARE_SIDE || self.mouse_pos.y > 8. * SQUARE_SIDE {
                return Ok(());
            }

            match self.board.gamemode {
                GameMode::SelectionWhite => self.handle_selection(),
                GameMode::MovementWhite => self.handle_movement(),
                _ => return Ok(()),
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
        draw(ctx, &self.board_mesh, DrawParam::default())?;
        self.draw_pieces(ctx)?;
        if self.board.gamemode == GameMode::MovementWhite {
            for m in &self.board.legal_moves {
                graphics::draw(
                    ctx,
                    &self.legal_move_mesh,
                    DrawParam::default().dest(Point2 {
                        x: (*m % 8 * SQUARE_SIDE as usize + SQUARE_SIDE as usize / 2) as f32,
                        y: (*m / 8 * SQUARE_SIDE as usize + SQUARE_SIDE as usize / 2) as f32,
                    }),
                )?
            }
        }

        ggez::graphics::present(ctx)?;
        return Ok(());
    }
}
