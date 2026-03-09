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
    pub killer_moves: [[Option<u32>; 2]; 16],
    pub move_lists: [MoveList; 16],
    pub move_scores: [[i16; 192]; 16],
}

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
    ) -> () {
        self.move_lists[depth].first_not_occupied = 0;
        board.pawn_moves(&state, color, &mut self.move_lists[depth]);
        board.knight_moves(color, &mut self.move_lists[depth]);
        board.bishop_moves(color, &mut self.move_lists[depth]);
        board.queen_moves(color, &mut self.move_lists[depth]);
        board.rook_moves(color, &mut self.move_lists[depth]);
        board.king_moves(&state, color, &mut self.move_lists[depth]);
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
        nodes: &mut u64,
    ) -> i32 {
        *nodes += 1;
        if depth == 0 {
            return self.evaluation;
        }
        let depth_as_index: usize = depth as usize;
        if maximizing {
            // white's branch

            let mut best_score: i32 = i32::MIN;
            let mut current_alpha: i32 = alpha;

            self.generate_pseudo_legal_moves(8, &board, &state, depth_as_index);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied);
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
                        nodes,
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
                    i32::MIN + (self.depth - depth) as i32
                } else {
                    0
                };
            }
            return best_score;
        } else {
            // black's branch
            let mut best_score: i32 = i32::MAX;
            let mut current_beta: i32 = beta;

            self.generate_pseudo_legal_moves(16, &board, &state, depth_as_index);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied);
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
                        nodes,
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
                    i32::MAX - (self.depth - depth) as i32
                } else {
                    0
                };
            }
            return best_score;
        }
    }

    pub fn find_best_move(&mut self, board: &Board, state: &mut GameState) -> Option<u32> {
        self.evaluate(board);
        let mut nodes: u64 = 0;
        self.killer_moves = [[None; 2]; 16];
        self.move_lists = [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 16];
        self.move_scores = [[0; 192]; 16];
        let (mut best_score, maximizing): (i32, bool) = match self.side {
            8 => (i32::MIN, false),
            16 => (i32::MAX, true),
            _ => unreachable!(),
        };
        let mut best_move: Option<u32> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        let depth_as_index: usize = self.depth as usize;
        let opponent_color = if self.side == 8 { 16 } else { 8 };

        copied_state.whose_turn = self.side.clone() as u32;
        self.generate_pseudo_legal_moves(self.side, board, &copied_state, depth_as_index);
        let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
        self.score_all_moves(depth_as_index, last_occupied);

        for i in 0..last_occupied {
            let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                .iter()
                .enumerate()
                .min_by_key(|&(_, score)| score)
                .unwrap();
            let true_index: usize = best_move_index + i;
            let allegedly_best_move: u32 = self.move_lists[depth_as_index].pseudo_moves[true_index];
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
                copied_board.cancel_move(&mut copied_state, self.side, &mut self.evaluation);
                continue;
            }

            let score: i32 = self.alpha_beta_pruning(
                &mut copied_board,
                self.depth - 1,
                i32::MIN,
                i32::MAX,
                maximizing,
                &mut copied_state,
                &mut nodes,
            );
            println!(
                "from: {}, to: {}, capture: {}, promotion {}, castling {} ||| score {score}",
                allegedly_best_move & FROM_MASK,
                (allegedly_best_move & TO_MASK) >> TO_SHIFT,
                captured_piece(allegedly_best_move),
                promotion(allegedly_best_move),
                castling(allegedly_best_move),
            );
            copied_board.cancel_move(&mut copied_state, self.side, &mut self.evaluation);
            /*if score
                == match self.side {
                    8 => i32::MAX,
                    _ => i32::MIN,
                }
            {
                return Some(allegedly_best_move);
            }*/

            if match self.side {
                8 => score > best_score || (best_score == i32::MIN && score == best_score),
                16 => score < best_score || (best_score == i32::MAX && score == best_score),
                _ => unreachable!(),
            } {
                best_score = score;
                best_move = Some(self.move_lists[depth_as_index].pseudo_moves[i]);
            }
        }
        println!("nodes: {nodes}");
        return best_move;
    }
}
