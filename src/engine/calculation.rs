use crate::{
    engine::Engine,
    game_logic::{
        Board,
        pieces::{ChessPiece, Piece},
        state_enums::{EvalBoost, KingChecked, PieceColor},
    },
    helper_functions::generate_legal_moves,
};

use std::cmp::{max, min};

impl Engine {
    pub fn minimax(
        &self,
        board: &Board,
        depth: u8,
        eval_boost: EvalBoost,
        king_idx: usize,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        en_peasant_target: Option<usize>,
    ) {
    }
    pub fn alpha_beta_pruning(
        &mut self,
        board: &Board,
        depth: u8,
        alpha: i16,
        beta: i16,
        maximizing: bool,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        en_peasant_target: Option<usize>,
    ) -> i16 {
        if depth == 0 {
            self.evaluate(&board);
            println!("eval: {}", self.evaluation);
            return self.evaluation;
        }
        let king_idx: usize = match maximizing {
            true => *board.white_locations.get(&15).unwrap(),
            false => *board.black_locations.get(&14).unwrap(),
        };
        if maximizing {
            let mut best_score: i16 = self.worst_possible_score;
            let mut current_aphla: i16 = alpha;
            for (_, piece_idx) in &board.white_locations {
                let piece: &ChessPiece = &board.squares[*piece_idx];
                let legal_moves: Vec<usize> = if let ChessPiece::K(k) = *piece {
                    k.legal_moves(&board, en_peasant_target)
                } else {
                    generate_legal_moves(
                        match piece {
                            ChessPiece::B(b) => b,
                            ChessPiece::N(n) => n,
                            ChessPiece::P(p) => p,
                            ChessPiece::Q(q) => q,
                            ChessPiece::R(r) => r,
                            ChessPiece::Square(_) | ChessPiece::K(_) => unreachable!(),
                        },
                        &board,
                        king_idx,
                        checked,
                        en_peasant_target,
                    )
                    .unwrap()
                };
                println!("legal moves: {legal_moves:?}");
                /*legal_moves.sort_by(|a: &usize, b: &usize| {
                    match (board.squares[*a].is_piece(), board.squares[*b].is_piece()) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                });*/
                /*println!(
                    "color: white, depth: {}, from {}, to {:?}",
                    depth, piece_idx, legal_moves
                );*/
                for m in legal_moves {
                    let mut copied_board: Board = board.clone();
                    let _ = copied_board
                        .perform_move(*piece_idx, m, en_peasant_target, PieceColor::White)
                        .unwrap();

                    best_score = max(
                        self.alpha_beta_pruning(
                            &copied_board,
                            depth - 1,
                            current_aphla,
                            beta,
                            false,
                            checked,
                            en_peasant_target,
                        ),
                        best_score,
                    );
                    current_aphla = max(current_aphla, best_score);
                    if current_aphla >= beta {
                        return best_score;
                    }
                }
            }
            return best_score;
        } else {
            let mut best_score: i16 = self.best_possible_score;
            let mut current_beta: i16 = beta;

            for (_, piece_idx) in &board.black_locations {
                let piece: &ChessPiece = &board.squares[*piece_idx];
                let legal_moves: Vec<usize> = if let ChessPiece::K(k) = *piece {
                    k.legal_moves(&board, en_peasant_target)
                } else {
                    generate_legal_moves(
                        match piece {
                            ChessPiece::B(b) => b,
                            ChessPiece::N(n) => n,
                            ChessPiece::P(p) => p,
                            ChessPiece::Q(q) => q,
                            ChessPiece::R(r) => r,
                            ChessPiece::Square(_) | ChessPiece::K(_) => unreachable!(),
                        },
                        &board,
                        king_idx,
                        checked,
                        en_peasant_target,
                    )
                    .unwrap()
                };
                println!("legal moves: {legal_moves:?}");
                /*legal_moves.sort_by(|a: &usize, b: &usize| {
                    match (board.squares[*a].is_piece(), board.squares[*b].is_piece()) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                });*/
                /*println!(
                    "color: black, depth: {}, from {}, to {:?}",
                    depth, piece_idx, legal_moves
                );*/
                for m in legal_moves {
                    let mut copied_board: Board = board.clone();
                    let _ = copied_board
                        .perform_move(*piece_idx, m, en_peasant_target, PieceColor::Black)
                        .unwrap();

                    best_score = min(
                        self.alpha_beta_pruning(
                            &copied_board,
                            depth - 1,
                            alpha,
                            current_beta,
                            true,
                            checked,
                            en_peasant_target,
                        ),
                        best_score,
                    );
                    current_beta = min(current_beta, best_score);
                    if current_beta <= alpha {
                        return best_score;
                    }
                }
            }
            return best_score;
        }
    }
}
