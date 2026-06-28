use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        heuristics::*,
        piece_values::*,
        zobrist_hashes::{BLACK_ZOBRIST_KEY, WHITE_ZOBRIST_KEY, ZOBRIST_HASH_TABLE},
    },
    converters::fen_converter::board_to_fen,
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
// const TIME_CHECK_NODES_OFFSET: u64 = 1000000000000000; // how often to check for timeout
// const TIMEOUT_RETURN: i32 = 2_000_001;
// const QUIESCENCE_DELTA: i32 = 100;
const BAD_DRAW_SCORE: i32 = 50;

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

        if self.killer_moves[depth][0] == Some(killer) {
            return;
        }

        self.killer_moves[depth][1] = self.killer_moves[depth][0];
        self.killer_moves[depth][0] = Some(killer);
    }

    pub fn find_best_move(
        &mut self,
        mut board: Board,
        mut state: GameState,
        time_constraints: Duration,
        max_depth: usize,
    ) -> Option<u16> {
        board.total_occupancy();
        board.update_full_cache();
        board.calculate_check_restrictions(&mut state, self.side);
        self.evaluate(&board);
        self.prepare_before_search(&board);
        let mut best_move: Option<u16> = None;
        let mut previous_best_move: u16 = 0;
        let opponent_color: u16 = if self.side == 8 { 16 } else { 8 };
        let bad_draw_score: i32 = if self.side == 8 {
            -BAD_DRAW_SCORE
        } else {
            BAD_DRAW_SCORE
        };
        let mut nodes: u64 = 0;

        let timer: Instant = Instant::now();

        'depth_loop: for d in 1..max_depth {
            self.generate_pseudo_legal_moves(self.side, &board, &state, d, false);
            let last_occupied: usize = self.move_lists[d].first_not_occupied;
            self.score_all_moves(d, last_occupied, &previous_best_move, &board);

            let (scores, moves) = (
                &mut self.move_scores[d],
                &mut self.move_lists[d].pseudo_moves,
            );
            Self::sort_moves(scores, moves, last_occupied);
            let mut depth_best_score: i32 = if self.side == 8 {
                -CHECKMATE_VALUE
            } else {
                CHECKMATE_VALUE
            };
            let mut depth_best_move: u16 = 0;

            for i in 0..last_occupied {
                if timer.elapsed() >= time_constraints {
                    break 'depth_loop;
                }

                let move_to_search: u16 = self.move_lists[d].pseudo_moves[i];

                let hash_before = self.current_hash;
                board.perform_move(
                    move_to_search,
                    &mut state,
                    &mut self.evaluation,
                    &mut self.current_hash,
                );
                assert_eq!(
                    self.current_hash,
                    Self::rebuild_hash(&board, opponent_color)
                );
                let current_king_square: u8 = match self.side {
                    8 => board.white_king_square,
                    _ => board.black_king_square,
                };

                if board.is_square_attacked(current_king_square, opponent_color) {
                    board.cancel_move(&mut state, &mut self.evaluation, &mut self.current_hash);
                    continue;
                }
                nodes += 1;

                let mut score: i32 = self.alpha_beta_pruning(
                    -CHECKMATE_VALUE,
                    CHECKMATE_VALUE,
                    d - 1,
                    &mut board,
                    &mut state,
                    &timer,
                    &time_constraints,
                    &mut nodes,
                );
                if state.is_repetition(self.current_hash) || state.fifty_moves_rule_counter >= 98 {
                    score = if match self.side {
                        8 => score < bad_draw_score,
                        _ => score > bad_draw_score,
                    } {
                        0
                    } else {
                        bad_draw_score
                    };
                }

                board.cancel_move(&mut state, &mut self.evaluation, &mut self.current_hash);
                assert_eq!(hash_before, self.current_hash);

                if match self.side {
                    8 => score > depth_best_score,
                    _ => score < depth_best_score,
                } || depth_best_move == 0
                {
                    depth_best_score = score;
                    depth_best_move = move_to_search;
                }
            }
            previous_best_move = depth_best_move;
            println!("reached depth {d}, score: {depth_best_score}");
        }
        if previous_best_move != 0 {
            best_move = Some(previous_best_move);
        }

        return best_move;
    }

    #[inline(always)]
    fn alpha_beta_pruning(
        &mut self,
        mut alpha: i32,
        mut beta: i32,
        depth: usize,
        board: &mut Board,
        state: &mut GameState,
        timer: &Instant,
        time_constraints: &Duration,
        node_count: &mut u64,
    ) -> i32 {
        *node_count += 1;

        let tt_entry: Option<TTEntry> = self.transposition_table.get_entry(&self.current_hash);
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
            return self.evaluation;
        }
        let opponent_color: u16 = if state.whose_turn == 8 { 16 } else { 8 };
        let checkmate_for_current: i32 = -(CHECKMATE_VALUE - depth as i32);
        self.generate_pseudo_legal_moves(state.whose_turn, board, state, depth, false);
        let last_occupied: usize = self.move_lists[depth].first_not_occupied;

        self.score_all_moves(depth, last_occupied, &best_move_transposition, &board);
        Self::sort_moves(
            &mut self.move_scores[depth],
            &mut self.move_lists[depth].pseudo_moves,
            last_occupied,
        );

        let mut best_score: i32 = alpha;
        let mut total_moves: i32 = 0;
        for i in 0..last_occupied {
            let move_to_search: u16 = self.move_lists[depth].pseudo_moves[i];
            let hash_before = self.current_hash;
            board.perform_move(
                move_to_search,
                state,
                &mut self.evaluation,
                &mut self.current_hash,
            );
            assert_eq!(
                self.current_hash,
                Self::rebuild_hash(&board, state.whose_turn),
                "{:?}, board: {}",
                state.moves_history,
                board_to_fen(&board, &state, &(state.whose_turn.clone() as u8)),
            );
            if board.is_square_attacked(
                if state.whose_turn == 16 {
                    board.white_king_square
                } else {
                    board.black_king_square
                },
                state.whose_turn,
            ) {
                board.cancel_move(state, &mut self.evaluation, &mut self.current_hash);
                continue;
            }
            total_moves += 1;

            let score: i32 = -self.alpha_beta_pruning(
                -beta,
                -alpha,
                depth - 1,
                board,
                state,
                timer,
                time_constraints,
                node_count,
            );
            board.cancel_move(state, &mut self.evaluation, &mut self.current_hash);
            assert_eq!(hash_before, self.current_hash, "{:?}", state.moves_history,);

            if score >= beta {
                return beta;
            }
            if score > best_score {
                best_score = score;
            }
            alpha = alpha.max(score);
            if !board.is_capture(move_to_search) {
                self.add_killer(move_to_search, depth as u8);
                let history_idx = (((move_to_search & FROM_MASK) as usize) << 6)
                    | ((move_to_search & TO_MASK) >> TO_SHIFT) as usize;
                self.history_heuristics[history_idx] += (depth * depth) as i16;
            }
        }

        if total_moves < 1 {
            if board.is_square_attacked(
                if state.whose_turn == 8 {
                    board.white_king_square
                } else {
                    board.black_king_square
                },
                opponent_color,
            ) {
                return checkmate_for_current;
            } else {
                return 0;
            }
        }
        let flag: u8 = if best_score >= beta {
            1
        } else if best_score <= alpha {
            2
        } else {
            0
        };

        self.transposition_table.record_entry(
            &self.current_hash,
            TTEntry {
                hash: self.current_hash,
                score: best_score,
                depth: depth,
                flag,
                best_move: 0,
            },
        );
        return best_score;
    }

    #[inline(always)]
    fn sort_moves(scores: &mut [i16; 192], moves: &mut [u16; 192], last_occupied: usize) -> () {
        for i in 0..last_occupied {
            let (best_move_index_offset, _) = scores[i..last_occupied]
                .iter()
                .enumerate()
                .max_by_key(|(_, score)| **score)
                .unwrap();
            let best_move_index: usize = i + best_move_index_offset;
            scores.swap(i, best_move_index);
            moves.swap(i, best_move_index);
        }
    }

    fn prepare_before_search(&mut self, board: &Board) -> () {
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

        self.current_hash = Self::rebuild_hash(board, self.side);
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

        if self.percent_finished() <= depth_percent {
            return true;
        }

        return false;
    }

    fn percent_finished(&self) -> f32 {
        return self.how_much_searched.0 / self.how_much_searched.1;
    }
}
