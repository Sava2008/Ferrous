use crate::{
    engine::Engine,
    game_logic::{
        Board,
        pieces::{ChessPiece, Piece},
        state_enums::{KingChecked, PieceColor},
    },
    helper_functions::generate_legal_moves,
};
use std::cmp::{max, min};

impl Engine {
    pub fn alpha_beta_pruning(
        &mut self,
        board: &mut Board,
        depth: u8,
        alpha: i16,
        beta: i16,
        maximizing: bool,
        checked: &(KingChecked, Option<usize>, Option<usize>),
        en_peasant_target: Option<usize>,
    ) -> i16 {
        if depth == 0 {
            self.evaluate(board);
            return self.evaluation;
        }

        let king_idx: usize = match maximizing {
            true => *board.white_locations.get(&15).unwrap(),
            false => *board.black_locations.get(&14).unwrap(),
        };

        if maximizing {
            let mut best_score: i16 = self.worst_possible_score;
            let mut current_alpha: i16 = alpha;

            let mut moves_to_try: Vec<(usize, usize)> = Vec::new();
            for (_, piece_idx) in &board.white_locations {
                let piece: &ChessPiece = &board.squares[*piece_idx];
                let legal_moves: Vec<usize> = if let ChessPiece::K(k) = *piece {
                    k.legal_moves(board, en_peasant_target)
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
                        board,
                        king_idx,
                        checked,
                        en_peasant_target,
                    )
                    .unwrap()
                };
                for m in legal_moves {
                    if !matches!(board.squares[m], ChessPiece::K(_)) {
                        moves_to_try.push((*piece_idx, m));
                    }
                }
            }

            'outer: for (piece_idx, m) in moves_to_try {
                board.perform_move(piece_idx, m, PieceColor::White).unwrap();

                let score: i16 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    current_alpha,
                    beta,
                    false,
                    checked,
                    en_peasant_target,
                );

                board.cancel_move().unwrap();

                best_score = max(score, best_score);
                current_alpha = max(current_alpha, best_score);
                if current_alpha >= beta {
                    break 'outer;
                }
            }
            best_score
        } else {
            let mut best_score: i16 = self.best_possible_score;
            let mut current_beta: i16 = beta;

            let mut moves_to_try: Vec<(usize, usize)> = Vec::new();
            for (_, piece_idx) in &board.black_locations {
                let piece: &ChessPiece = &board.squares[*piece_idx];
                let legal_moves: Vec<usize> = if let ChessPiece::K(k) = *piece {
                    k.legal_moves(board, en_peasant_target)
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
                        board,
                        king_idx,
                        checked,
                        en_peasant_target,
                    )
                    .unwrap()
                };
                for m in legal_moves {
                    if !matches!(board.squares[m], ChessPiece::K(_)) {
                        moves_to_try.push((*piece_idx, m));
                    }
                }
            }

            'outer: for (piece_idx, m) in moves_to_try {
                board.perform_move(piece_idx, m, PieceColor::Black).unwrap();

                let score: i16 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    alpha,
                    current_beta,
                    true,
                    checked,
                    en_peasant_target,
                );

                board.cancel_move().unwrap();

                best_score = min(best_score, score);
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    break 'outer;
                }
            }
            best_score
        }
    }
}
