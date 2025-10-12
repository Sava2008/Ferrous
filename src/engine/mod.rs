use crate::{
    game_logic::{
        Board,
        pieces::{ChessPiece, Piece},
        state_enums::{KingChecked, PieceColor},
    },
    helper_functions::generate_legal_moves,
};
use rand::prelude::*;

pub mod calculation;
pub mod evaluation;

pub struct Engine {
    pub side: PieceColor,
    pub rng: ThreadRng,
}

impl Engine {
    pub fn new(side: PieceColor) -> Self {
        return Engine {
            side,
            rng: rand::rng(),
        };
    }

    pub fn best_move(
        &mut self,
        board: &Board,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        en_peasant_target: Option<usize>,
    ) -> Result<(usize, usize), String> {
        let king_idx: usize = match self.side {
            PieceColor::Black => *board.black_locations.get(&14).unwrap(),
            PieceColor::White => *board.white_locations.get(&15).unwrap(),
        };

        let mut legal_moves: Vec<usize>;
        if checked.0 == KingChecked::None {
            println!(
                "king's moves {:?}",
                board.squares[king_idx].legal_moves(&board, en_peasant_target, checked, 8)
            );
        }
        let piece_idx: usize;
        for (_, index) in match self.side {
            PieceColor::Black => board.black_locations.iter(),
            PieceColor::White => board.white_locations.iter(),
        } {
            legal_moves = generate_legal_moves(
                match &board.squares[*index] {
                    ChessPiece::B(b) => b as &dyn Piece,
                    ChessPiece::K(k) => {
                        let moves: Vec<usize> = k.legal_moves(&board, en_peasant_target);
                        if moves.len() > 0 {
                            return Ok((k.index, *moves.choose(&mut self.rng).unwrap()));
                        } else {
                            continue;
                        }
                    }
                    ChessPiece::N(n) => n as &dyn Piece,
                    ChessPiece::Q(q) => q as &dyn Piece,
                    ChessPiece::R(r) => r as &dyn Piece,
                    ChessPiece::P(p) => p as &dyn Piece,
                    ChessPiece::Square(_) => unreachable!(),
                },
                &board,
                king_idx,
                checked,
                en_peasant_target,
            )
            .unwrap();
            if legal_moves.len() > 0 {
                piece_idx = *index;
                return Ok((piece_idx, *legal_moves.choose(&mut self.rng).unwrap()));
            }
        }
        return Err("no moves".to_string());
    }
}
