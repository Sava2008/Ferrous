pub mod board;
pub mod pieces;
pub mod state_enums;

use crate::{
    constants::{COORDS, SQUARE_SIDE},
    engine::Engine,
    game_logic::{
        pieces::{ChessPiece, Void},
        state_enums::KingChecked,
    },
    helper_functions::{coords_to_index, load_images},
};
pub use board::Board;
use ggez::graphics;
pub use ggez::{
    Context, GameResult,
    event::EventHandler,
    graphics::{Color, DrawParam, GlBackendSpec, ImageGeneric, Mesh, MeshBuilder, clear, draw},
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
    pub check: (KingChecked, Option<usize>, Option<usize>),
    pub player_side: PieceColor,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
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
            max_id: 0,
            gamemode: GameMode::SelectionWhite,
            board: Board::new(ctx)?,
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
        match self.gamemode {
            GameMode::SelectionBlack => {
                (self.engine_moves) = Some(
                    self.engine
                        .best_move(&self.board, &self.check, self.en_peasant_susceptible)
                        .unwrap(),
                )
            }
            _ => (),
        };

        self.selected = match self.gamemode {
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
            piece => match (&self.gamemode, piece.color()) {
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
            &self.gamemode,
            &self.board.squares[selection_idx].is_piece(),
            &self.board.squares[selection_idx],
        ) {
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

    fn successful_move(&mut self, selection_idx: usize, destination_idx: usize) -> GameResult {
        self.board.squares[selection_idx].new_idx(destination_idx);
        match self.board.squares[selection_idx] {
            ChessPiece::R(ref mut r) => r.was_moved = true,
            ChessPiece::K(ref mut k) => k.was_moved = true,
            ChessPiece::P(ref mut p) => {
                p.was_moved = true;
                if let Some(en_peasant_target) = self.en_peasant_susceptible {
                    if en_peasant_target == destination_idx {
                        let enemy_pawn_idx: usize = match p.key.0 {
                            PieceColor::Black => destination_idx - 8,
                            PieceColor::White => destination_idx + 8,
                        };
                        self.take_piece(enemy_pawn_idx)?;
                        let _ = std::mem::replace(
                            &mut self.board.squares[enemy_pawn_idx],
                            ChessPiece::Square(Void),
                        );
                    }
                }
            }
            _ => (),
        };
        let moving_piece: ChessPiece = std::mem::replace(
            &mut self.board.squares[selection_idx],
            ChessPiece::Square(Void),
        );

        let _ = std::mem::replace(&mut self.board.squares[destination_idx], moving_piece);
        if let ChessPiece::P(p) = &self.board.squares[destination_idx] {
            self.board.promote(p.index);
        }
        self.reset_mainstate(destination_idx, true)?;
        self.check = self
            .board
            .is_check(self.board.squares[destination_idx].color().unwrap());
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
            match self.gamemode {
                GameMode::MovementBlack => {
                    self.board
                        .black_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.black_vision()?;
                    self.gamemode = GameMode::SelectionWhite;
                }
                GameMode::MovementWhite => {
                    self.board
                        .white_locations
                        .insert(self.board.squares[destination_idx].id()?, destination_idx);
                    self.board.white_vision()?;
                    self.gamemode = GameMode::SelectionBlack;
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

    fn take_piece(&mut self, destination_idx: usize) -> GameResult {
        match self.board.squares[destination_idx].color().unwrap() {
            PieceColor::Black => self
                .board
                .black_locations
                .remove(&self.board.squares[destination_idx].id()?),
            PieceColor::White => self
                .board
                .white_locations
                .remove(&self.board.squares[destination_idx].id()?),
        };
        return Ok(());
    }

    fn handle_movement(&mut self) -> GameResult {
        match self.gamemode {
            GameMode::MovementBlack => self.board.black_vision.clear(),
            GameMode::MovementWhite => self.board.white_vision.clear(),
            _ => unreachable!(),
        }
        self.destination = match self.gamemode {
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
        if self.board.squares[destination_idx].is_piece() {
            self.take_piece(destination_idx)?;
        }

        if &self.board.squares[selection_idx].color()
            != &self.board.squares[destination_idx].color()
        {
            self.successful_move(selection_idx, destination_idx)?;
            self.reset_en_peasant_target(selection_idx, destination_idx);
            if let ChessPiece::K(k) = &self.board.squares[destination_idx]
                && max(selection_idx, destination_idx) - min(selection_idx, destination_idx) == 2
            {
                match k.key.0 {
                    PieceColor::Black => {
                        if selection_idx > destination_idx {
                            self.successful_move(0, destination_idx + 1)?;
                            self.board.black_locations.insert(
                                self.board.squares[destination_idx + 1].id()?,
                                destination_idx + 1,
                            );
                        } else {
                            self.successful_move(7, destination_idx - 1)?;
                            self.board.black_locations.insert(
                                self.board.squares[destination_idx - 1].id()?,
                                destination_idx - 1,
                            );
                        }
                    }
                    PieceColor::White => {
                        if selection_idx > destination_idx {
                            self.successful_move(56, destination_idx + 1)?;
                            self.board.white_locations.insert(
                                self.board.squares[destination_idx + 1].id()?,
                                destination_idx + 1,
                            );
                        } else {
                            self.successful_move(63, destination_idx - 1)?;
                            self.board.white_locations.insert(
                                self.board.squares[destination_idx - 1].id()?,
                                destination_idx - 1,
                            );
                        }
                    }
                }
            }
        } else {
            self.reset_mainstate(destination_idx, false)?;
        }
        (self.selected, self.destination, self.legal_moves) = (None, None, Vec::new());
        if !self.any_legal_moves(match self.gamemode {
            GameMode::SelectionWhite => PieceColor::White,
            GameMode::SelectionBlack => PieceColor::Black,
            _ => return Ok(()),
        }) {
            match self.check.0 {
                KingChecked::None => {
                    self.gamemode = GameMode::Draw;
                    println!("gamemode: {:?}", self.gamemode);
                }
                KingChecked::Black => {
                    self.gamemode = GameMode::BlackWin;
                    println!("gamemode: {:?}", self.gamemode);
                }
                KingChecked::White => {
                    self.gamemode = GameMode::WhiteWin;
                    println!("gamemode: {:?}", self.gamemode);
                }
            };
        };
        /*println!(
            "white locations = {:?},\nblack locations = {:?}",
            self.board.white_locations, self.board.black_locations
        );*/
        println!("engine evaluation: {}", self.engine.evaluate(&self.board));
        return Ok(());
    }

    fn handle_input(&mut self, ctx: &mut Context) -> GameResult {
        self.mouse_pressed = mouse::button_pressed(ctx, mouse::MouseButton::Left);
        match self.gamemode {
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

            match self.gamemode {
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

        draw(ctx, &self.board.board_mesh, DrawParam::default())?;
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
