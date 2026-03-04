use crate::{
    board::Board,
    board_geometry_templates::{Bitboard, NO_PIECE_BLACK, NO_PIECE_WHITE},
    constants::heuristics::*,
    gamestate::GameState,
};
use std::cmp::{max, min};
pub struct Engine {
    pub side: u8, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
    pub killer_moves: [[Option<u16>; 2]; 16],
}

impl Engine {
    pub fn evaluate(&mut self, board: &Board) -> () {
        self.evaluation = 0;
        let mut p: Bitboard = board.white_bishops;
        while p != 0 {
            self.evaluation += WHITE_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_knights;
        while p != 0 {
            self.evaluation += WHITE_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_rooks;
        while p != 0 {
            self.evaluation += WHITE_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_pawns;
        while p != 0 {
            self.evaluation += WHITE_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_queens;
        while p != 0 {
            self.evaluation += WHITE_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.white_king;
        while p != 0 {
            self.evaluation += WHITE_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_bishops;
        while p != 0 {
            self.evaluation -= BLACK_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_knights;
        while p != 0 {
            self.evaluation -= BLACK_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_rooks;
        while p != 0 {
            self.evaluation -= BLACK_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_pawns;
        while p != 0 {
            self.evaluation -= BLACK_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_queens;
        while p != 0 {
            self.evaluation -= BLACK_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.black_king;
        while p != 0 {
            self.evaluation -= BLACK_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        self.evaluation += board.material;
    }
    pub fn generate_pseudo_legal_moves(
        &self,
        color: &u8,
        board: &Board,
        state: &GameState,
    ) -> Vec<u16> {
        let mut pseudo_legal_moves: Vec<u16> = board.pawn_moves(&state, &color);
        pseudo_legal_moves.extend(board.knight_moves(&state, &color));
        pseudo_legal_moves.extend(board.bishop_moves(&state, &color));
        pseudo_legal_moves.extend(board.queen_moves(&state, &color));
        pseudo_legal_moves.extend(board.rook_moves(&state, &color));
        pseudo_legal_moves.extend(board.king_moves(&state, &color));
        return pseudo_legal_moves;
    }

    fn add_killer(&mut self, killer: u16, depth: u8) {
        let depth: usize = depth as usize;

        if self.killer_moves[depth][0] == Some(killer) {
            return;
        }

        self.killer_moves[depth][1] = self.killer_moves[depth][0];
        self.killer_moves[depth][0] = Some(killer);
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
            self.evaluate(board);
            return self.evaluation;
        }
        if maximizing {
            // white's branch
            let mut best_score: i32 = i32::MIN;
            let mut current_alpha: i32 = alpha;
            state.whose_turn = NO_PIECE_WHITE;

            let mut pseudo_legal_moves: Vec<u16> =
                self.generate_pseudo_legal_moves(&state.whose_turn, &board, &state);

            if pseudo_legal_moves.len() == 0 {
                return if board.is_square_attacked(board.white_king.trailing_zeros() as u8, &16) {
                    i32::MIN + (self.depth - depth) as i32
                } else {
                    0
                };
            }

            for i in 0..pseudo_legal_moves.len() {
                let best_move_index: usize =
                    self.find_best_for_alpha_beta(&board, depth as usize, &pseudo_legal_moves, i);
                let (current_move, allegedly_best_move) =
                    (pseudo_legal_moves[i], pseudo_legal_moves[best_move_index]);
                pseudo_legal_moves[i] = allegedly_best_move;
                pseudo_legal_moves[best_move_index] = current_move;

                board.perform_move(&allegedly_best_move, state);
                if board.is_square_attacked(board.white_king.trailing_zeros() as u8, &16) {
                    board.cancel_move(state);
                    continue;
                }

                best_score = max(
                    self.alpha_beta_pruning(board, depth - 1, current_alpha, beta, false, state),
                    best_score,
                );
                board.cancel_move(state);
                current_alpha = max(current_alpha, best_score);
                if current_alpha >= beta {
                    if !board.is_capture(&allegedly_best_move) && depth < self.depth {
                        self.add_killer(allegedly_best_move, depth);
                    }
                    break;
                }
            }
            return best_score;
        } else {
            // black's branch
            let mut best_score: i32 = i32::MAX;
            let mut current_beta: i32 = beta;
            state.whose_turn = NO_PIECE_BLACK;

            let mut pseudo_legal_moves: Vec<u16> =
                self.generate_pseudo_legal_moves(&state.whose_turn, &board, &state);

            if pseudo_legal_moves.len() == 0 {
                return if board.is_square_attacked(board.black_king.trailing_zeros() as u8, &8) {
                    i32::MAX - (self.depth - depth) as i32
                } else {
                    0
                };
            }
            for i in 0..pseudo_legal_moves.len() {
                let best_move_index: usize =
                    self.find_best_for_alpha_beta(&board, depth as usize, &pseudo_legal_moves, i);
                let (current_move, allegedly_best_move) =
                    (pseudo_legal_moves[i], pseudo_legal_moves[best_move_index]);
                pseudo_legal_moves[i] = allegedly_best_move;
                pseudo_legal_moves[best_move_index] = current_move;

                board.perform_move(&allegedly_best_move, state);
                if board.is_square_attacked(board.black_king.trailing_zeros() as u8, &8) {
                    board.cancel_move(state);
                    continue;
                }

                best_score = min(
                    self.alpha_beta_pruning(board, depth - 1, alpha, current_beta, true, state),
                    best_score,
                );
                board.cancel_move(state);
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    if !board.is_capture(&allegedly_best_move) && depth < self.depth {
                        self.add_killer(allegedly_best_move, depth);
                    }
                    break;
                }
            }
            return best_score;
        }
    }

    pub fn find_best_move(&mut self, board: &Board, state: &mut GameState) -> Option<u16> {
        self.killer_moves = [[None; 2]; 16];
        let (mut best_score, maximizing): (i32, bool) = match self.side {
            8 => (i32::MIN, false),
            16 => (i32::MAX, true),
            _ => unreachable!(),
        };
        let mut best_move: Option<u16> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        copied_state.whose_turn = self.side.clone();
        let (king_square, color) = match self.side {
            8 => (board.white_king.trailing_zeros() as u8, 16),
            16 => (board.black_king.trailing_zeros() as u8, 8),
            _ => unreachable!(),
        };

        let mut pseudo_legal_moves: Vec<u16> =
            self.generate_pseudo_legal_moves(&self.side, board, &copied_state);
        for i in 0..pseudo_legal_moves.len() {
            let best_move_index: usize = self.find_best_for_alpha_beta(
                &copied_board,
                self.depth as usize,
                &pseudo_legal_moves,
                i,
            );
            let (current_move, allegedly_best_move) =
                (pseudo_legal_moves[i], pseudo_legal_moves[best_move_index]);
            pseudo_legal_moves[i] = allegedly_best_move;
            pseudo_legal_moves[best_move_index] = current_move;

            copied_board.perform_move(&allegedly_best_move, &mut copied_state);
            if copied_board.is_square_attacked(king_square, &color) {
                copied_board.cancel_move(&mut copied_state);
                continue;
            }

            let score: i32 = self.alpha_beta_pruning(
                &mut copied_board,
                self.depth,
                i32::MIN,
                i32::MAX,
                maximizing,
                &mut copied_state,
            );
            copied_board.cancel_move(&mut copied_state);

            if match self.side {
                8 => score > best_score,
                16 => score < best_score,
                _ => unreachable!(),
            } {
                best_score = score;
                best_move = Some(pseudo_legal_moves[i]);
            }
        }
        return best_move;
    }
}
