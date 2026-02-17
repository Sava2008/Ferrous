use crate::{
    board::Board,
    enums::PieceColor,
    gamestate::{GameState, PieceMove},
};
use std::cmp::{max, min};
pub struct Engine {
    pub side: PieceColor, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
}

impl Engine {
    pub fn generate_legal_moves(
        color: &PieceColor,
        board: &Board,
        state: &GameState,
    ) -> Vec<PieceMove> {
        let mut legal_moves: Vec<PieceMove> = board.pawn_moves(&state, &color);
        legal_moves.extend(board.knight_moves(&state, &color));
        legal_moves.extend(board.bishop_moves(&state, &color));
        legal_moves.extend(board.queen_moves(&state, &color));
        legal_moves.extend(board.rook_moves(&state, &color));
        legal_moves.extend(board.king_moves(&state, &color));
        return legal_moves;
    }

    pub fn alpha_beta_pruning(
        &mut self,
        board: &mut Board,
        depth: u8,
        alpha: i32,
        beta: i32,
        maximizing: bool,
        state: &mut GameState,
    ) -> i32 {
        if depth == 0 {
            self.evaluate(&board);
            return self.evaluation;
        }
        if maximizing {
            // white's branch
            let mut best_score: i32 = i32::MIN;
            let mut current_alpha: i32 = alpha;
            state.whose_turn = PieceColor::White;

            let mut legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&state.whose_turn, &board, &state);

            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MIN + (self.depth - depth) as i32
                } else {
                    0
                };
            }
            legal_moves.sort_by_key(|m: &PieceMove| {
                if board.is_check(m, &!state.whose_turn) {
                    0
                } else if board.is_capture(m) {
                    1
                } else if board.does_improve_piece(m) {
                    2
                } else {
                    3
                }
            });

            for m in &legal_moves {
                board.perform_move(&m, state);

                board.total_occupancy();
                state.check_info.update(&board, &!state.whose_turn.clone());
                state.pin_info.update(&board, &!state.whose_turn.clone());
                state.update_check_constraints(&board);

                best_score = max(
                    self.alpha_beta_pruning(board, depth - 1, current_alpha, beta, false, state),
                    best_score,
                );
                board.cancel_move(state);
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
            state.whose_turn = PieceColor::Black;

            let mut legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&state.whose_turn, &board, &state);

            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MAX - (self.depth - depth) as i32
                } else {
                    0
                };
            }
            legal_moves.sort_by_key(|m: &PieceMove| {
                if board.is_check(m, &!state.whose_turn) {
                    0
                } else if board.is_capture(m) {
                    1
                } else if board.does_improve_piece(m) {
                    2
                } else {
                    3
                }
            });

            for m in &legal_moves {
                board.perform_move(&m, state);

                board.total_occupancy();
                state.check_info.update(&board, &!state.whose_turn.clone());
                state.pin_info.update(&board, &!state.whose_turn.clone());
                state.update_check_constraints(&board);

                best_score = min(
                    self.alpha_beta_pruning(board, depth - 1, alpha, current_beta, true, state),
                    best_score,
                );
                board.cancel_move(state);
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    break;
                }
            }
            return best_score;
        }
    }

    pub fn find_best_move(&mut self, board: &Board, state: &mut GameState) -> Option<PieceMove> {
        let (mut best_score, maximizing): (i32, bool) = match self.side {
            PieceColor::White => (i32::MIN, false),
            PieceColor::Black => (i32::MAX, true),
        };
        let mut best_move: Option<PieceMove> = None;
        let mut copied_board: Board = board.clone();
        copied_board.total_occupancy();
        let mut copied_state: GameState = state.clone();
        copied_state.whose_turn = self.side.clone();

        let mut legal_moves: Vec<PieceMove> =
            Self::generate_legal_moves(&self.side, board, &copied_state);

        legal_moves.sort_by_key(|m: &PieceMove| {
            if board.is_check(m, &!self.side) {
                0
            } else if board.is_capture(m) {
                1
            } else if board.does_improve_piece(m) {
                2
            } else {
                3
            }
        });
        for m in &legal_moves {
            copied_board.perform_move(&m, &mut copied_state);
            copied_board.total_occupancy();
            copied_state
                .check_info
                .update(&copied_board, &!copied_state.whose_turn.clone());
            copied_state
                .pin_info
                .update(&copied_board, &!copied_state.whose_turn.clone());
            copied_state.update_check_constraints(&copied_board);

            let score: i32 = self.alpha_beta_pruning(
                &mut copied_board,
                self.depth,
                i32::MIN,
                i32::MAX,
                maximizing,
                &mut copied_state,
            );
            println!("Move: {:?}, Score: {}", m, score);
            /*println!(
                "move: {} {}, score: {}",
                INDICES_TO_COORDS.get(&m.from).unwrap(),
                INDICES_TO_COORDS.get(&m.to).unwrap(),
                score
            );*/
            copied_board.cancel_move(&mut copied_state);

            if match self.side {
                PieceColor::White => score > best_score,
                PieceColor::Black => score < best_score,
            } {
                best_score = score;
                best_move = Some(m.clone());
            }
        }
        return best_move;
    }
}
