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
    pub killer_moves: [[Option<u16>; 2]; 128],
    pub move_lists: [MoveList; 128],
    pub move_scores: [[i16; 192]; 128],
    pub history_heuristics: [i16; 4096],
    pub current_hash: u64,
    pub transposition_table: TranspositionTable,
    pub nodes_since_last_check: u64,
    pub how_much_searched: (f32, f32), /* First: how many root move searched,
                                       second how many root moves to search.
                                       By applying the formula (how_much_searched.0 / how_much_searched.1)
                                       the engine can determine whether to end the session or not */
}

const CHECKMATE_VALUE: i32 = 1_000_000;
const TIME_CHECK_NODES_OFFSET: u64 = 25; // how often to check for time
const TIMEOUT_RETURN: i32 = 2_000_001;
const QUIESCENCE_DELTA: i32 = 50;

impl Engine {
    pub fn new(side: u16, depth: u8) -> Self {
        return Engine {
            side,
            depth,
            evaluation: 0,
            killer_moves: [[None; 2]; 128],
            move_lists: [MoveList {
                pseudo_moves: [0; 192],
                first_not_occupied: 0,
            }; 128],
            history_heuristics: [0; 4096],
            move_scores: [[0; 192]; 128],
            current_hash: 0,
            transposition_table: TranspositionTable::new(),
            nodes_since_last_check: 0,
            how_much_searched: (0., 0.),
        };
    }
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
        board.knight_moves(color, &mut self.move_lists[depth], state, captures_only);
        board.bishop_moves(color, &mut self.move_lists[depth], state, captures_only);
        board.queen_moves(color, &mut self.move_lists[depth], state, captures_only);
        board.rook_moves(color, &mut self.move_lists[depth], state, captures_only);
        board.king_moves(&state, color, &mut self.move_lists[depth], captures_only);
    }

    fn add_killer(&mut self, killer: u16, depth: u8) {
        let depth: usize = depth as usize;

        if self.killer_moves[depth][1].is_some() {
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
        start_time: &Instant,
        time_limit_ms: &u128,
        max_depth: usize,
    ) -> i32 {
        *node_count += 1;

        let nodes_since_check: &mut u64 = &mut self.nodes_since_last_check;
        *nodes_since_check += 1;
        if *nodes_since_check >= TIME_CHECK_NODES_OFFSET {
            *nodes_since_check = 0;
            if start_time.elapsed().as_millis() >= *time_limit_ms && !self.proceed_search(depth) {
                return TIMEOUT_RETURN;
            }
        }

        let (original_alpha, original_beta): (i32, i32) = (alpha, beta);
        let tt_entry: Option<TTEntry> = self.transposition_table.get_entry(&self.current_hash);
        let best_move_transposition: u16 = if let Some(entry) = tt_entry {
            if entry.depth >= depth as usize {
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
                max_depth + 1,
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
                &board,
            );

            let mut total_moves: usize = last_occupied;

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
                    8,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                if board.cached_pieces[to_square(allegedly_best_move) as usize] != WHITE_KING_U16 {
                    if board.is_square_attacked(board.white_king_square, 16) {
                        board.cancel_move(state, 8, &mut self.evaluation, &mut self.current_hash);
                        total_moves -= 1;
                        continue;
                    }
                }

                let current_score: i32 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    current_alpha,
                    beta,
                    false,
                    state,
                    node_count,
                    start_time,
                    time_limit_ms,
                    max_depth,
                );
                if current_score == TIMEOUT_RETURN {
                    return current_score;
                }

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
            if total_moves < 1 {
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
                &board,
            );

            let mut total_moves: usize = last_occupied;

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

                if board.cached_pieces[to_square(allegedly_best_move) as usize] != BLACK_KING_U16 {
                    if board.is_square_attacked(board.black_king_square, 8) {
                        board.cancel_move(state, 16, &mut self.evaluation, &mut self.current_hash);
                        total_moves -= 1;
                        continue;
                    }
                }

                let current_score: i32 = self.alpha_beta_pruning(
                    board,
                    depth - 1,
                    alpha,
                    current_beta,
                    true,
                    state,
                    node_count,
                    start_time,
                    time_limit_ms,
                    max_depth,
                );
                if current_score == TIMEOUT_RETURN {
                    return current_score;
                }

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
            if total_moves < 1 {
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
                    depth: depth as usize,
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
        depth: usize,
        color: u16,
        node_count: &mut u64,
    ) -> i32 {
        *node_count += 1;
        if depth >= 24 {
            return self.evaluation;
        }

        let enemy_color: u16 = if color == 8 { 16 } else { 8 };
        let in_check: bool = if color == 8 {
            board.is_square_attacked(board.white_king_square, 16)
        } else {
            board.is_square_attacked(board.black_king_square, 8)
        };

        let tt_entry: Option<TTEntry> = self.transposition_table.get_entry(&self.current_hash);
        let best_move_transposition: u16 = if let Some(entry) = tt_entry {
            if entry.depth == 0 {
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
            } else if entry.depth >= 2 {
                entry.best_move
            } else {
                0
            }
        } else {
            0
        };

        let stand_pat: i32 = self.evaluation;

        if !in_check {
            if maximizing {
                if stand_pat + QUIESCENCE_DELTA < alpha {
                    return stand_pat;
                }
                if stand_pat >= beta {
                    return stand_pat;
                }
                if stand_pat > alpha {
                    alpha = stand_pat;
                }
            } else {
                if stand_pat - QUIESCENCE_DELTA > beta {
                    return stand_pat;
                }
                if stand_pat <= alpha {
                    return stand_pat;
                }
                if stand_pat < beta {
                    beta = stand_pat;
                }
            }
        }

        self.generate_pseudo_legal_moves(color, board, state, depth, !in_check);
        let last_occupied: usize = self.move_lists[depth].first_not_occupied;

        self.score_all_moves(depth, last_occupied, &best_move_transposition, &board);

        let scores: &mut [i16; 192] = &mut self.move_scores[depth];
        let moves: &mut [u16; 192] = &mut self.move_lists[depth].pseudo_moves;
        for i in 0..last_occupied {
            let (best_idx, _) = scores[i..last_occupied]
                .iter()
                .enumerate()
                .max_by_key(|(_, score)| **score)
                .unwrap();
            let best_idx = i + best_idx;
            scores.swap(i, best_idx);
            moves.swap(i, best_idx);
        }

        let mut best_score: i32 = stand_pat;
        let mut best_move: u16 = 0;
        let mut moves_tried: i32 = 0;
        let original_alpha: i32 = alpha;
        let original_beta: i32 = beta;

        for i in 0..last_occupied {
            let move_to_search: u16 = self.move_lists[depth].pseudo_moves[i];
            let to_sq: u16 = (move_to_search & TO_MASK) >> TO_SHIFT;
            let captured_piece: u16 = board.cached_pieces[to_sq as usize];

            if !in_check && captured_piece == 0 {
                continue;
            }

            if !in_check {
                let capture_value: i32 = if captured_piece != 0 {
                    VALUE_TABLE[captured_piece as usize - 1]
                } else {
                    0
                };

                if maximizing {
                    if stand_pat + capture_value + QUIESCENCE_DELTA < alpha {
                        continue;
                    }
                } else {
                    if stand_pat - capture_value - QUIESCENCE_DELTA > beta {
                        continue;
                    }
                }
            }

            board.perform_move(
                move_to_search,
                state,
                color,
                &mut self.evaluation,
                &mut self.current_hash,
            );

            let moving_piece: u16 = board.cached_pieces[to_square(move_to_search) as usize];
            if moving_piece != WHITE_KING_U16 && moving_piece != BLACK_KING_U16 {
                let king_in_check: bool = if color == 8 {
                    board.is_square_attacked(board.white_king_square, 16)
                } else {
                    board.is_square_attacked(board.black_king_square, 8)
                };

                if king_in_check {
                    board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);
                    continue;
                }
            }

            moves_tried += 1;

            let score: i32 = self.quiescence_search(
                board,
                state,
                alpha,
                beta,
                !maximizing,
                depth + 1,
                enemy_color,
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

        if moves_tried == 0 {
            if in_check {
                return -(CHECKMATE_VALUE - depth as i32).abs();
            } else {
                return stand_pat;
            }
        }

        if moves_tried > 0 && (best_score != stand_pat || in_check) {
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
        }

        return best_score;
    }

    pub fn find_best_move(
        &mut self,
        board: &Board,
        state: &mut GameState,
        time_contrainsts: Duration,
        max_depth: u8,
    ) -> Option<u16> {
        let mut node_count: u64 = 0;
        let mut best_move: Option<u16> = None;
        let mut copied_board: Board = board.clone();
        let mut copied_state: GameState = state.clone();
        self.prepare_before_search(&mut copied_board, &mut copied_state);

        copied_state.whose_turn = self.side.clone() as u16;

        let mut previous_best_move: u16 = 0;
        let bad_draw_score: i32 = match self.side {
            8 => -50,
            _ => 50,
        };

        let mut time_limit_ms: u128 = time_contrainsts.as_millis();
        if time_limit_ms == 0 {
            time_limit_ms = 10000 * 1000; // 10_000 seconds
        }
        let max_depth_limit: u8 = max_depth + 1;

        let mut last_finished_depth: usize = 0;
        let mut depth_best_moves: [u16; 64] = [0; 64];

        let opponent_color: u16 = if self.side == 8 { 16 } else { 8 };

        let mut best_score_eval: i32 = 0;
        let timer_start: Instant = Instant::now();

        'outer: for d in 1..=self.depth {
            if max_depth_limit == d {
                break;
            }
            let maximizing: bool = match self.side {
                8 => false,
                _ => true,
            };
            let depth_as_index: usize = d as usize;

            self.generate_pseudo_legal_moves(
                self.side,
                &copied_board,
                &copied_state,
                depth_as_index,
                false,
            );
            let last_occupied: usize = self.move_lists[depth_as_index].first_not_occupied;
            self.how_much_searched.1 = last_occupied as f32;

            self.score_all_moves(
                depth_as_index,
                last_occupied,
                &previous_best_move,
                &copied_board,
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

            let mut moves_searched: usize = 0;

            let mut total_moves: usize = last_occupied;

            for i in 0..last_occupied {
                let allegedly_best_move: u16 = self.move_lists[depth_as_index].pseudo_moves[i];

                let hash_before_move: u64 = self.current_hash;

                copied_board.perform_move(
                    allegedly_best_move,
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                assert_eq!(
                    self.current_hash,
                    Self::rebuild_hash(&mut copied_board, opponent_color)
                );

                if copied_board.is_square_attacked(
                    if self.side == 8 {
                        copied_board.white_king_square
                    } else {
                        copied_board.black_king_square
                    },
                    opponent_color,
                ) {
                    copied_board.cancel_move(
                        &mut copied_state,
                        self.side,
                        &mut self.evaluation,
                        &mut self.current_hash,
                    );
                    total_moves -= 1;
                    continue;
                }

                let mut score: i32 = self.alpha_beta_pruning(
                    &mut copied_board,
                    d - 1,
                    -CHECKMATE_VALUE,
                    CHECKMATE_VALUE,
                    maximizing,
                    &mut copied_state,
                    &mut node_count,
                    &timer_start,
                    &time_limit_ms,
                    depth_as_index,
                );
                if score == TIMEOUT_RETURN {
                    break 'outer;
                }
                self.how_much_searched.0 += 1.;
                moves_searched += 1;

                if copied_state.is_repetition(self.current_hash)
                    || copied_state.fifty_moves_rule_counter >= 98
                {
                    score = if match self.side {
                        8 => score <= bad_draw_score,
                        _ => score >= bad_draw_score,
                    } {
                        0
                    } else {
                        bad_draw_score
                    };
                }

                copied_board.cancel_move(
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                assert_eq!(self.current_hash, hash_before_move);
                assert_eq!(
                    Self::rebuild_hash(&mut copied_board, self.side),
                    hash_before_move
                );

                if match self.side {
                    8 => score > depth_best_score,
                    _ => score < depth_best_score,
                } || depth_best_move == 0
                {
                    depth_best_score = score;
                    depth_best_move = allegedly_best_move;
                }
            }
            if moves_searched == total_moves {
                best_score_eval = depth_best_score;
                previous_best_move = depth_best_move;
                depth_best_moves[last_finished_depth] = previous_best_move;
                last_finished_depth += 1;
                println!("reached depth {d}, eval: {depth_best_score}");
                continue;
            }
            break;
        }
        if *depth_best_moves
            .iter()
            .take_while(|&&m| m != 0)
            .last()
            .unwrap()
            != 0
        {
            best_move = Some(previous_best_move);
        }

        println!("HCE eval: {best_score_eval}");
        println!("nodes: {node_count}\n");
        return best_move;
    }

    fn prepare_before_search(&mut self, board: &mut Board, state: &mut GameState) -> () {
        for i in 0..4096 {
            self.history_heuristics[i] /= 100;
        }
        self.killer_moves = [[None; 2]; 128];
        self.move_lists = [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 128];
        self.move_scores = [[0; 192]; 128];
        self.current_hash = 0;
        self.evaluation = 0;

        self.transposition_table.hits = 0;
        self.transposition_table.collisions = 0;
        self.transposition_table.replacements = 0;

        self.how_much_searched.0 = 0.;
        self.how_much_searched.1 = 0.;

        self.current_hash = Self::rebuild_hash(board, self.side);
        self.evaluate(board);
        board.calculate_check_restrictions(state, state.whose_turn.clone());
    }

    pub fn rebuild_hash(board: &Board, side: u16) -> u64 {
        // calculate the hash of the position in the beginning
        let (
            mut white_queens_amount,
            mut black_queens_amount,
            mut white_pieces_left,
            mut black_pieces_left,
        ) = (0, 0, 0, 0);
        let mut board_hash = 0;
        for (i, piece) in board.cached_pieces.iter().enumerate() {
            let piece: u16 = *piece;
            if piece != 0 {
                let zobrist_index: usize = (piece as usize - 1) * 64 + i;
                board_hash ^= ZOBRIST_HASH_TABLE[zobrist_index];
                match piece {
                    WHITE_QUEEN_U16 => white_queens_amount += 1,
                    BLACK_QUEEN_U16 => black_queens_amount += 1,
                    WHITE_KING_U16 | BLACK_KING_U16 => continue, // don't count kings, they self negate each other
                    _ => (),
                };
                if piece < 7 {
                    white_pieces_left += 1;
                } else {
                    black_pieces_left += 1;
                }
            }
        }
        board_hash ^= if side == 8 {
            WHITE_ZOBRIST_KEY
        } else {
            BLACK_ZOBRIST_KEY
        };
        let piece_heuristic_table: *mut [[i32; 64]; 12] = &raw mut HEURISTICS_TABLE;
        unsafe {
            if white_queens_amount == 0 && white_pieces_left < 8 {
                (*piece_heuristic_table)[5] = ENDGAME_WHITE_KING_HEURISTICS;
            } else {
                (*piece_heuristic_table)[5] = WHITE_KING_HEURISTICS;
            }
            if black_queens_amount == 0 && black_pieces_left < 8 {
                (*piece_heuristic_table)[11] = ENDGAME_BLACK_KING_HEURISTICS;
            } else {
                (*piece_heuristic_table)[11] = BLACK_KING_HEURISTICS;
            }
        }
        return board_hash;
    }

    fn proceed_search(&self, depth: u8) -> bool {
        let depth_percent: f32 = match depth {
            1..=4 => 0.10,
            5..=8 => 0.05,
            _ => 0.02,
        };

        if self.percent_finished() >= depth_percent {
            return false;
        }

        return true;
    }

    fn percent_finished(&self) -> f32 {
        return self.how_much_searched.0 / self.how_much_searched.1;
    }
}
