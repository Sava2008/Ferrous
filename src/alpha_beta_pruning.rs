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
        board: &Board,
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

            let mut legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&state.whose_turn, &board, &state);
            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MIN
                } else {
                    0
                };
            }
            legal_moves.sort_by_key(|m| if board.is_capture(m) { 0 } else { 1 });
			// println!("legal_moves after sorting: {legal_moves:?},\nboard: {board:?}");

            for m in legal_moves {
                let mut copied_board: Board = board.clone();
                let mut copied_state: GameState = state.clone();
                copied_board.perform_move(&m);
				println!("move: {m:?}, board: {board:?}");

                copied_board.total_occupancy();
				copied_state.whose_turn = match copied_state.whose_turn {
					PieceColor::White => PieceColor::Black,
					PieceColor::Black => PieceColor::White,
				};
                copied_state
                    .check_info
                    .update(&copied_board, &PieceColor::Black);
                copied_state
                    .pin_info
                    .update(&copied_board, &PieceColor::Black);
                copied_state.update_check_constraints(&copied_board);
				if copied_board.is_king_attacked(&PieceColor::White) {
					println!("skippinng illegal move: {m:?}");
					continue;
				}
                best_score = max(
                    self.alpha_beta_pruning(
                        &copied_board,
                        depth - 1,
                        current_alpha,
                        beta,
                        false,
                        &mut copied_state,
                    ),
                    best_score,
                );
				// println!("best score: {best_score}, current_alpha: {current_alpha}");
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

            let mut legal_moves: Vec<PieceMove> =
                Self::generate_legal_moves(&state.whose_turn, &board, &state);

            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MAX
                } else {
                    0
                };
            }
            legal_moves.sort_by_key(|m| if board.is_capture(m) { 0 } else { 1 });

            for m in legal_moves {
                let mut copied_board: Board = board.clone();
                let mut copied_state: GameState = state.clone();
                copied_board.perform_move(&m);
				println!("move: {m:?}, board: {board:?}");

                copied_board.total_occupancy();
				copied_state.whose_turn = match copied_state.whose_turn {
					PieceColor::White => PieceColor::Black,
					PieceColor::Black => PieceColor::White,
				};
                copied_state
                    .check_info
                    .update(&copied_board, &PieceColor::White);
                copied_state
                    .pin_info
                    .update(&copied_board, &PieceColor::White);
                copied_state.update_check_constraints(&copied_board);
				if copied_board.is_king_attacked(&PieceColor::Black) {
					println!("skippinng illegal move: {m:?}");
					continue;
				}

                best_score = min(
                    self.alpha_beta_pruning(
                        &copied_board,
                        depth - 1,
                        alpha,
                        current_beta,
                        true,
                        &mut copied_state,
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

    pub fn find_best_move(&mut self, board: &Board, state: &GameState) -> Option<PieceMove> {
        let mut best_score: i32 = i32::MIN;
        let mut best_move: Option<PieceMove> = None;
        let mut legal_moves: Vec<PieceMove> = Self::generate_legal_moves(&self.side, board, state);
        legal_moves.sort_by_key(|m| if board.is_capture(m) { 0 } else { 1 });
        for m in legal_moves {
            println!("{:?}", m);

            let mut copied_board: Board = board.clone();
            let mut copied_state: GameState = state.clone();

            copied_board.perform_move(&m);
            copied_board.total_occupancy();
			copied_state.whose_turn = match copied_state.whose_turn {
				PieceColor::White => PieceColor::Black,
				PieceColor::Black => PieceColor::White,
			};
            copied_state
                .check_info
                .update(&copied_board, &PieceColor::Black);
            copied_state
                .pin_info
                .update(&copied_board, &PieceColor::Black);
            copied_state.update_check_constraints(&copied_board);
            println!("pruning...");
            let score: i32 = self.alpha_beta_pruning(
                &copied_board,
                self.depth,
                i32::MIN,
                i32::MAX,
                false,
                &mut copied_state,
            );
            println!("passed the move");

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }
        return best_move;
    }
}
