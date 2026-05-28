use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        heuristics::*,
        piece_values::*,
        zobrist_hashes::{BLACK_ZOBRIST_KEY, WHITE_ZOBRIST_KEY, ZOBRIST_HASH_TABLE},
    },
    gamestate::GameState,
    moves::MoveList,
    transposition::{TTEntry, TranspositionTable},
};
use std::{
    cmp::{max, min},
    time::{Duration, Instant},
};
pub struct Engine {
    pub side: u16, // which color Ferrous plays
    pub depth: u8,
    pub evaluation: i32,
    pub killer_moves: [[Option<u16>; 2]; 32],
    pub move_lists: [MoveList; 32],
    pub move_scores: [[i16; 192]; 32],
    pub history_heuristics: [i16; 4096],
    pub quiescence_limitation: u8,
    pub current_hash: u64,
    pub transposition_table: TranspositionTable,
}
const CHECKMATE_VALUE: i32 = 1_000_000;

impl Engine {
    #[inline(always)]
    pub fn evaluate(&mut self, board: &Board) -> () {
        self.evaluation = 0;
        let mut p: u64 = board.bitboards[2];
        while p != 0 {
            self.evaluation += WHITE_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[1];
        while p != 0 {
            self.evaluation += WHITE_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[3];
        while p != 0 {
            self.evaluation += WHITE_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[0];
        while p != 0 {
            self.evaluation += WHITE_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[4];
        while p != 0 {
            self.evaluation += WHITE_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[5];
        while p != 0 {
            self.evaluation += WHITE_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[8];
        while p != 0 {
            self.evaluation -= BLACK_BISHOP_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[7];
        while p != 0 {
            self.evaluation -= BLACK_KNIGHT_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[9];
        while p != 0 {
            self.evaluation -= BLACK_ROOK_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[6];
        while p != 0 {
            self.evaluation -= BLACK_PAWN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[10];
        while p != 0 {
            self.evaluation -= BLACK_QUEEN_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        p = board.bitboards[11];
        while p != 0 {
            self.evaluation -= BLACK_KING_HEURISTICS[p.trailing_zeros() as usize];
            p &= p - 1;
        }
        for piece in board.cached_pieces {
            if piece == 0 {
                continue;
            }
            self.evaluation += VALUE_TABLE[piece as usize - 1];
        }
    }

    #[inline(always)]
    pub fn generate_pseudo_legal_moves(
        &mut self,
        color: u16,
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
        let best_move_transposition: u16 = if let Some(entry) = tt_entry {
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
            return self.quiescence_search(
                board,
                state,
                alpha,
                beta,
                maximizing,
                self.depth as usize,
                if state.whose_turn == 8 { 16 } else { 8 },
                node_count,
            );
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
            self.score_all_moves(
                depth_as_index,
                last_occupied,
                &best_move_transposition,
                &board.cached_pieces,
            );
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                /*let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
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
                self.move_scores[depth_as_index].swap(true_index, i);*/

                let true_index: usize = if i < 8 {
                    let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                        .iter()
                        .enumerate()
                        .max_by_key(|&(_, score)| score)
                        .unwrap();
                    best_move_index + i
                } else {
                    i
                };

                let allegedly_best_move: u16 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];

                if i < 8 {
                    self.move_lists[depth_as_index]
                        .pseudo_moves
                        .swap(true_index, i);
                    self.move_scores[depth_as_index].swap(true_index, i);
                }

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
                    if !board.is_capture(allegedly_best_move) {
                        self.add_killer(allegedly_best_move, depth);

                        let history_idx: usize = (((allegedly_best_move & FROM_MASK) as usize)
                            << 6)
                            | ((allegedly_best_move & TO_MASK) >> TO_SHIFT) as usize;
                        self.history_heuristics[history_idx] += (depth * depth) as i16;
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
            self.score_all_moves(
                depth_as_index,
                last_occupied,
                &best_move_transposition,
                &board.cached_pieces,
            );
            let mut legal_moves_amount: usize = last_occupied;

            for i in 0..last_occupied {
                let true_index: usize = if i < 8 {
                    let (best_move_index, _) = self.move_scores[depth_as_index][i..last_occupied]
                        .iter()
                        .enumerate()
                        .max_by_key(|&(_, score)| score)
                        .unwrap();
                    best_move_index + i
                } else {
                    i
                };

                let allegedly_best_move: u16 =
                    self.move_lists[depth_as_index].pseudo_moves[true_index];

                if i < 8 {
                    self.move_lists[depth_as_index]
                        .pseudo_moves
                        .swap(true_index, i);
                    self.move_scores[depth_as_index].swap(true_index, i);
                }

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
                    if !board.is_capture(allegedly_best_move) {
                        self.add_killer(allegedly_best_move, depth);

                        let history_idx: usize = (((allegedly_best_move & FROM_MASK) as usize)
                            << 6)
                            | ((allegedly_best_move & TO_MASK) >> TO_SHIFT) as usize;
                        self.history_heuristics[history_idx] += (depth * depth) as i16;
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
        mut beta: i32,
        maximizing: bool,
        mut quiescence_depth: usize,
        color: u16,
        node_count: &mut u64,
    ) -> i32 {
        quiescence_depth += 1;
        if quiescence_depth >= 24 {
            return self.evaluation;
        }

        *node_count += 1;
        let (original_alpha, original_beta): (i32, i32) = (alpha, beta);
        let tt_entry: Option<TTEntry> = self.transposition_table.get_entry(&self.current_hash, 0);
        let best_move_transposition: u16 = if let Some(entry) = tt_entry {
            match entry.flag {
                0 => return entry.score,
                1 => alpha = alpha.max(entry.score),
                2 => beta = beta.min(entry.score),
                _ => (),
            }
            if alpha >= beta {
                return entry.score;
            }
            entry.best_move
        } else {
            0
        };

        let in_check: bool = match color {
            8 => board.is_square_attacked(board.white_king_square, 16),
            _ => board.is_square_attacked(board.black_king_square, 8),
        };
        let stand_pat: i32 = self.evaluation;

        if !in_check {
            if maximizing {
                if stand_pat >= beta {
                    self.transposition_table.record_entry(
                        &self.current_hash,
                        TTEntry {
                            hash: self.current_hash,
                            score: stand_pat,
                            depth: 0,
                            flag: 1,
                            best_move: 0,
                        },
                    );
                    return stand_pat;
                }
                if stand_pat > alpha {
                    alpha = stand_pat;
                }

                if stand_pat + QUEEN_VALUE + 100 < alpha {
                    self.transposition_table.record_entry(
                        &self.current_hash,
                        TTEntry {
                            hash: self.current_hash,
                            score: stand_pat,
                            depth: 0,
                            flag: 2,
                            best_move: 0,
                        },
                    );
                    return alpha;
                }
            } else {
                if stand_pat <= alpha {
                    self.transposition_table.record_entry(
                        &self.current_hash,
                        TTEntry {
                            hash: self.current_hash,
                            score: stand_pat,
                            depth: 0,
                            flag: 1,
                            best_move: 0,
                        },
                    );
                    return stand_pat;
                }
                if stand_pat < beta {
                    beta = stand_pat;
                }

                if stand_pat - QUEEN_VALUE - 100 > beta {
                    self.transposition_table.record_entry(
                        &self.current_hash,
                        TTEntry {
                            hash: self.current_hash,
                            score: stand_pat,
                            depth: 0,
                            flag: 2,
                            best_move: 0,
                        },
                    );
                    return beta;
                }
            }
        }

        self.generate_pseudo_legal_moves(color, board, state, quiescence_depth, !in_check);
        let last_occupied: usize = self.move_lists[quiescence_depth].first_not_occupied;

        if last_occupied < 1 {
            return if in_check {
                let checkmate_score: i32 = CHECKMATE_VALUE - quiescence_depth as i32;
                if color == 8 {
                    -checkmate_score
                } else {
                    checkmate_score
                }
            } else {
                stand_pat
            };
        }

        self.score_all_moves(
            quiescence_depth,
            last_occupied,
            &best_move_transposition,
            &board.cached_pieces,
        );

        let mut best_score: i32 = stand_pat;

        let mut best_move: u16 = 0;

        let opponent: u16 = if color == 8 { 16 } else { 8 };

        let scores: &mut [i16; 192] = &mut self.move_scores[quiescence_depth];
        let moves: &mut [u16; 192] = &mut self.move_lists[quiescence_depth].pseudo_moves;

        for i in 0..last_occupied {
            let (best_move_index_offset, _) = scores[i..last_occupied]
                .iter()
                .enumerate()
                .max_by_key(|(_, score)| **score)
                .unwrap();
            let best_move_index: usize = i + best_move_index_offset;
            (*scores).swap(i, best_move_index);
            (*moves).swap(i, best_move_index);
        }

        for i in 0..last_occupied {
            let move_to_search: u16 = self.move_lists[quiescence_depth].pseudo_moves[i];
            /*if board.cached_pieces[from_square(move_to_search) as usize] == 0 {
                panic!(
                    "no piece at starting square, depth: {quiescence_depth}, square: {}\nking square: {}, cached pieces: {:?}",
                    from_square(move_to_search),
                    if color == 16 {
                        board.black_king_square
                    } else {
                        board.white_king_square
                    },
                    board.cached_pieces
                );
            }*/

            board.perform_move(
                move_to_search,
                state,
                color,
                &mut self.evaluation,
                &mut self.current_hash,
            );
            let king_sq: u8 = match color {
                8 => board.white_king_square,
                _ => board.black_king_square,
            };

            if board.is_square_attacked(king_sq, opponent) {
                board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);
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

            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = move_to_search;
                }
                if score > alpha {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = move_to_search;
                }
                if score < beta {
                    beta = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }
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
                depth: 0,
                flag,
                best_move,
            },
        );

        return best_score;
    }

    pub fn find_best_move(
        &mut self,
        board: &Board,
        state: &mut GameState,
        correspondence: bool,
    ) -> Option<u16> {
        for i in 0..4096 {
            self.history_heuristics[i] /= 100;
        }
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

        let mut best_score_eval: i32 = if self.side == 8 {
            -CHECKMATE_VALUE
        } else {
            CHECKMATE_VALUE
        };

        // calculate the hash of the position in the beginning
        for (i, piece) in board.cached_pieces.iter().enumerate() {
            let piece: u16 = *piece;
            if piece != 0 {
                let zobrist_index: usize = (piece as usize - 1) * 64 + i;
                self.current_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
            }
        }
        self.current_hash ^= if self.side == 8 {
            WHITE_ZOBRIST_KEY
        } else {
            BLACK_ZOBRIST_KEY
        };

        let mut best_move: Option<u16> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        let opponent_color: u16 = if self.side == 8 { 16 } else { 8 };

        copied_state.whose_turn = self.side.clone() as u16;

        let mut previous_best_move: u16 = 0;

        let time_limit: Duration = Duration::from_secs(5);
        let timer_start: Instant = Instant::now();

        self.evaluate(board);

        for d in 1..=self.depth {
            if timer_start.elapsed() >= time_limit && !correspondence {
                break;
            }
            let maximizing: bool = match self.side {
                8 => false,
                _ => true,
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

            self.score_all_moves(
                depth_as_index,
                last_occupied,
                &previous_best_move,
                &copied_board.cached_pieces,
            );
            let scores: &mut [i16; 192] = &mut self.move_scores[depth_as_index];
            let moves: &mut [u16; 192] = &mut self.move_lists[depth_as_index].pseudo_moves;

            for i in 0..last_occupied {
                let (best_move_index_offset, _) = scores[i..last_occupied]
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, score)| **score)
                    .unwrap();
                let best_move_index: usize = i + best_move_index_offset;
                (*scores).swap(i, best_move_index);
                (*moves).swap(i, best_move_index);
            }
            let mut depth_best_score: i32 = if self.side == 8 {
                -CHECKMATE_VALUE
            } else {
                CHECKMATE_VALUE
            };
            let mut depth_best_move: u16 = 0;

            for i in 0..last_occupied {
                let allegedly_best_move: u16 = self.move_lists[depth_as_index].pseudo_moves[i];

                copied_board.perform_move(
                    allegedly_best_move,
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );

                let current_king_square: u8 = match self.side {
                    8 => copied_board.white_king_square,
                    _ => copied_board.black_king_square,
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

                if state.is_repetition(self.current_hash) || state.fifty_moves_rule_counter >= 98 {
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

                copied_board.cancel_move(
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );

                if match self.side {
                    8 => score > depth_best_score,
                    _ => score < depth_best_score,
                } || depth_best_move == 0
                {
                    depth_best_score = score;
                    depth_best_move = allegedly_best_move;
                }
                best_score_eval = depth_best_score;
            }
            previous_best_move = depth_best_move;
            //println!("reached depth {d}");
        }
        if previous_best_move != 0 {
            best_move = Some(previous_best_move);
        }
        //println!("eval: {best_score_eval}");

        //println!("nodes: {node_count}\n");
        return best_move;
    }
}
