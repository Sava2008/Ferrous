use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        heuristics::*, masks::BIT_MASKS, piece_values::*, zobrist_hashes::ZOBRIST_HASH_TABLE,
    },
    gamestate::{GameState, PreviousMove},
};

impl Board {
    fn perform_capture(
        &mut self,
        enemy: u32,
        previous_move: &mut PreviousMove,
        to_sq: usize,
        evaluation: &mut i32,
        current_hash: &mut u64,
        captured_table_idx: usize,
        occupancy_idx: usize,
    ) -> () {
        *evaluation -= VALUE_TABLE[captured_table_idx];
        previous_move.moved_piece |= enemy << CAPTURED_PIECE_TYPE_SHIFT;
        let to_sq_as_index: usize = to_sq as usize;
        let capture: u64 = !BIT_MASKS[to_sq_as_index];
        self.bitboards[captured_table_idx] &= capture;
        self.occupancies[occupancy_idx] &= capture;
        *current_hash ^= ZOBRIST_HASH_TABLE[captured_table_idx * 64 + to_sq_as_index];
    }

    fn castling(
        &mut self,
        previous_move: &mut PreviousMove,
        from_sq: usize,
        to_sq: usize,
        color: u32,
        eval: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        let (rook, (rook_from, rook_to)) = if color == 8 {
            (
                WHITE_ROOK_U32,
                if to_sq > from_sq { (7, 5) } else { (0, 3) },
            )
        } else {
            (
                BLACK_ROOK_U32,
                if to_sq > from_sq { (63, 61) } else { (56, 59) },
            )
        };
        self.cached_pieces
            .swap(rook_from as usize, rook_to as usize);
        previous_move.moved_piece |= 1 << CASTLING_SHIFT;

        let (start, end): (u64, u64) = (!BIT_MASKS[rook_from], BIT_MASKS[rook_to]);
        self.total_occupancy &= start;
        self.total_occupancy |= end;
        let (occupancy, (rook_from_heuristic, rook_to_heuristic), rook_bb) = match color {
            8 => (
                &mut self.occupancies[0],
                (
                    -WHITE_ROOK_HEURISTICS[rook_from],
                    WHITE_ROOK_HEURISTICS[rook_to],
                ),
                &mut self.bitboards[3],
            ),
            16 => (
                &mut self.occupancies[1],
                (
                    BLACK_ROOK_HEURISTICS[rook_from],
                    -BLACK_ROOK_HEURISTICS[rook_to],
                ),
                &mut self.bitboards[9],
            ),
            _ => unreachable!(),
        };
        *occupancy &= start;
        *occupancy |= end;
        *rook_bb &= start;
        *rook_bb |= end;
        *eval += rook_from_heuristic + rook_to_heuristic;

        let rook_hash: usize = if color == 8 {
            rook as usize - 9
        } else {
            rook as usize - 11
        } * 64;
        *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_from];
        *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_to];
    }

    fn en_passant(
        &mut self,
        e_p: u8,
        previous_move: &mut PreviousMove,
        color: u32,
        eval: &mut i32,
    ) -> () {
        let (pawns, occupancy, captured_pawn_square, material_subtraction) = match color {
            8 => (
                &mut self.bitboards[6],
                &mut self.occupancies[1],
                e_p - 8,
                PAWN_VALUE,
            ),
            16 => (
                &mut self.bitboards[0],
                &mut self.occupancies[0],
                e_p + 8,
                -PAWN_VALUE,
            ),
            _ => unreachable!(),
        };
        self.cached_pieces[captured_pawn_square as usize] = 0;
        previous_move.moved_piece |= 1 << EN_PASSANT_SHIFT;
        let capture: u64 = !BIT_MASKS[captured_pawn_square as usize];
        *pawns &= capture;
        *occupancy &= capture;
        self.total_occupancy &= capture;
        *eval += material_subtraction;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(
        &mut self,
        piece_move: u32,
        state: &mut GameState,
        color: u32,
        evaluation: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        let evaluation_before: i32 = evaluation.clone();
        let (from_sq, to_sq): (u32, u32) =
            ((piece_move & FROM_MASK), (piece_move & TO_MASK) >> TO_SHIFT);
        let (from_sq_index, to_sq_index): (usize, usize) = (from_sq as usize, to_sq as usize);

        let moving_piece: u32 = moving_piece_type(piece_move);
        let captured_piece: u32 = captured_piece_type(piece_move);

        let (moving_piece_table_idx, occupancy_idx): (usize, usize) =
            get_bb_index(moving_piece, &color);
        let (captured_piece_table_idx, captured_occupancy_idx): (usize, usize) =
            if captured_piece != 0 {
                get_bb_index(captured_piece, if color == 8 { &16 } else { &8 })
            } else {
                (0, 0)
            };

        let to_sq_index_base_one: usize = to_sq_index + 1;

        let moving_piece_hash: usize = moving_piece_table_idx * 64;

        *current_hash ^= ZOBRIST_HASH_TABLE[moving_piece_hash + from_sq_index];
        *current_hash ^= ZOBRIST_HASH_TABLE[moving_piece_hash + to_sq_index];

        let e_p: u8 = en_passant(piece_move);
        let castling: u8 = castling(piece_move);

        self.cached_pieces[to_sq as usize] = moving_piece | color;
        self.cached_pieces[from_sq as usize] = 0;

        let promotion_choice: usize = promotion(piece_move) as usize;
        let mut previous_move: PreviousMove = PreviousMove {
            moved_piece: piece_move,
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: state.castling_rights,
            material_difference: 0,
        };
        if captured_piece != 0 {
            self.perform_capture(
                captured_piece,
                &mut previous_move,
                to_sq_index,
                evaluation,
                current_hash,
                captured_piece_table_idx,
                captured_occupancy_idx,
            );
        }
        if castling != 0 {
            if moving_piece != 6 {
                panic!("piece: {moving_piece}, board: {self:?}, from: {from_sq}, to: {to_sq}");
            }
            self.castling(
                &mut previous_move,
                from_sq_index,
                to_sq_index,
                color,
                evaluation,
                current_hash,
            );
        }
        if e_p != 0 {
            self.en_passant(
                state.en_passant_target.unwrap(),
                &mut previous_move,
                color,
                evaluation,
            );

            let (en_passant_pawn_type, en_passant_pawn_offset) =
                if color == 8 { (0, 8) } else { (6, -8) };
            *current_hash ^= ZOBRIST_HASH_TABLE[en_passant_pawn_type * 64
                + ((state.en_passant_target.unwrap() as i8 + en_passant_pawn_offset) as usize)]; // removing en passant'ed pawn
        }
        match moving_piece_table_idx {
            3 => {
                if from_sq == 0 {
                    state.castling_rights &= !WHITE_LONG_MASK;
                } else if from_sq == 7 {
                    state.castling_rights &= !WHITE_SHORT_MASK;
                }
            }
            9 => {
                if from_sq == 56 {
                    state.castling_rights &= !BLACK_LONG_MASK;
                } else if from_sq == 63 {
                    state.castling_rights &= !BLACK_SHORT_MASK;
                }
            }
            5 => {
                self.white_king_square = to_sq as u8;
                state.castling_rights &= !(WHITE_LONG_MASK | WHITE_SHORT_MASK);
            }
            11 => {
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
        self.total_occupancy &= start;
        self.total_occupancy |= end;
        *occupancy &= start;
        *occupancy |= end;

        let (from_heuristic, to_heuristic) = (
            HEURISTICS_TABLE[moving_piece_table_idx][from_sq_index],
            HEURISTICS_TABLE[moving_piece_table_idx][to_sq_index],
        );

        if color == 8 {
            *evaluation += to_heuristic - from_heuristic;
        } else {
            *evaluation -= to_heuristic + from_heuristic;
        }

        let to_sq_index_base_zero: usize = to_sq_index_base_one - 1;

        if promotion_choice != 0 {
            let promotion_choice_table_idx: usize = if color == 8 {
                self.bitboards[0] &= start;
                promotion_choice
            } else {
                *&mut self.bitboards[6] &= start;
                promotion_choice + 6
            };
            self.bitboards[promotion_choice_table_idx] |= end;
            *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_zero];
            *current_hash ^=
                ZOBRIST_HASH_TABLE[promotion_choice_table_idx * 64 + to_sq_index_base_zero];
            self.cached_pieces[to_sq as usize] = U32_PIECES_TABLE[promotion_choice_table_idx];
        } else {
            *moving_piece_bb |= end;
            *moving_piece_bb &= start;
        }
        previous_move.material_difference = *evaluation - evaluation_before;
        if moving_piece == WHITE_PAWN_U32 && from_sq < 16 && to_sq > 23 && to_sq < 32 {
            state.en_passant_target = Some(to_sq as u8 - 8);
        } else if moving_piece == BLACK_PAWN_U32 && from_sq > 47 && to_sq > 40 && to_sq < 31 {
            state.en_passant_target = Some(to_sq as u8 + 8);
        } else {
            state.en_passant_target = None;
        }
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(
        &mut self,
        state: &mut GameState,
        color: u32,
        evaluation: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            *evaluation -= previous_move.material_difference;

            let m: u32 = previous_move.moved_piece;
            let (start, end, main_piece, promotion, captured_piece, castling, en_passant) = (
                from_square(m),
                to_square(m),
                moving_piece_type(m),
                promotion(m),
                captured_piece_type(m),
                castling(m),
                en_passant(m),
            );
            let enemy_color: u32 = if color == 8 { 16 } else { 8 };
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
            match moving_piece_table_idx {
                5 => self.white_king_square = start,
                11 => self.black_king_square = start,
                _ => (),
            };
            let (start_index, end_index): (usize, usize) = (start as usize, end as usize);

            let main_piece_hash: usize = moving_piece_table_idx * 64;

            *current_hash ^= ZOBRIST_HASH_TABLE[main_piece_hash + start_index];
            self.cached_pieces[end_index] = 0;

            let (moved_piece_bitboard, color_occupancy): (&mut u64, &mut u64) = (
                &mut self.bitboards[moving_piece_table_idx],
                &mut self.occupancies[moving_piece_occupancy_idx],
            );
            let start_bb: u64 = BIT_MASKS[start_index];
            let not_end_bb: u64 = !BIT_MASKS[end_index];
            self.cached_pieces[start_index] = main_piece | color;
            *moved_piece_bitboard &= not_end_bb;
            if promotion == 0 {
                *moved_piece_bitboard |= start_bb;
                *current_hash ^= ZOBRIST_HASH_TABLE[main_piece_hash + end_index];
            } else {
                let (pawns, promotion_piece_encoding, pawn) = if color == 8 {
                    (&mut self.bitboards[0], promotion as usize, WHITE_PAWN_U32)
                } else {
                    (
                        &mut self.bitboards[6],
                        promotion as usize + 6,
                        BLACK_PAWN_U32,
                    )
                };
                *pawns |= start_bb;
                self.bitboards[promotion_piece_encoding] &= not_end_bb;
                self.cached_pieces[start_index] = pawn;
                *current_hash ^= ZOBRIST_HASH_TABLE[promotion_piece_encoding * 64 + end_index];
            }

            *color_occupancy |= start_bb;
            *color_occupancy &= not_end_bb;
            self.total_occupancy |= start_bb;
            if captured_piece != 0 {
                let end_bb: u64 = BIT_MASKS[end_index];

                self.occupancies[captured_piece_occupancy_idx] |= end_bb;
                *current_hash ^= ZOBRIST_HASH_TABLE[(captured_piece_table_idx) * 64 + end_index];
                self.bitboards[captured_piece_table_idx] |= end_bb;
                self.cached_pieces[end_index] = captured_piece | enemy_color;
                self.total_occupancy |= end_bb;
            } else {
                self.cached_pieces[end_index] = 0;
                self.total_occupancy &= not_end_bb;
            }

            if en_passant != 0 {
                // placing en passant'ed pawn back on the board
                let (pawn, enemy_pawns, enemy_occupancy, taken_pawn_square): (
                    u32,
                    &mut u64,
                    &mut u64,
                    usize,
                ) = match color {
                    8 => {
                        *current_hash ^= ZOBRIST_HASH_TABLE[6 * 64 + (end_index - 8)];
                        (
                            BLACK_PAWN_U32,
                            &mut self.bitboards[6],
                            &mut self.occupancies[1],
                            end_index - 8,
                        )
                    }
                    16 => {
                        *current_hash ^= ZOBRIST_HASH_TABLE[end_index + 8];
                        (
                            WHITE_PAWN_U32,
                            &mut self.bitboards[0],
                            &mut self.occupancies[0],
                            end_index + 8,
                        )
                    }
                    _ => unreachable!(),
                };
                let taken_pawn_square_bb: u64 = BIT_MASKS[taken_pawn_square];
                *enemy_pawns |= taken_pawn_square_bb;
                self.total_occupancy |= taken_pawn_square_bb;
                *enemy_occupancy |= taken_pawn_square_bb;
                self.cached_pieces[taken_pawn_square] = pawn;
            }

            if castling != 0 {
                let (rooks, occupancy, rook_start, rook_end, rook_encoding) = match (start, end) {
                    (4, 2) => (&mut self.bitboards[3], &mut self.occupancies[0], 0, 3, 5),
                    (4, 6) => (&mut self.bitboards[3], &mut self.occupancies[0], 7, 5, 5),
                    (60, 58) => (
                        &mut self.bitboards[9],
                        &mut self.occupancies[1],
                        56 as usize,
                        59 as usize,
                        10,
                    ),
                    (60, 62) => (
                        &mut self.bitboards[9],
                        &mut self.occupancies[1],
                        63 as usize,
                        61 as usize,
                        10,
                    ),
                    _ => unreachable!("from: {start}, to {end}, piece: {main_piece}"),
                };
                let (rook_start_bb, rook_end_bb): (u64, u64) =
                    (BIT_MASKS[rook_start], !BIT_MASKS[rook_end]);
                *rooks |= rook_start_bb;
                *rooks &= rook_end_bb;
                *occupancy |= rook_start_bb;
                *occupancy &= rook_end_bb;
                self.total_occupancy |= rook_start_bb;
                self.total_occupancy &= rook_end_bb;
                self.cached_pieces.swap(rook_end, rook_start);

                let rook_hash: usize = (rook_encoding - 1) * 64;
                *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_start];
                *current_hash ^= ZOBRIST_HASH_TABLE[rook_hash + rook_end];
            }

            state.castling_rights = previous_move.previous_castling_rights;
            state.en_passant_target = previous_move.previous_en_passant;
        }
    }
}
