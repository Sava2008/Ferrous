use crate::{
    board::Board,
    board_geometry_templates::{
        Bitboard, FROM_MASK, PROMOTION_MASK, PROMOTION_SHIFT, TO_MASK, TO_SHIFT,
    },
    constants::heuristics::*,
    enums::PieceColor,
    gamestate::GameState,
};
use std::cmp::{max, min};
pub struct Engine {
    pub side: PieceColor, // which color Ferrous plays
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
    pub fn generate_legal_moves(
        &self,
        color: &PieceColor,
        board: &Board,
        state: &GameState,
        depth: usize,
    ) -> Vec<u16> {
        let mut legal_moves: Vec<u16> = board.pawn_moves(&state, &color);
        legal_moves.extend(board.knight_moves(&state, &color));
        legal_moves.extend(board.bishop_moves(&state, &color));
        legal_moves.extend(board.queen_moves(&state, &color));
        legal_moves.extend(board.rook_moves(&state, &color));
        legal_moves.extend(board.king_moves(&state, &color));
        legal_moves.sort_by_key(|m: &u16| -(self.move_priority(board, m, depth) as i16));
        return legal_moves;
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
        nodes: &mut u64,
    ) -> i32 {
        *nodes += 1;
        if depth == 0 {
            self.evaluate(board);
            return self.evaluation;
        }
        if maximizing {
            // white's branch
            let mut best_score: i32 = i32::MIN;
            let mut current_alpha: i32 = alpha;
            state.whose_turn = PieceColor::White;

            let legal_moves: Vec<u16> =
                self.generate_legal_moves(&state.whose_turn, &board, &state, depth as usize);

            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MIN + (self.depth - depth) as i32
                } else {
                    0
                };
            }

            for m in &legal_moves {
                board.perform_move(&m, state);

                state.check_info.update(&board, &!state.whose_turn.clone());
                state.pin_info.update(&board, &!state.whose_turn.clone());
                state.update_check_constraints(&board);

                best_score = max(
                    self.alpha_beta_pruning(
                        board,
                        depth - 1,
                        current_alpha,
                        beta,
                        false,
                        state,
                        nodes,
                    ),
                    best_score,
                );
                board.cancel_move(state);
                current_alpha = max(current_alpha, best_score);
                if current_alpha >= beta {
                    if !board.is_capture(m) && depth < self.depth {
                        self.add_killer(*m, depth);
                    }
                    break;
                }
            }
            return best_score;
        } else {
            // black's branch
            let mut best_score: i32 = i32::MAX;
            let mut current_beta: i32 = beta;
            state.whose_turn = PieceColor::Black;

            let legal_moves: Vec<u16> =
                self.generate_legal_moves(&state.whose_turn, &board, &state, depth as usize);

            if legal_moves.len() == 0 {
                return if state.check_info.checked_king.is_some() {
                    i32::MAX - (self.depth - depth) as i32
                } else {
                    0
                };
            }
            for m in &legal_moves {
                board.perform_move(&m, state);

                state.check_info.update(&board, &!state.whose_turn.clone());
                state.pin_info.update(&board, &!state.whose_turn.clone());
                state.update_check_constraints(&board);

                best_score = min(
                    self.alpha_beta_pruning(
                        board,
                        depth - 1,
                        alpha,
                        current_beta,
                        true,
                        state,
                        nodes,
                    ),
                    best_score,
                );
                board.cancel_move(state);
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    if !board.is_capture(m) && depth < self.depth {
                        self.add_killer(*m, depth);
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
            PieceColor::White => (i32::MIN, false),
            PieceColor::Black => (i32::MAX, true),
        };
        let mut best_move: Option<u16> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        copied_state.whose_turn = self.side.clone();
        let mut nodes: u64 = 0;

        let legal_moves: Vec<u16> =
            self.generate_legal_moves(&self.side, board, &copied_state, self.depth as usize);
        for m in &legal_moves {
            // println!("move: {m:?}");
            copied_board.perform_move(&m, &mut copied_state);
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
                &mut nodes,
            );
            /*println!(
                "from: {}, to {}, promo: {}, Score: {}",
                m & FROM_MASK,
                (m & TO_MASK) >> TO_SHIFT,
                (m & PROMOTION_MASK) >> PROMOTION_SHIFT,
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
        println!("nodes: {nodes}");
        return best_move;
    }
}
