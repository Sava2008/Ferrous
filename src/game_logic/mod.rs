pub mod board;
pub mod pieces;
pub mod state_enums;

use crate::{
    constants::{COORDS, SQUARE_SIDE},
    engine::Engine,
    game_logic::{pieces::ChessPiece, state_enums::KingChecked},
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
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub struct MainState {
    pub engine: Engine,
    pub engine_moves: Option<(usize, usize)>,
    pub legal_move_mesh: Mesh,
    pub board_mesh: Mesh,
    pub max_id: u8,
    pub board: Board,
    pub pieces_images: HashMap<(PieceColor, PieceVariant), ImageGeneric<GlBackendSpec>>,
    pub mouse_clicked: bool,
    pub mouse_pressed: bool,
    pub mouse_pos: Point2<f32>,
    pub selected: Option<usize>,
    pub destination: Option<usize>,
    pub en_peasant_susceptible: Option<usize>,
    pub legal_moves: Vec<usize>,
    pub check: (KingChecked, Option<usize>, Option<usize>),
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
            engine_moves: None,
            legal_move_mesh: Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect {
                    x: 0.,
                    y: 0.,
                    w: 10.,
                    h: 10.,
                },
                graphics::Color::GREEN,
            )?,
            board_mesh: mesh_builder.build(ctx)?,
            max_id: 0,
            board: Board::new()?,
            pieces_images: load_images(ctx)?,
            mouse_clicked: false,
            mouse_pressed: false,
            mouse_pos: Point2 { x: -5., y: -5. },
            selected: None,
            destination: None,
            en_peasant_susceptible: None,
            legal_moves: Vec::new(),
            check: (KingChecked::None, None, None),
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

    fn any_legal_moves(&mut self, color: PieceColor) -> bool {
        let (map, checked_king_idx) = match color {
            PieceColor::Black => (
                &self.board.black_locations,
                *self.board.black_locations.get(&14).unwrap(),
            ),
            PieceColor::White => (
                &self.board.white_locations,
                *self.board.white_locations.get(&15).unwrap(),
            ),
        };
        for (_, idx) in map {
            let legal_moves: GameResult<Vec<usize>> = self.board.squares[*idx].legal_moves(
                &self.board,
                self.en_peasant_susceptible,
                &self.check,
                checked_king_idx,
            );
            if let Ok(legal_moves_vec) = legal_moves {
                if legal_moves_vec.len() > 0 {
                    return true;
                }
            }
        }
        return false;
    }

    fn handle_selection(&mut self) -> GameResult {
        match self.board.gamemode {
            GameMode::SelectionBlack => {
                (self.engine_moves) = Some(
                    self.engine
                        .find_best_move(&self.board, &self.check, self.en_peasant_susceptible)
                        .unwrap(),
                )
            }
            _ => (),
        };

        self.selected = match self.board.gamemode {
            GameMode::SelectionBlack => Some(self.engine_moves.unwrap().0),
            GameMode::SelectionWhite => coords_to_index(self.mouse_pos),
            _ => unreachable!(),
        };
        let selection_idx: usize = if let Some(x) = self.selected {
            x
        } else {
            return Ok(());
        };

        match &self.board.squares[selection_idx] {
            ChessPiece::Square(_) => (),
            piece => match (&self.board.gamemode, piece.color()) {
                (&GameMode::SelectionWhite, Some(PieceColor::White))
                | (&GameMode::SelectionBlack, Some(PieceColor::Black)) => {
                    self.legal_moves = piece.legal_moves(
                        &self.board,
                        self.en_peasant_susceptible,
                        &self.check,
                        match piece.color().unwrap() {
                            PieceColor::Black => *self.board.black_locations.get(&14).unwrap(),
                            PieceColor::White => *self.board.white_locations.get(&15).unwrap(),
                        },
                    )?;
                    piece.generate_vision(&self.board);
                }
                _ => return Ok(()),
            },
        };

        match (
            &self.board.gamemode,
            &self.board.squares[selection_idx].is_piece(),
            &self.board.squares[selection_idx],
        ) {
            (GameMode::SelectionWhite, true, piece) => {
                if piece.color() != Some(PieceColor::White) {
                    self.selected = None;
                } else {
                    self.board.gamemode = GameMode::MovementWhite;
                }
            }
            (GameMode::SelectionBlack, true, piece) => {
                if piece.color() != Some(PieceColor::Black) {
                    self.selected = None;
                } else {
                    self.board.gamemode = GameMode::MovementBlack;
                }
            }
            _ => {
                self.selected = None;
            }
        };

        return Ok(());
    }

    fn legit_move(&self, selection_idx: usize, destination_idx: usize) -> bool {
        if selection_idx > 63 || destination_idx > 63 {
            return false;
        }
        if !self
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

    fn reset_en_peasant_target(&mut self, selection_idx: usize, destination_idx: usize) -> () {
        if let ChessPiece::P(p) = &self.board.squares[destination_idx] {
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
    }

    fn handle_movement(&mut self) -> GameResult {
        match self.board.gamemode {
            GameMode::MovementBlack => self.board.black_vision.clear(),
            GameMode::MovementWhite => self.board.white_vision.clear(),
            _ => unreachable!(),
        }
        self.destination = match self.board.gamemode {
            GameMode::MovementWhite => coords_to_index(self.mouse_pos),
            GameMode::MovementBlack => Some(self.engine_moves.unwrap().1),
            _ => unreachable!(),
        };

        let destination_idx: usize = match self.destination {
            Some(i) => i,
            None => return Ok(()),
        };
        let selection_idx: usize = match self.selected {
            Some(i) => i,
            None => return Ok(()),
        };

        if !self.legit_move(selection_idx, destination_idx) {
            self.legal_moves.clear();
            self.reset_mainstate(destination_idx, false)?;
            return Ok(());
        }

        if &self.board.squares[selection_idx].color()
            != &self.board.squares[destination_idx].color()
        {
            self.board.perform_move(
                selection_idx,
                destination_idx,
                self.en_peasant_susceptible,
                match self.board.gamemode {
                    GameMode::MovementBlack => PieceColor::Black,
                    GameMode::MovementWhite => PieceColor::White,
                    _ => unreachable!(),
                },
            )?;
            self.reset_mainstate(destination_idx, true)?;
            self.check = self
                .board
                .is_check(self.board.squares[destination_idx].color().unwrap());
            self.reset_en_peasant_target(selection_idx, destination_idx);
        } else {
            self.reset_mainstate(destination_idx, false)?;
        }
        (self.selected, self.destination, self.legal_moves) = (None, None, Vec::new());
        if !self.any_legal_moves(match self.board.gamemode {
            GameMode::SelectionWhite => PieceColor::White,
            GameMode::SelectionBlack => PieceColor::Black,
            _ => return Ok(()),
        }) {
            match self.check.0 {
                KingChecked::None => {
                    self.board.gamemode = GameMode::Draw;
                    println!("gamemode: {:?}", self.board.gamemode);
                }
                KingChecked::Black => {
                    self.board.gamemode = GameMode::BlackWin;
                    println!("gamemode: {:?}", self.board.gamemode);
                }
                KingChecked::White => {
                    self.board.gamemode = GameMode::WhiteWin;
                    println!("gamemode: {:?}", self.board.gamemode);
                }
            };
        };
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
        for m in &self.legal_moves {
            graphics::draw(
                ctx,
                &self.legal_move_mesh,
                DrawParam::default().dest(Point2 {
                    x: (*m % 8 * SQUARE_SIDE as usize + SQUARE_SIDE as usize / 2) as f32,
                    y: (*m / 8 * SQUARE_SIDE as usize + SQUARE_SIDE as usize / 2) as f32,
                }),
            )?
        }

        ggez::graphics::present(ctx)?;
        return Ok(());
    }
}
