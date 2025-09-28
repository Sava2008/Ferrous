use ggez::{
    Context, GameResult,
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
};

use crate::{
    constants::{BOARD_AREA, COORDS, SQUARE_SIDE},
    game_logic::{
        PieceColor,
        pieces::{Bishop, ChessPiece, King, Knight, Pawn, Queen, Rook},
    },
    helper_functions::generate_empty_board,
};

pub struct Board {
    pub squares: [ChessPiece; BOARD_AREA],
    pub board_mesh: Mesh,
}
impl Board {
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
        return Ok(Board {
            squares: generate_empty_board(),
            board_mesh: mesh_builder.build(ctx)?,
        });
    }

    pub fn set(&mut self) -> GameResult {
        for i in 0..=7 {
            let black_pawn_idx: usize = i + 8;
            self.squares[black_pawn_idx] =
                ChessPiece::P(Pawn::new(PieceColor::Black, black_pawn_idx));
            let white_pawn_idx: usize = BOARD_AREA - black_pawn_idx - 1;
            self.squares[white_pawn_idx] =
                ChessPiece::P(Pawn::new(PieceColor::White, white_pawn_idx));

            let white_piece_pos: usize = BOARD_AREA - i - 1;

            match i {
                0 | 7 => {
                    self.squares[i] = ChessPiece::R(Rook::new(PieceColor::Black, i));
                    self.squares[white_piece_pos] =
                        ChessPiece::R(Rook::new(PieceColor::White, white_piece_pos));
                }
                1 | 6 => {
                    self.squares[i] = ChessPiece::N(Knight::new(PieceColor::Black, i));
                    self.squares[white_piece_pos] =
                        ChessPiece::N(Knight::new(PieceColor::White, white_piece_pos));
                }
                2 | 5 => {
                    self.squares[i] = ChessPiece::B(Bishop::new(PieceColor::Black, i));
                    self.squares[white_piece_pos] =
                        ChessPiece::B(Bishop::new(PieceColor::White, white_piece_pos));
                }
                3 => {
                    self.squares[i] = ChessPiece::Q(Queen::new(PieceColor::Black, i));
                    self.squares[white_piece_pos - 1] =
                        ChessPiece::Q(Queen::new(PieceColor::White, white_piece_pos - 1));
                }
                4 => {
                    self.squares[i] = ChessPiece::K(King::new(PieceColor::Black, i));
                    self.squares[white_piece_pos + 1] =
                        ChessPiece::K(King::new(PieceColor::White, white_piece_pos + 1));
                }
                _ => (),
            }
        }

        return Ok(());
    }
}
