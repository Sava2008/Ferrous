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
            return self.evaluation;
        }
        let king_idx: usize = match maximizing {
            true => *board.white_locations.get(&15).unwrap(),
            false => *board.black_locations.get(&14).unwrap(),
        };
        if maximizing {
            let mut best_score: i16 = self.worst_possible_score;
            let mut current_aphla: i16 = alpha;
            'outer: for (_, piece_idx) in &board.white_locations {
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
                for m in legal_moves {
                    if let ChessPiece::K(_) = board.squares[m] {
                        break;
                    }
                    let mut copied_board: Board = board.clone();
                    let _ = copied_board
                        .perform_move(*piece_idx, m, PieceColor::White)
                        .unwrap();
                    copied_board.is_check(PieceColor::White);

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
                        break 'outer;
                    }
                }
            }
            return best_score;
        } else {
            let mut best_score: i16 = self.best_possible_score;
            let mut current_beta: i16 = beta;

            'outer: for (_, piece_idx) in &board.black_locations {
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
                for m in legal_moves {
                    if let ChessPiece::K(_) = board.squares[m] {
                        break;
                    }
                    let mut copied_board: Board = board.clone();
                    let _ = copied_board
                        .perform_move(*piece_idx, m, PieceColor::Black)
                        .unwrap();
                    copied_board.is_check(PieceColor::Black);

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
                        break 'outer;
                    }
                }
            }
            return best_score;
        }
    }
}
