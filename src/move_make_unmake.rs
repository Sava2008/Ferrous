use std::u16;

use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        attacks::{
            BLACK_PAWN_ATTACKS, EN_PASSANT_TARGETS, KNIGHT_ATTACKS, RAYS_BETWEEN,
            WHITE_PAWN_ATTACKS, bishop_attacks, rook_attacks,
        },
        heuristics::*,
        masks::BIT_MASKS,
        piece_values::*,
        zobrist_hashes::{BLACK_ZOBRIST_KEY, WHITE_ZOBRIST_KEY, ZOBRIST_HASH_TABLE},
    },
    gamestate::{GameState, PreviousMove},
};

impl Board {
    fn perform_capture(
        &mut self,
        state: &mut GameState,
        enemy: u16,
        previous_move: &mut PreviousMove,
        to_sq: usize,
        evaluation: &mut i32,
        current_hash: &mut u64,
        captured_table_idx: usize,
        occupancy_idx: usize,
        color: u16,
    ) -> () {
        *evaluation -= VALUE_TABLE[captured_table_idx];
        let dest_heuristic: i32 = unsafe { HEURISTICS_TABLE[captured_table_idx][to_sq] };
        *evaluation -= if color == 8 {
            -dest_heuristic
        } else {
            dest_heuristic
        };
        previous_move.captured_piece |= enemy;
        let capture: u64 = !BIT_MASKS[to_sq];
        if enemy == WHITE_ROOK_U16 {
            if to_sq == 7 {
                state.castling_rights &= !WHITE_SHORT;
            } else if to_sq == 0 {
                state.castling_rights &= !WHITE_LONG;
            }
        } else if enemy == BLACK_ROOK_U16 {
            if to_sq == 63 {
                state.castling_rights &= !BLACK_SHORT;
            } else if to_sq == 56 {
                state.castling_rights &= !BLACK_LONG;
            }
        }
        self.bitboards[captured_table_idx] &= capture;
        self.occupancies[occupancy_idx] &= capture;
        *current_hash ^= ZOBRIST_HASH_TABLE[captured_table_idx * 64 + to_sq];
    }

    #[cold]
    fn castling(
        &mut self,
        previous_move: &mut PreviousMove,
        from_sq: usize,
        to_sq: usize,
        color: u16,
        eval: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        let (
            rook,
            rook_from,
            rook_to,
            occupancy,
            (rook_from_heuristic, rook_to_heuristic),
            rook_bb,
        ) = if color == 8 {
            let (rook_from_idx, rook_to_idx) = if to_sq > from_sq { (7, 5) } else { (0, 3) };
            (
                WHITE_ROOK_U16,
                rook_from_idx,
                rook_to_idx,
                &mut self.occupancies[0],
                (
                    WHITE_ROOK_HEURISTICS[rook_from_idx],
                    WHITE_ROOK_HEURISTICS[rook_to_idx],
                ),
                &mut self.bitboards[3],
            )
        } else {
            let (rook_from_idx, rook_to_idx) = if to_sq > from_sq { (63, 61) } else { (56, 59) };
            (
                BLACK_ROOK_U16,
                rook_from_idx,
                rook_to_idx,
                &mut self.occupancies[1],
                (
                    BLACK_ROOK_HEURISTICS[rook_from_idx],
                    BLACK_ROOK_HEURISTICS[rook_to_idx],
                ),
                &mut self.bitboards[9],
            )
        };

        self.cached_pieces.swap(rook_from, rook_to);
        previous_move.move_flag = 0b0001;

        let (start, end): (u64, u64) = (!BIT_MASKS[rook_from], BIT_MASKS[rook_to]);
        self.total_occupancy = (self.total_occupancy & start) | end;

        *occupancy = (*occupancy & start) | end;
        *rook_bb = (*rook_bb & start) | end;
        *eval += if color == 8 {
            rook_to_heuristic - rook_from_heuristic
        } else {
            rook_from_heuristic - rook_to_heuristic
        };

        let rook_hash: usize = (rook as usize - 1) * 64;
        *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_from];
        *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_to];
    }

    #[cold]
    fn en_passant(&mut self, e_p: u8, color: u16, eval: &mut i32) -> () {
        let (pawns, occupancy, captured_pawn_square, mut material_subtraction) = match color {
            8 => (
                &mut self.bitboards[6],
                &mut self.occupancies[1],
                e_p as usize - 8,
                PAWN_VALUE,
            ),
            _ => (
                &mut self.bitboards[0],
                &mut self.occupancies[0],
                e_p as usize + 8,
                -PAWN_VALUE,
            ),
        };
        let piece_heuristics = &raw const HEURISTICS_TABLE;
        material_subtraction -= if color == 8 {
            unsafe { -(*piece_heuristics)[6][captured_pawn_square] }
        } else {
            unsafe { (*piece_heuristics)[0][captured_pawn_square] }
        };
        self.cached_pieces[captured_pawn_square] = 0;
        let capture: u64 = !BIT_MASKS[captured_pawn_square];
        *pawns &= capture;
        *occupancy &= capture;
        self.total_occupancy &= capture;
        *eval += material_subtraction;
    }

    #[cold]
    #[inline(never)]
    fn promote_pawn(
        &mut self,
        zobrist_table: &[u64; 768],
        current_hash: &mut u64,
        evaluation: &mut i32,
        color: u16,
        start: &u64,
        promotion_choice: usize,
        to_sq_index_base_zero: usize,
        end: &u64,
    ) -> () {
        let promotion_choice_table_idx: usize = if color == 8 {
            self.bitboards[0] &= start;
            promotion_choice
        } else {
            self.bitboards[6] &= start;
            promotion_choice + 6
        };
        *evaluation += VALUE_TABLE[promotion_choice_table_idx]
            + if color == 8 { -PAWN_VALUE } else { PAWN_VALUE };
        self.bitboards[promotion_choice_table_idx] |= end;
        *current_hash ^= zobrist_table[to_sq_index_base_zero];
        *current_hash ^= zobrist_table[promotion_choice_table_idx * 64 + to_sq_index_base_zero];
        self.cached_pieces[to_sq_index_base_zero] = U16_PIECES_TABLE[promotion_choice_table_idx];
    }

    #[inline(always)]
    pub fn adjust_move_restriction(
        &self,
        state: &mut GameState,
        to: u16,
        flag: u16,
        moving_piece: u16,
        color: u16,
    ) -> () {
        let to_sq_bb: u64 = 1 << to;
        let total_occ: u64 = self.total_occupancy;
        if color == 8 {
            let king_sq: u8 = self.black_king_square;
            let king_sq_index: usize = king_sq as usize;
            let diag_discovery_attacks: usize = ((bishop_attacks(king_sq_index, total_occ)
                & !to_sq_bb)
                & (self.bitboards[4] | self.bitboards[2]))
                .trailing_zeros() as usize;
            let line_discovery_attacks: usize = ((rook_attacks(king_sq_index, total_occ)
                & !to_sq_bb)
                & (self.bitboards[4] | self.bitboards[3]))
                .trailing_zeros() as usize;

            let squares: u64 = unsafe {
                if diag_discovery_attacks != 64 {
                    RAYS_BETWEEN[king_sq_index][diag_discovery_attacks]
                        | (1 << diag_discovery_attacks)
                } else if line_discovery_attacks != 64 {
                    RAYS_BETWEEN[king_sq_index][line_discovery_attacks]
                        | (1 << line_discovery_attacks)
                } else {
                    64
                }
            };
            let direct_attacks: u64 = match moving_piece {
                5 => {
                    bishop_attacks(king_sq_index, total_occ)
                        | rook_attacks(king_sq_index, total_occ)
                } // queen
                4 => rook_attacks(king_sq_index, total_occ), // rook
                3 => bishop_attacks(king_sq_index, total_occ), // bishop
                2 => KNIGHT_ATTACKS[king_sq_index],
                1 => match flag {
                    0 | 2 => BLACK_PAWN_ATTACKS[king_sq_index],
                    3 => KNIGHT_ATTACKS[king_sq_index],
                    4 => bishop_attacks(king_sq_index, total_occ),
                    5 => rook_attacks(king_sq_index, total_occ),
                    6 => {
                        bishop_attacks(king_sq_index, total_occ)
                            | rook_attacks(king_sq_index, total_occ)
                    }
                    _ => unreachable!(),
                },
                6 => 0, // king
                _ => unreachable!(),
            };
            let direct_attack: bool = direct_attacks & to_sq_bb != 0;
            let is_discovery: bool = squares != 64;
            if direct_attack && is_discovery {
                state.black_legal_squares_mask = 0;
                return ();
            }
            if direct_attack {
                state.black_legal_squares_mask =
                    unsafe { RAYS_BETWEEN[king_sq_index][to as usize] | to_sq_bb };
                return ();
            }
            if is_discovery {
                state.black_legal_squares_mask = squares;
                return ();
            }

            state.black_legal_squares_mask = u64::MAX;
        } else {
            let king_sq: u8 = self.white_king_square;
            let king_sq_index: usize = king_sq as usize;
            let diag_discovery_attacks: usize = ((bishop_attacks(king_sq_index, total_occ)
                & !to_sq_bb)
                & (self.bitboards[10] | self.bitboards[8]))
                .trailing_zeros() as usize;
            let line_discovery_attacks: usize = ((rook_attacks(king_sq_index, total_occ)
                & !to_sq_bb)
                & (self.bitboards[10] | self.bitboards[9]))
                .trailing_zeros() as usize;

            let squares: u64 = unsafe {
                if diag_discovery_attacks != 64 {
                    RAYS_BETWEEN[king_sq_index][diag_discovery_attacks]
                        | (1 << diag_discovery_attacks)
                } else if line_discovery_attacks != 64 {
                    RAYS_BETWEEN[king_sq_index][line_discovery_attacks]
                        | (1 << line_discovery_attacks)
                } else {
                    64
                }
            };
            let direct_attacks: u64 = match moving_piece {
                11 => {
                    bishop_attacks(king_sq_index, total_occ)
                        | rook_attacks(king_sq_index, total_occ)
                }
                // queen
                10 => rook_attacks(king_sq_index, total_occ), // rook
                9 => bishop_attacks(king_sq_index, total_occ), // bishop
                8 => KNIGHT_ATTACKS[king_sq_index],
                7 => match flag {
                    0 | 2 => WHITE_PAWN_ATTACKS[king_sq_index],
                    3 => KNIGHT_ATTACKS[king_sq_index],
                    4 => bishop_attacks(king_sq_index, total_occ),
                    5 => rook_attacks(king_sq_index, total_occ),
                    6 => {
                        bishop_attacks(king_sq_index, total_occ)
                            | rook_attacks(king_sq_index, total_occ)
                    }
                    _ => unreachable!(),
                },
                12 => 0, // king
                _ => unreachable!(),
            };
            let direct_attack: bool = direct_attacks & to_sq_bb != 0;
            let is_discovery: bool = squares != 64;
            if direct_attack && is_discovery {
                state.white_legal_squares_mask = 0;
                return ();
            }
            if direct_attack {
                state.white_legal_squares_mask =
                    unsafe { RAYS_BETWEEN[king_sq_index][to as usize] } | to_sq_bb;
                return ();
            }
            if is_discovery {
                state.white_legal_squares_mask = squares;
                return ();
            }
            state.white_legal_squares_mask = u64::MAX;
        }
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(
        &mut self,
        piece_move: u16,
        state: &mut GameState,
        color: u16,
        evaluation: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        let evaluation_before: i32 = *evaluation;
        let (from_sq, to_sq): (u16, u16) =
            ((piece_move & FROM_MASK), (piece_move & TO_MASK) >> TO_SHIFT);
        let (from_sq_index, to_sq_index): (usize, usize) = (from_sq as usize, to_sq as usize);

        let (cached_pieces, zobrist_table) = (&self.cached_pieces, &ZOBRIST_HASH_TABLE);

        let moving_piece: u16 = cached_pieces[from_sq_index];
        let captured_piece: u16 = cached_pieces[to_sq_index];
        let move_flag: u16 = (piece_move & MARK_MASK) >> MARK_SHIFT;

        let (moving_piece_table_idx, occupancy_idx): (usize, usize) =
            get_bb_index(moving_piece, &color);
        let (captured_piece_table_idx, captured_occupancy_idx): (usize, usize) =
            if captured_piece != 0 {
                get_bb_index(captured_piece, if color == 8 { &16 } else { &8 })
            } else {
                (0, 0)
            };

        let check_restrictions: u64 = if color == 8 {
            state.black_legal_squares_mask
        } else {
            state.white_legal_squares_mask
        };

        let moving_piece_hash: usize = moving_piece_table_idx * 64;

        *current_hash ^= zobrist_table[moving_piece_hash + from_sq_index];
        *current_hash ^= zobrist_table[moving_piece_hash + to_sq_index];

        let promotion_choice: usize = if move_flag > 2 && move_flag < 7 {
            move_flag as usize - 2
        } else {
            0
        };
        let mut previous_move: PreviousMove = PreviousMove {
            moved_piece: piece_move,
            captured_piece: captured_piece,
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: state.castling_rights,
            material_difference: 0,
            move_flag,
            check_restrictions,
        };
        if captured_piece != 0 {
            self.perform_capture(
                state,
                captured_piece,
                &mut previous_move,
                to_sq_index,
                evaluation,
                current_hash,
                captured_piece_table_idx,
                captured_occupancy_idx,
                color,
            );
        }
        if move_flag == 1 {
            self.castling(
                &mut previous_move,
                from_sq_index,
                to_sq_index,
                color,
                evaluation,
                current_hash,
            );
        } else if move_flag == 2 {
            self.en_passant(state.en_passant_target.unwrap(), color, evaluation);

            let (en_passant_pawn_type, en_passant_pawn_offset) =
                if color == 8 { (0, 8) } else { (6, -8) };
            *current_hash ^= zobrist_table[en_passant_pawn_type * 64
                + ((state.en_passant_target.unwrap() as i8 + en_passant_pawn_offset) as usize)]; // removing en passant'ed pawn
        }

        let cached_pieces: &mut [u16; 64] = &mut self.cached_pieces;
        cached_pieces[to_sq_index] = moving_piece;
        cached_pieces[from_sq_index] = 0;

        match moving_piece {
            4 => {
                if from_sq == 0 {
                    state.castling_rights &= !WHITE_LONG_MASK;
                } else if from_sq == 7 {
                    state.castling_rights &= !WHITE_SHORT_MASK;
                }
            }
            10 => {
                if from_sq == 56 {
                    state.castling_rights &= !BLACK_LONG_MASK;
                } else if from_sq == 63 {
                    state.castling_rights &= !BLACK_SHORT_MASK;
                }
            }
            6 => {
                self.white_king_square = to_sq as u8;
                state.castling_rights &= !(WHITE_LONG_MASK | WHITE_SHORT_MASK);
            }
            12 => {
                self.black_king_square = to_sq as u8;
                state.castling_rights &= !(BLACK_LONG_MASK | BLACK_SHORT_MASK);
            }
            _ => (),
        };
        let (occupancy, moving_piece_bb) = (
            &mut self.occupancies[occupancy_idx],
            &mut self.bitboards[moving_piece_table_idx],
        );

        let (start, end): (u64, u64) = (!BIT_MASKS[from_sq_index], BIT_MASKS[to_sq_index]);
        self.total_occupancy = (self.total_occupancy & start) | end;
        *occupancy = (*occupancy & start) | end;

        let moving_piece_heuristics: &[i32; 64] =
            unsafe { &HEURISTICS_TABLE[moving_piece_table_idx] };

        let (from_heuristic, to_heuristic) = (
            moving_piece_heuristics[from_sq_index],
            moving_piece_heuristics[to_sq_index],
        );

        if color == 8 {
            *current_hash ^= WHITE_ZOBRIST_KEY;
            *evaluation += to_heuristic - from_heuristic;
        } else {
            *current_hash ^= BLACK_ZOBRIST_KEY;
            *evaluation += from_heuristic - to_heuristic;
        }

        if promotion_choice != 0 {
            self.promote_pawn(
                zobrist_table,
                current_hash,
                evaluation,
                color,
                &start,
                promotion_choice,
                to_sq_index,
                &end,
            );
        } else {
            *moving_piece_bb = (*moving_piece_bb & start) | end;
        }
        if moving_piece == WHITE_PAWN_U16 || moving_piece == BLACK_PAWN_U16 {
            let potential_en_passant: u8 = EN_PASSANT_TARGETS[to_sq_index][from_sq_index];
            if potential_en_passant < 64 {
                state.en_passant_target = Some(potential_en_passant);
            }
        } else {
            state.en_passant_target = None;
        }

        *evaluation -= state.pawn_structure.sum();
        self.modify_pawn_structure(&mut state.pawn_structure);
        *evaluation += state.pawn_structure.sum();

        previous_move.material_difference = *evaluation - evaluation_before;
        self.adjust_move_restriction(state, to_sq, move_flag, moving_piece, color);
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(
        &mut self,
        state: &mut GameState,
        color: u16,
        evaluation: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            *evaluation -= previous_move.material_difference;

            let (cached_pieces, zobrist_table) = (&mut self.cached_pieces, &ZOBRIST_HASH_TABLE);

            let m: u16 = previous_move.moved_piece;
            let (start, end, captured_piece) =
                (from_square(m), to_square(m), previous_move.captured_piece);
            let (start_index, end_index): (usize, usize) = (start as usize, end as usize);
            let main_piece: u16 = cached_pieces[end_index];
            let enemy_color: u16 = if color == 8 {
                state.black_legal_squares_mask = previous_move.check_restrictions;
                *current_hash ^= WHITE_ZOBRIST_KEY;
                16
            } else {
                state.white_legal_squares_mask = previous_move.check_restrictions;
                *current_hash ^= BLACK_ZOBRIST_KEY;
                8
            };
            let (
                (moving_piece_table_idx, moving_piece_occupancy_idx),
                (captured_piece_table_idx, captured_piece_occupancy_idx),
            ) = (
                get_bb_index(main_piece, &color),
                if captured_piece != 0 {
                    get_bb_index(captured_piece, &enemy_color)
                } else {
                    (0, 0)
                },
            );

            match main_piece {
                6 => self.white_king_square = start,
                12 => self.black_king_square = start,
                _ => (),
            };

            let (promotion, castling, en_passant) = match previous_move.move_flag {
                0 => (0, 0, 0),
                1 => (0, 1, 0),
                2 => (0, 0, 1),
                3 | 4 | 5 | 6 => (previous_move.move_flag - 2, 0, 0),
                _ => unreachable!(),
            };

            let main_piece_hash: usize = moving_piece_table_idx * 64;

            *current_hash ^= zobrist_table[main_piece_hash + start_index];
            cached_pieces[end_index] = 0;

            let (moved_piece_bitboard, color_occupancy): (&mut u64, &mut u64) = (
                &mut self.bitboards[moving_piece_table_idx],
                &mut self.occupancies[moving_piece_occupancy_idx],
            );
            let start_bb: u64 = BIT_MASKS[start_index];
            let not_end_bb: u64 = !BIT_MASKS[end_index];
            cached_pieces[start_index] = main_piece;
            *moved_piece_bitboard &= not_end_bb;
            if promotion == 0 {
                *moved_piece_bitboard |= start_bb;
                *current_hash ^= zobrist_table[main_piece_hash + end_index];
            } else {
                let promotion_as_index: usize = promotion as usize;
                let (pawns, promotion_piece_encoding, pawn) = if color == 8 {
                    (&mut self.bitboards[0], promotion_as_index, WHITE_PAWN_U16)
                } else {
                    (
                        &mut self.bitboards[6],
                        promotion_as_index + 6,
                        BLACK_PAWN_U16,
                    )
                };
                *pawns |= start_bb;
                self.bitboards[promotion_piece_encoding] &= not_end_bb;
                cached_pieces[start_index] = pawn;
                *current_hash ^= zobrist_table[promotion_piece_encoding * 64 + end_index];
            }

            *color_occupancy = (*color_occupancy & not_end_bb) | start_bb;
            self.total_occupancy |= start_bb;
            if captured_piece != 0 {
                let end_bb: u64 = BIT_MASKS[end_index];

                self.occupancies[captured_piece_occupancy_idx] |= end_bb;
                *current_hash ^= zobrist_table[captured_piece_table_idx * 64 + end_index];
                self.bitboards[captured_piece_table_idx] |= end_bb;
                cached_pieces[end_index] = captured_piece;
                self.total_occupancy |= end_bb;
            } else {
                cached_pieces[end_index] = 0;
                self.total_occupancy &= not_end_bb;
            }

            if en_passant != 0 {
                // placing en passant'ed pawn back on the board
                let (pawn, enemy_pawns, enemy_occupancy, taken_pawn_square): (
                    u16,
                    &mut u64,
                    &mut u64,
                    usize,
                ) = match color {
                    8 => {
                        *current_hash ^= zobrist_table[6 * 64 + (end_index - 8)];
                        (
                            BLACK_PAWN_U16,
                            &mut self.bitboards[6],
                            &mut self.occupancies[1],
                            end_index - 8,
                        )
                    }
                    _ => {
                        *current_hash ^= zobrist_table[end_index + 8];
                        (
                            WHITE_PAWN_U16,
                            &mut self.bitboards[0],
                            &mut self.occupancies[0],
                            end_index + 8,
                        )
                    }
                };
                let taken_pawn_square_bb: u64 = BIT_MASKS[taken_pawn_square];
                *enemy_pawns |= taken_pawn_square_bb;
                self.total_occupancy |= taken_pawn_square_bb;
                *enemy_occupancy |= taken_pawn_square_bb;
                cached_pieces[taken_pawn_square] = pawn;
            }

            if castling != 0 {
                let (rooks, occupancy, rook_start, rook_end, rook_encoding) = match (start, end) {
                    (4, 2) => (&mut self.bitboards[3], &mut self.occupancies[0], 0, 3, 4),
                    (4, 6) => (&mut self.bitboards[3], &mut self.occupancies[0], 7, 5, 4),
                    (60, 58) => (&mut self.bitboards[9], &mut self.occupancies[1], 56, 59, 10),
                    (60, 62) => (&mut self.bitboards[9], &mut self.occupancies[1], 63, 61, 10),
                    _ => unreachable!("from: {start}, to {end}, piece: {main_piece}"),
                };
                let (rook_start_bb, rook_end_bb): (u64, u64) =
                    (BIT_MASKS[rook_start], !BIT_MASKS[rook_end]);

                *rooks = (*rooks & rook_end_bb) | rook_start_bb;
                *occupancy = (*occupancy & rook_end_bb) | rook_start_bb;
                self.total_occupancy = (self.total_occupancy & rook_end_bb) | rook_start_bb;
                cached_pieces.swap(rook_end, rook_start);

                let rook_hash: usize = (rook_encoding - 1) * 64;
                *current_hash ^= zobrist_table[rook_hash + rook_start];
                *current_hash ^= zobrist_table[rook_hash + rook_end];
            }

            state.castling_rights = previous_move.previous_castling_rights;
            state.en_passant_target = previous_move.previous_en_passant;
        }
    }
}
