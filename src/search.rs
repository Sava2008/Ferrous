use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        attacks::MVV_LVA,
        heuristics::*,
        piece_values::*,
        zobrist_hashes::{BLACK_ZOBRIST_KEY, WHITE_ZOBRIST_KEY, ZOBRIST_HASH_TABLE},
    },
    gamestate::GameState,
    moves::MoveList,
    transposition::{TTEntry, TranspositionTable},
};
use std::time::{Duration, Instant};
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
const TIME_CHECK_NODES_OFFSET: u64 = 2500; // how often to check for time
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

    pub fn negamax(
        &mut self,
        board: &mut Board,
        depth: u8,
        ply: usize,
        color: u16,
        mut alpha: i32,
        mut beta: i32,
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

        let tt_entry: Option<TTEntry> = self.transposition_table.get_entry(&self.current_hash);
        let best_move_transposition: u16 = if let Some(entry) = tt_entry {
            let tt_score: i32 = entry.score;
            if entry.depth >= depth as usize {
                match entry.flag {
                    0 => return tt_score,
                    2 => alpha = alpha.max(tt_score),
                    1 => beta = beta.min(tt_score),
                    _ => (),
                }
                if alpha >= beta {
                    return tt_score;
                }
            }
            entry.best_move
        } else {
            0
        };

        let enemy_color: u16 = if color == 8 { 16 } else { 8 };

        if depth == 0 {
			// return if color == 8 { self.evaluation } else { -self.evaluation };
            return self.quiescence_search(
                board,
                state,
                alpha,
                beta,
                max_depth + 1,
                ply + 1,
                color,
                node_count,
            );
        }
        let mut best_score: i32 = -CHECKMATE_VALUE;
        let mut best_move: u16 = 0;
        let (original_alpha, original_beta) = (alpha, beta);

        let mut current_alpha: i32 = alpha;

        self.generate_pseudo_legal_moves(color, &board, &state, ply, false);

        let last_occupied: usize = self.move_lists[ply].first_not_occupied;
        self.score_all_moves(ply, last_occupied, &best_move_transposition, &board);
        Self::lazy_sort_moves(
            &mut self.move_lists[ply].pseudo_moves,
            &mut self.move_scores[ply],
            last_occupied,
        );
        let mut total_moves: usize = 0;

        for i in 0..last_occupied {
            let allegedly_best_move: u16 = self.move_lists[ply].pseudo_moves[i];

            board.perform_move(
                allegedly_best_move,
                state,
                color,
                &mut self.evaluation,
                &mut self.current_hash,
            );
            let moving_piece: u16 = board.cached_pieces[to_square(allegedly_best_move) as usize];
            let (king_piece, king_square) = if color == 8 {
                (WHITE_KING_U16, board.white_king_square)
            } else {
                (BLACK_KING_U16, board.black_king_square)
            };
            if moving_piece != king_piece {
                if board.is_square_attacked(king_square, enemy_color) {
                    board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);
                    continue;
                }
            }
            total_moves += 1;
            // let reduction: u8 = if current_is_quiet {
            //     let lmr: u8 = match total_moves {
            //         0..8 => 0,
            //         8..12 => 1,
            //         _ => 2,
            //     };
            //     if depth > 2 && i > 7 { lmr } else { 0 }
            // } else {
            //     0
            // };

            let current_score: i32 = -self.negamax(
                board,
                depth - 1,
                ply + 1,
                enemy_color,
                -beta,
                -current_alpha,
                state,
                node_count,
                start_time,
                time_limit_ms,
                max_depth,
            );
            if current_score == TIMEOUT_RETURN || current_score == -TIMEOUT_RETURN {
                return current_score;
            }

            // if current_score > current_alpha && current_score < beta && i > 0 {
            //     current_score = -self.negamax(
            //         board,
            //         depth - 1,
            //         ply + 1,
            //         enemy_color,
            //         -beta,
            //         -current_alpha,
            //         state,
            //         node_count,
            //         start_time,
            //         time_limit_ms,
            //         max_depth,
            //     );
            // }

            if current_score > best_score {
                best_score = current_score;
                best_move = allegedly_best_move;
            }
            board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);

            current_alpha = current_alpha.max(best_score);
            if current_alpha >= beta {
                if !board.is_capture(allegedly_best_move) {
                    self.add_killer(allegedly_best_move, depth);

                    let history_idx: usize = (((allegedly_best_move & FROM_MASK) as usize) << 6)
                        | ((allegedly_best_move & TO_MASK) >> TO_SHIFT) as usize;
                    self.history_heuristics[history_idx] += (depth * depth) as i16;
                }
                break;
            }
        }

        if total_moves < 1 {
            return if board.is_square_attacked(
                if color == 8 {
                    board.white_king_square
                } else {
                    board.black_king_square
                },
                enemy_color,
            ) {
                -CHECKMATE_VALUE + ply as i32
            } else {
                0
            };
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
        depth: usize,
        ply: usize,
        color: u16,
        node_count: &mut u64,
    ) -> i32 {
        *node_count += 1;
        let stand_pat: i32 = if color == 8 {
            self.evaluation
        } else {
            -self.evaluation
        };
        if depth >= 24 {
            return stand_pat;
        }
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

        let enemy_color: u16 = if color == 8 { 16 } else { 8 };
        let in_check: bool = if color == 8 {
            board.is_square_attacked(board.white_king_square, 16)
        } else {
            board.is_square_attacked(board.black_king_square, 8)
        };

        if !in_check {
            if stand_pat + QUIESCENCE_DELTA < alpha {
                return stand_pat;
            }
            if stand_pat >= beta {
                return stand_pat;
            }
            if stand_pat > alpha {
                alpha = stand_pat;
            }
        }

        self.generate_pseudo_legal_moves(color, board, state, ply, !in_check);
        let last_occupied: usize = self.move_lists[ply].first_not_occupied;

        self.score_all_moves(ply, last_occupied, &best_move_transposition, &board);

        let scores: &mut [i16; 192] = &mut self.move_scores[ply];
        let moves: &mut [u16; 192] = &mut self.move_lists[ply].pseudo_moves;
        Self::n_log_n_sort_moves(moves, scores, last_occupied);

        let mut best_score: i32 = stand_pat;
        let mut moves_tried: i32 = 0;

        for i in 0..last_occupied {
            let move_to_search: u16 = self.move_lists[ply].pseudo_moves[i];
            let to_sq: u16 = (move_to_search & TO_MASK) >> TO_SHIFT;
            let mut captured_piece: u16 = board.cached_pieces[to_sq as usize];

            if !in_check && captured_piece == 0 {
                continue;
            }

            if !in_check {
                let capture_value: i32 = if captured_piece != 0 {
					if captured_piece > 6 {
						captured_piece -= 6;
					}
                    VALUE_TABLE[captured_piece as usize - 1]
                } else {
                    0
                };

                if stand_pat + capture_value + QUIESCENCE_DELTA < alpha {
                    continue;
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

            let score: i32 = -self.quiescence_search(
                board,
                state,
                -beta,
                -alpha,
                depth + 1,
                ply + 1,
                enemy_color,
                node_count,
            );

            board.cancel_move(state, color, &mut self.evaluation, &mut self.current_hash);

            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }

        if moves_tried == 0 {
            if in_check {
                return -CHECKMATE_VALUE + ply as i32;
            } else {
                return stand_pat;
            }
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
        copied_state.whose_turn = self.side as u16;

        self.prepare_before_search(&mut copied_board, &mut copied_state);

        let mut previous_best_move: u16 = 0;

        let bad_draw_score: i32 = match self.side {
            8 => -50,
            _ => 50,
        };

        let mut time_limit_ms: u128 = time_contrainsts.as_millis();
        if time_limit_ms == 0 {
            time_limit_ms = 1_000_000 * 1000; // 1_000_000 seconds
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
            let depth_as_index: usize = d as usize;

            self.generate_pseudo_legal_moves(self.side, &copied_board, &copied_state, 0, false);
            let last_occupied: usize = self.move_lists[0].first_not_occupied;
            self.how_much_searched.1 = last_occupied as f32;

            self.score_all_moves(0, last_occupied, &previous_best_move, &copied_board);
            let scores: &mut [i16; 192] = &mut self.move_scores[0];
            let moves: &mut [u16; 192] = &mut self.move_lists[0].pseudo_moves;

            Self::n_log_n_sort_moves(moves, scores, last_occupied);
            let mut depth_best_score: i32 = -CHECKMATE_VALUE;
            let mut depth_best_move: u16 = 0;

            let mut moves_searched: usize = 0;

            let mut total_moves: usize = last_occupied;

            for i in 0..last_occupied {
                let allegedly_best_move: u16 = self.move_lists[0].pseudo_moves[i];

                copied_board.perform_move(
                    allegedly_best_move,
                    &mut copied_state,
                    self.side,
                    &mut self.evaluation,
                    &mut self.current_hash,
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

                let move_extension: i8 =
                    Self::move_increment(&copied_board.cached_pieces, allegedly_best_move);
                self.how_much_searched.0 += 1.;

                let mut score: i32 = -self.negamax(
                    &mut copied_board,
                    if move_extension >= 0 {
                        d - 1 + move_extension as u8
                    } else {
                        (d - 1).saturating_sub(move_extension as u8)
                    },
                    1,
                    opponent_color,
                    -CHECKMATE_VALUE,
                    CHECKMATE_VALUE,
                    &mut copied_state,
                    &mut node_count,
                    &timer_start,
                    &time_limit_ms,
                    depth_as_index,
                );

                copied_state.whose_turn = self.side;

                if score == TIMEOUT_RETURN || score == -TIMEOUT_RETURN {
                    break 'outer;
                }

                moves_searched += 1;
                // if moves_searched < 11 {
                //     println!(
                //         "{}. move: {}{}",
                //         moves_searched,
                //         INDICES_TO_COORDS
                //             .get(&from_square(allegedly_best_move))
                //             .unwrap(),
                //         INDICES_TO_COORDS
                //             .get(&(to_square(allegedly_best_move) as u8))
                //             .unwrap()
                //     );
                // }

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

                if score > depth_best_score || depth_best_move == 0 {
                    depth_best_score = score;
                    depth_best_move = allegedly_best_move;
                }
            }
            if moves_searched == total_moves
                || match self.side {
                    8 => depth_best_score <= best_score_eval,
                    _ => depth_best_score >= best_score_eval,
                }
            // do not discard best move if it's better than what we already have
            {
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
        board.calculate_check_restrictions(state, state.whose_turn);
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

        return self.percent_finished() < depth_percent;
    }

    fn percent_finished(&self) -> f32 {
        return self.how_much_searched.0 / self.how_much_searched.1;
    }

    fn move_increment(board: &[u16; 64], m: u16) -> i8 {
        let mut increment: i8 = 0;
        increment += match (m & MARK_MASK) >> MARK_SHIFT {
            3..10 => 1,
            10..14 => 2,
            _ => 0,
        };

        let (attacker, victim) = (board[from_square(m) as usize], board[to_square(m) as usize]);
        if attacker > 0 && attacker < victim {
            increment +=
                (unsafe { MVV_LVA[Self::get_piece_value(victim)][Self::get_piece_value(attacker)] }
                    / 15) as i8;
        }

        return increment;
    }

    #[allow(unused)]
    fn is_quiet(board: &[u16; 64], m: u16) -> bool {
        return board[to_square(m) as usize] == 0 && ((m & MARK_MASK) >> MARK_SHIFT) < 3;
    }
}
