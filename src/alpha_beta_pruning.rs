use crate::{
    board::Board,
    enums::{PieceColor, PieceType},
    gamestate::{GameState, PieceMove},
};
use std::cmp::{max, min};
pub struct Engine {
    pub side: PieceColor, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
}

impl Engine {
    fn generate_legal_moves(
        color: &PieceColor,
        board: &Board,
        state: &GameState,
    ) -> Vec<PieceMove> {
        let mut legal_moves: Vec<PieceMove> = board.pawn_moves(&state, &color).unwrap();
        legal_moves.extend(board.knight_moves(&state, &color).unwrap());
        legal_moves.extend(board.bishop_moves(&state, &color).unwrap());
        legal_moves.extend(board.queen_moves(&state, &color).unwrap());
        legal_moves.extend(board.rook_moves(&state, &color).unwrap());
        legal_moves.extend(board.king_moves(&state, &color).unwrap());
        return legal_moves;
    }

    pub fn alpha_beta_pruning(
        &mut self,
        board: &Board,
        depth: u8,
        alpha: i32,
        beta: i32,
        maximizing: bool,
        state: &GameState,
    ) -> i32 {
        if depth == 0 {
            self.evaluate(&board);
            return self.evaluation;
        }

        if maximizing {
            // white's branch
            let mut best_score: i32 = i32::MIN;
            let mut current_alpha: i32 = alpha;

            let legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&PieceColor::White, board, state);

            for m in legal_moves {
                if board.bitboard_contains(m.to) == Some((PieceColor::Black, PieceType::King)) {
                    // checkmate, because a king's capture is IN legal moves
                    self.evaluation = i32::MAX;
                    break;
                }

                let mut copied_board: Board = board.clone();
                let mut copied_state: GameState = state.clone();
                let _ = copied_board.perform_move(m);
                copied_state
                    .check_info
                    .update(&copied_board, &PieceColor::Black);

                best_score = max(
                    self.alpha_beta_pruning(
                        &copied_board,
                        depth - 1,
                        current_alpha,
                        beta,
                        false,
                        state,
                    ),
                    best_score,
                );
                current_alpha = max(current_alpha, best_score);
                if current_alpha >= beta {
                    break;
                }
            }
            return best_score;
        } else {
            // black's branch
            let mut best_score: i32 = i32::MAX;
            let mut current_beta: i32 = beta;

            let legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&PieceColor::Black, &board, &state);

            for m in legal_moves {
                if board.bitboard_contains(m.to) == Some((PieceColor::White, PieceType::King)) {
                    // checkmate, because a king's capture is IN legal moves
                    self.evaluation = i32::MIN;
                    break;
                }

                let mut copied_board: Board = board.clone();
                let mut copied_state: GameState = state.clone();
                let _ = copied_board.perform_move(m);
                copied_state
                    .check_info
                    .update(&copied_board, &PieceColor::White);

                best_score = min(
                    self.alpha_beta_pruning(
                        &copied_board,
                        depth - 1,
                        alpha,
                        current_beta,
                        false,
                        state,
                    ),
                    best_score,
                );
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    break;
                }
            }
            return best_score;
        }
    }
}
