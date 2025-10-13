use std::i16;

use crate::{
    game_logic::{
        Board,
        pieces::ChessPiece,
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
    pub best_possible_score: i16,
    pub worst_possible_score: i16,
    pub depth: u8,
    pub evaluation: i16,
}

impl Engine {
    pub fn new(side: PieceColor) -> Self {
        return Engine {
            side,
            rng: rand::rng(),
            best_possible_score: i16::MAX - 1,
            worst_possible_score: i16::MIN + 1,
            depth: 4,
            evaluation: 0,
        };
    }

    pub fn find_best_move(
        &mut self,
        board: &Board,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        en_peasant_target: Option<usize>,
    ) -> Option<(usize, usize)> {
        let mut best_score: i16 = self.worst_possible_score;
        let mut best_move: Option<(usize, usize)> = None;
        let king_idx: usize = *board.black_locations.get(&14).unwrap();

        for (_, piece_idx) in &board.black_locations {
            let piece: &ChessPiece = &board.squares[*piece_idx];
            let legal_moves: Vec<usize> = generate_legal_moves(
                match piece {
                    ChessPiece::B(b) => b,
                    ChessPiece::K(k) => k,
                    ChessPiece::N(n) => n,
                    ChessPiece::P(p) => p,
                    ChessPiece::Q(q) => q,
                    ChessPiece::R(r) => r,
                    ChessPiece::Square(_) => unreachable!(),
                },
                board,
                king_idx,
                checked,
                en_peasant_target,
            )
            .unwrap();

            for m in legal_moves {
                let mut copied_board: Board = board.clone();
                let _ = copied_board
                    .perform_move(*piece_idx, m, en_peasant_target, PieceColor::Black)
                    .unwrap();

                let score: i16 = self.alpha_beta_pruning(
                    &copied_board,
                    self.depth,
                    self.worst_possible_score,
                    self.best_possible_score,
                    false,
                    checked,
                    en_peasant_target,
                );

                if score > best_score {
                    best_score = score;
                    best_move = Some((*piece_idx, m));
                }
            }
        }
        return best_move;
    }
}
