use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{heuristics::*, piece_values::*},
    gamestate::GameState,
    moves::MoveList,
};
use std::cmp::{max, min};
pub struct Engine {
    pub side: u32, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
    pub killer_moves: [[Option<u32>; 2]; 32],
    pub move_lists: [MoveList; 32],
    pub move_scores: [[i16; 192]; 32],
    pub quiescence_limitation: u8,
}
const CHECKMATE_VALUE: i32 = 1_000_000;

impl Engine {
    #[inline(always)]
    pub fn evaluate(&mut self, board: &Board) -> () {
        self.evaluation = 0;
        let mut p: u64 = board.white_bishops;
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
        for piece in board.cached_pieces {
            if piece.is_none() {
                continue;
            }
            self.evaluation += match piece.unwrap() {
                WHITE_PAWN_U32 => PAWN_VALUE,
                WHITE_BISHOP_U32 => BISHOP_VALUE,
                WHITE_KNIGHT_U32 => KNIGHT_VALUE,
                WHITE_QUEEN_U32 => QUEEN_VALUE,
                WHITE_ROOK_U32 => ROOK_VALUE,
                BLACK_PAWN_U32 => -PAWN_VALUE,
                BLACK_BISHOP_U32 => -BISHOP_VALUE,
                BLACK_KNIGHT_U32 => -KNIGHT_VALUE,
                BLACK_QUEEN_U32 => -QUEEN_VALUE,
                BLACK_ROOK_U32 => -ROOK_VALUE,
                _ => continue,
            }
        }
    }

    #[inline(always)]
    pub fn generate_pseudo_legal_moves(
        &mut self,
        color: u32,
        board: &Board,
        state: &GameState,
        depth: usize,
        captures_only: bool,
    ) -> () {
        self.move_lists[depth].first_not_occupied = 0;
        board.pawn_moves(&state, color, &mut self.move_lists[depth], captures_only);
        board.knight_moves(color, &mut self.move_lists[depth], captures_only);
        board.bishop_moves(color, &mut self.move_lists[depth], captures_only);
        board.queen_moves(color, &mut self.move_lists[depth], captures_only);
        board.rook_moves(color, &mut self.move_lists[depth], captures_only);
        board.king_moves(&state, color, &mut self.move_lists[depth], captures_only);
    }

    fn add_killer(&mut self, killer: u32, depth: u8) {
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
        node_count: &mut u64,
    ) -> i32 {
        *node_count += 1;
        if depth == 0 {
            /*return self.quiescence_search(
                board,
                state,
                alpha,
                beta,
                maximizing,
                self.depth as usize,
                if state.whose_turn == 8 { 16 } else { 8 },
                node_count,
            );*/
            return self.evaluation;
        }
        let depth_as_index: usize = depth as usize;
        if maximizing {
            // white's branch
            state.whose_turn = 8;

            let mut best_score: i32 = -CHECKMATE_VALUE;
            let mut current_alpha: i32 = alpha;

            self.generate_pseudo_legal_moves(8, &board, &state, depth_as_index, false);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied, &0);
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, score)| score)
                    .unwrap();
                let true_index: usize = best_move_index + i;
                let allegedly_best_move: u32 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];
                self.move_lists[depth_as_index]
                    .pseudo_moves
                    .swap(true_index, i);
                self.move_scores[depth_as_index].swap(true_index, i);

                board.perform_move(allegedly_best_move, state, 8, &mut self.evaluation);
                if board.is_square_attacked(board.white_king_square, 16) {
                    board.cancel_move(state, 8, &mut self.evaluation);
                    legal_moves_amount -= 1;
                    continue;
                }

                best_score = max(
                    self.alpha_beta_pruning(
                        board,
                        depth - 1,
                        current_alpha,
                        beta,
                        false,
                        state,
                        node_count,
                    ),
                    best_score,
                );
                board.cancel_move(state, 8, &mut self.evaluation);
                current_alpha = max(current_alpha, best_score);
                if current_alpha >= beta {
                    if !board.is_capture(allegedly_best_move) && depth < self.depth {
                        self.add_killer(allegedly_best_move, depth);
                    }
                    break;
                }
            }
            if legal_moves_amount == 0 {
                return if board.is_square_attacked(board.white_king_square, 16) {
                    -CHECKMATE_VALUE + (self.depth - depth) as i32
                } else {
                    0
                };
            }
            return best_score;
        } else {
            // black's branch
            state.whose_turn = 16;

            let mut best_score: i32 = CHECKMATE_VALUE;
            let mut current_beta: i32 = beta;

            self.generate_pseudo_legal_moves(16, &board, &state, depth_as_index, false);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied, &0);
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, score)| score)
                    .unwrap();
                let true_index: usize = best_move_index + i;
                let allegedly_best_move: u32 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];
                self.move_lists[depth_as_index]
                    .pseudo_moves
                    .swap(true_index, i);
                self.move_scores[depth_as_index].swap(true_index, i);

                board.perform_move(allegedly_best_move, state, 16, &mut self.evaluation);
                if board.is_square_attacked(board.black_king_square, 8) {
                    board.cancel_move(state, 16, &mut self.evaluation);
                    legal_moves_amount -= 1;
                    continue;
                }

                best_score = min(
                    self.alpha_beta_pruning(
                        board,
                        depth - 1,
                        alpha,
                        current_beta,
                        true,
                        state,
                        node_count,
                    ),
                    best_score,
                );
                board.cancel_move(state, 16, &mut self.evaluation);
                current_beta = min(current_beta, best_score);
                if current_beta <= alpha {
                    if !board.is_capture(allegedly_best_move) && depth < self.depth {
                        self.add_killer(allegedly_best_move, depth);
                    }
                    break;
                }
            }
            if legal_moves_amount == 0 {
                return if board.is_square_attacked(board.black_king_square, 8) {
                    CHECKMATE_VALUE - (self.depth - depth) as i32
                } else {
                    0
                };
            }
            return best_score;
        }
    }

    pub fn quiescence_search(
        &mut self,
        board: &mut Board,
        state: &mut GameState,
        mut alpha: i32,
        beta: i32,
        maximizing: bool,
        mut quiescence_depth: usize,
        color: u32,
        node_count: &mut u64,
    ) -> i32 {
        quiescence_depth += 1;
        if quiescence_depth >= 32 {
            return self.evaluation;
        }

        *node_count += 1;

        let in_check: bool = match color {
            8 => board.is_square_attacked(board.white_king_square, 16),
            16 => board.is_square_attacked(board.black_king_square, 8),
            _ => unreachable!(),
        };
        if self.evaluation >= beta {
            return beta;
        }

        if self.evaluation > alpha {
            alpha = self.evaluation;
        }

        self.generate_pseudo_legal_moves(color, board, state, quiescence_depth, !in_check);
        let last_occupied: usize = self.move_lists[quiescence_depth].first_not_occupied;

        let mut legal_moves_count: usize = last_occupied;

        self.score_all_moves(quiescence_depth, last_occupied, &0);

        let mut best_score: i32 = if maximizing {
            -CHECKMATE_VALUE
        } else {
            CHECKMATE_VALUE
        };

        let opponent: u32 = if color == 8 { 16 } else { 8 };

        for i in 0..last_occupied {
            let move_to_search: u32 = self.move_lists[quiescence_depth].pseudo_moves[i];

            board.perform_move(move_to_search, state, color, &mut self.evaluation);
            let king_sq: u8 = match color {
                8 => board.white_king_square,
                16 => board.black_king_square,
                _ => unreachable!(),
            };

            if board.is_square_attacked(king_sq, opponent) {
                board.cancel_move(state, color, &mut self.evaluation);
                legal_moves_count -= 1;
                continue;
            }

            let score: i32 = self.quiescence_search(
                board,
                state,
                alpha,
                beta,
                !maximizing,
                quiescence_depth,
                if color == 8 { 16 } else { 8 },
                node_count,
            );
            board.cancel_move(state, color, &mut self.evaluation);

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }

            if maximizing && alpha >= beta {
                break;
            }
            if !maximizing && alpha >= beta {
                break;
            }
        }
        if legal_moves_count < 1 {
            return if in_check {
                let checkmate_score: i32 = CHECKMATE_VALUE - quiescence_depth as i32;
                if color == 8 {
                    -checkmate_score
                } else {
                    checkmate_score
                }
            } else {
                0
            };
        }

        return alpha;
    }

    pub fn find_best_move(&mut self, board: &Board, state: &mut GameState) -> Option<u32> {
        let mut node_count: u64 = 0;
        self.killer_moves = [[None; 2]; 32];
        self.move_lists = [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 32];
        self.move_scores = [[0; 192]; 32];

        let mut best_move: Option<u32> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        let opponent_color: u32 = if self.side == 8 { 16 } else { 8 };

        copied_state.whose_turn = self.side.clone() as u32;

        let mut window: i32 = 30;

        let mut previous_best_move: u32 = 0;
        for d in 2..=self.depth {
            let (mut best_score, maximizing): (i32, bool) = match self.side {
                8 => (-CHECKMATE_VALUE, false),
                16 => (CHECKMATE_VALUE, true),
                _ => unreachable!(),
            };
            let depth_as_index: usize = d as usize;
            self.generate_pseudo_legal_moves(
                self.side,
                board,
                &copied_state,
                depth_as_index,
                false,
            );
            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied, &previous_best_move);
            if previous_best_move != 0 {
                if let Some(pos) = self.move_lists[depth_as_index].pseudo_moves[..last_occupied]
                    .iter()
                    .position(|&m| m == previous_best_move)
                {
                    self.move_lists[depth_as_index].pseudo_moves.swap(0, pos);
                    self.move_scores[depth_as_index].swap(0, pos);
                }
            }
            let mut alpha: i32 = -CHECKMATE_VALUE;
            let mut beta: i32 = CHECKMATE_VALUE;
            if d > 2 {
                (alpha, beta) = (best_score - window, best_score + window);
            }

            let mut depth_best_score: i32 = best_score;
            // let mut depth_best_move: u32 = previous_best_move;
            loop {
                for i in 0..last_occupied {
                    let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, score)| score)
                        .unwrap();
                    let true_index: usize = best_move_index + i;
                    let allegedly_best_move: u32 =
                        self.move_lists[depth_as_index].pseudo_moves[true_index];

                    self.move_lists[depth_as_index]
                        .pseudo_moves
                        .swap(true_index, i);
                    self.move_scores[depth_as_index].swap(true_index, i);
                    copied_board.perform_move(
                        allegedly_best_move,
                        &mut copied_state,
                        self.side,
                        &mut self.evaluation,
                    );

                    let current_king_square: u8 = match self.side {
                        8 => copied_board.white_king_square,
                        16 => copied_board.black_king_square,
                        _ => unreachable!(),
                    };

                    if copied_board.is_square_attacked(current_king_square, opponent_color) {
                        copied_board.cancel_move(
                            &mut copied_state,
                            self.side,
                            &mut self.evaluation,
                        );
                        continue;
                    }

                    let score: i32 = self.alpha_beta_pruning(
                        &mut copied_board,
                        d - 1,
                        -CHECKMATE_VALUE,
                        CHECKMATE_VALUE,
                        maximizing,
                        &mut copied_state,
                        &mut node_count,
                    );
                    println!(
                        "from: {}, to: {} ||| score: {}",
                        allegedly_best_move & FROM_MASK,
                        (allegedly_best_move & TO_MASK) >> TO_SHIFT,
                        score
                    );

                    copied_board.cancel_move(&mut copied_state, self.side, &mut self.evaluation);

                    if match self.side {
                        8 => score > depth_best_score,
                        16 => score < depth_best_score,
                        _ => unreachable!(),
                    } || previous_best_move == 0
                    {
                        depth_best_score = score;
                        previous_best_move = allegedly_best_move;

                        if match self.side {
                            8 => score > best_score,
                            16 => score < best_score,
                            _ => unreachable!(),
                        } {
                            best_score = score;
                        }
                    }
                }
                if depth_best_score > alpha && depth_best_score < beta {
                    best_score = depth_best_score;
                    window = 30;
                    break;
                } else if depth_best_score <= alpha {
                    beta = (alpha + beta) / 2;
                    alpha = depth_best_score - window;
                } else if depth_best_score >= beta {
                    beta = depth_best_score + window;
                }
                window *= 2;
                if window > 300 {
                    alpha = -CHECKMATE_VALUE;
                    beta = CHECKMATE_VALUE;
                    window = 30;
                }
            }
            if previous_best_move != 0 {
                best_move = Some(previous_best_move);
                println!(
                    "best move: {} {}",
                    previous_best_move & FROM_MASK,
                    (previous_best_move & TO_MASK) >> TO_SHIFT
                );
                println!("previous best move found");
            }
            println!("reached depth {d}");
        }
        println!("nodes: {node_count}");
        return best_move;
    }
}
