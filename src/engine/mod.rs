use crate::{
    game_logic::{
        Board,
        pieces::{ChessPiece, Piece},
        state_enums::{KingChecked, PieceColor},
    },
    helper_functions::generate_legal_moves,
};
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

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
        let mut piece_locations: HashMap<u8, usize> = match self.side {
            PieceColor::Black => board.black_locations.clone(),
            PieceColor::White => board.white_locations.clone(),
        };

        let king_idx: usize = match self.side {
            PieceColor::Black => *board.black_locations.get(&14).unwrap(),
            PieceColor::White => *board.white_locations.get(&15).unwrap(),
        };

        let mut legal_moves: Vec<usize>;
        let mut piece_idx: &usize;
        let mut piece_id: &u8;
        loop {
            (piece_id, piece_idx) = piece_locations.iter().choose(&mut self.rng).unwrap();
            legal_moves = generate_legal_moves(
                match &board.squares[*piece_idx] {
                    ChessPiece::B(b) => b as &dyn Piece,
                    ChessPiece::K(k) => k as &dyn Piece,
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
                break;
            }
        }
        return Ok((*piece_idx, *legal_moves.choose(&mut self.rng).unwrap()));
    }
}
