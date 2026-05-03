use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{heuristics::*, piece_values::*, zobrist_hashes::ZOBRIST_HASH_TABLE},
    gamestate::GameState,
    moves::MoveList,
    transposition::{TTEntry, TranspositionTable},
};
use std::{
    cmp::{max, min},
    //time::{Duration, Instant},
};
pub struct Engine {
    pub side: u32, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
    pub killer_moves: [[Option<u32>; 2]; 32],
    pub move_lists: [MoveList; 32],
    pub move_scores: [[i16; 192]; 32],
    pub quiescence_limitation: u8,
    pub current_hash: u64,
    pub transposition_table: TranspositionTable,
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
            if piece == 0 {
                continue;
            }
            self.evaluation += match piece {
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
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        state: &mut GameState,
        node_count: &mut u64,
    ) -> i32 {
        *node_count += 1;
        let (original_alpha, original_beta): (i32, i32) = (alpha, beta);
        let tt_entry: Option<TTEntry> = self
            .transposition_table
            .get_entry(&self.current_hash, depth);
        let best_move_transposition: u32 = if let Some(entry) = tt_entry {
            if entry.depth >= depth {
                match entry.flag {
                    0 => return entry.score,
                    1 => alpha = alpha.max(entry.score),
                    2 => beta = beta.min(entry.score),
                    _ => (),
                }
                if alpha >= beta {
                    return entry.score;
                }
            }
            entry.best_move
        } else {
            0
        };
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
        let (mut best_score, mut best_move) = (
            if maximizing {
                -CHECKMATE_VALUE
            } else {
                CHECKMATE_VALUE
            },
            0,
        );
        if maximizing {
            // white's branch
            state.whose_turn = 8;
            let mut current_alpha: i32 = alpha;

            self.generate_pseudo_legal_moves(8, &board, &state, depth_as_index, false);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied, &best_move_transposition);
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, score)| score)
                    .unwrap();
                let true_index: usize = best_move_index + i;
                let allegedly_best_move: u32 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];
                self.move_lists[depth_as_index]
                    .pseudo_moves
                    .swap(true_index, i);
                self.move_scores[depth_as_index].swap(true_index, i);

                board.perform_move(
                    allegedly_best_move,
                    state,
                    8,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                if board.is_square_attacked(board.white_king_square, 16) {
                    board.cancel_move(state, 8, &mut self.evaluation, &mut self.current_hash);
                    legal_moves_amount -= 1;
                    continue;
                }

                let current_score: i32 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    current_alpha,
                    beta,
                    false,
                    state,
                    node_count,
                );
                if current_score > best_score {
                    best_score = current_score;
                    best_move = allegedly_best_move;
                }
                board.cancel_move(state, 8, &mut self.evaluation, &mut self.current_hash);

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
        } else {
            // black's branch
            state.whose_turn = 16;
            let mut current_beta: i32 = beta;

            self.generate_pseudo_legal_moves(16, &board, &state, depth_as_index, false);

            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.score_all_moves(depth_as_index, last_occupied, &best_move_transposition);
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, score)| score)
                    .unwrap();
                let true_index: usize = best_move_index + i;
                let allegedly_best_move: u32 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];
                self.move_lists[depth_as_index]
                    .pseudo_moves
                    .swap(true_index, i);
                self.move_scores[depth_as_index].swap(true_index, i);

                board.perform_move(
                    allegedly_best_move,
                    state,
                    16,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                if board.is_square_attacked(board.black_king_square, 8) {
                    board.cancel_move(state, 16, &mut self.evaluation, &mut self.current_hash);
                    legal_moves_amount -= 1;
                    continue;
                }

                let current_score: i32 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    alpha,
                    current_beta,
                    true,
                    state,
                    node_count,
                );
                if current_score < best_score {
                    best_score = current_score;
                    best_move = allegedly_best_move;
                }

                board.cancel_move(state, 16, &mut self.evaluation, &mut self.current_hash);
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
        }

        if depth >= 1 {
            let flag: u8 = if best_score >= original_beta {
                1
            } else if best_score <= original_alpha {
                2
            } else {
                0
            };
            self.transposition_table.record_entry(
                &self.current_hash,
                TTEntry {
                    hash: self.current_hash,
                    score: best_score,
                    depth,
                    flag,
                    best_move,
                },
            );
        }
        return best_score;
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

            board.perform_move(
                move_to_search,
                state,
                color,
                &mut self.evaluation,
                &mut self.current_hash,
            );
            let king_sq: u8 = match color {
                8 => board.white_king_square,
                16 => board.black_king_square,
                _ => unreachable!(),
            };

            if board.is_square_attacked(king_sq, opponent) {
                board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);
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
            board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);

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
        self.current_hash = 0;

        self.transposition_table.hits = 0;
        self.transposition_table.collisions = 0;
        self.transposition_table.replacements = 0;

        // calculate the hash of the position in the beginning
        for (i, piece) in board.cached_pieces.iter().enumerate() {
            let piece: u32 = *piece;
            if piece != 0 {
                let zobrist_index: usize = if piece <= 14 {
                    piece as usize - 9
                } else {
                    piece as usize - 11
                } * 64
                    + i;
                self.current_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
            }
        }

        let mut best_move: Option<u32> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        let opponent_color: u32 = if self.side == 8 { 16 } else { 8 };

        copied_state.whose_turn = self.side.clone() as u32;

        let mut previous_best_move: u32 = 0;
        for d in 2..=self.depth {
            let maximizing: bool = match self.side {
                8 => false,
                16 => true,
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
            let mut depth_best_score: i32 = if self.side == 8 {
                -CHECKMATE_VALUE
            } else {
                CHECKMATE_VALUE
            };
            let mut depth_best_move: u32 = 0;
            if previous_best_move != 0 {
                if let Some(pos) = self.move_lists[depth_as_index].pseudo_moves[..last_occupied]
                    .iter()
                    .position(|&m| m == previous_best_move)
                {
                    self.move_lists[depth_as_index].pseudo_moves.swap(0, pos);
                    self.move_scores[depth_as_index].swap(0, pos);
                }
            }

            for i in 0..last_occupied {
                if self.move_lists[depth_as_index].pseudo_moves[i] != previous_best_move {
                    let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                        .iter()
                        .enumerate()
                        .max_by_key(|&(_, score)| score)
                        .unwrap();
                    let true_index: usize = best_move_index + i;

                    self.move_lists[depth_as_index]
                        .pseudo_moves
                        .swap(true_index, i);
                    self.move_scores[depth_as_index].swap(true_index, i);
                }
                let allegedly_best_move: u32 = self.move_lists[depth_as_index].pseudo_moves[i];

                copied_board.perform_move(
                    allegedly_best_move,
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
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
                        &mut self.current_hash,
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

                copied_board.cancel_move(
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );

                if match self.side {
                    8 => score > depth_best_score,
                    16 => score < depth_best_score,
                    _ => unreachable!(),
                } || depth_best_move == 0
                {
                    depth_best_score = score;
                    depth_best_move = allegedly_best_move;
                }
                println!(
                    "occupied: {}, collisions: {}, replacements: {}, hits: {}, hit rate: {}",
                    self.transposition_table.occupied,
                    self.transposition_table.collisions,
                    self.transposition_table.replacements,
                    self.transposition_table.hits,
                    self.transposition_table.hits as f64 / node_count as f64,
                );
            }
            previous_best_move = depth_best_move;
            println!("reached depth {d}");
        }
        if previous_best_move != 0 {
            best_move = Some(previous_best_move);
            println!(
                "best move: {} {}",
                previous_best_move & FROM_MASK,
                (previous_best_move & TO_MASK) >> TO_SHIFT
            );
            println!("best move found");
        }

        println!("nodes: {node_count}");
        return best_move;
    }
}
