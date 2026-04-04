use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::{
        heuristics::*, masks::BIT_MASKS, piece_values::*, zobrist_hashes::ZOBRIST_HASH_TABLE,
    },
    gamestate::{GameState, PreviousMove},
};

impl Board {
    #[inline(always)]
    fn reset_bit(
        &mut self,
        piece: u32,
        bit_position1: usize,
        bit_position2: usize,
        color: u32,
    ) -> () {
        let from_mask: u64 = !BIT_MASKS[bit_position1];
        let to_mask: u64 = BIT_MASKS[bit_position2];
        let bitboard_to_mutate: &mut u64 = if color == 8 {
            match piece {
                WHITE_PAWN_U32 => &mut self.white_pawns,
                WHITE_KNIGHT_U32 => &mut self.white_knights,
                WHITE_BISHOP_U32 => &mut self.white_bishops,
                WHITE_QUEEN_U32 => &mut self.white_queens,
                WHITE_KING_U32 => &mut self.white_king,
                WHITE_ROOK_U32 => &mut self.white_rooks,
                _ => unreachable!(),
            }
        } else {
            match piece {
                BLACK_PAWN_U32 => &mut self.black_pawns,
                BLACK_KNIGHT_U32 => &mut self.black_knights,
                BLACK_BISHOP_U32 => &mut self.black_bishops,
                BLACK_QUEEN_U32 => &mut self.black_queens,
                BLACK_KING_U32 => &mut self.black_king,
                BLACK_ROOK_U32 => &mut self.black_rooks,
                _ => unreachable!(),
            }
        };

        *bitboard_to_mutate &= from_mask;
        *bitboard_to_mutate |= to_mask;
    }

    fn perform_capture(
        &mut self,
        state: &mut GameState,
        enemy: u32,
        previous_move: &mut PreviousMove,
        to_sq: usize,
        color: u32,
        m: &u32,
        evaluation: &mut i32,
        current_hash: &mut u64,
    ) -> () {
        let (bitboard_for_capture, occupancy, encoded_capture): (&mut u64, &mut u64, usize) =
            if color == 16 {
                match enemy {
                    WHITE_BISHOP_U32 => {
                        *evaluation -= BISHOP_VALUE;
                        *evaluation -= WHITE_BISHOP_HEURISTICS[to_sq];
                        (&mut self.white_bishops, &mut self.white_occupancy, 3)
                    }
                    WHITE_KNIGHT_U32 => {
                        *evaluation -= KNIGHT_VALUE;
                        *evaluation -= WHITE_KNIGHT_HEURISTICS[to_sq];
                        (&mut self.white_knights, &mut self.white_occupancy, 2)
                    }
                    WHITE_PAWN_U32 => {
                        *evaluation -= PAWN_VALUE;
                        *evaluation -= WHITE_PAWN_HEURISTICS[to_sq];
                        (&mut self.white_pawns, &mut self.white_occupancy, 1)
                    }
                    WHITE_QUEEN_U32 => {
                        *evaluation -= QUEEN_VALUE;
                        *evaluation -= WHITE_QUEEN_HEURISTICS[to_sq];
                        (&mut self.white_queens, &mut self.white_occupancy, 5)
                    }
                    WHITE_ROOK_U32 => {
                        *evaluation -= ROOK_VALUE;
                        *evaluation -= WHITE_ROOK_HEURISTICS[to_sq];
                        if to_sq == 7 {
                            previous_move.previous_castling_rights =
                                Some(state.castling_rights.clone());
                            state.castling_rights &= !WHITE_SHORT;
                        } else if to_sq == 0 {
                            previous_move.previous_castling_rights =
                                Some(state.castling_rights.clone());
                            state.castling_rights &= !WHITE_LONG;
                        }
                        (&mut self.white_rooks, &mut self.white_occupancy, 4)
                    }
                    WHITE_KING_U32 => {
                        panic!(
                            "attemped to capture white king. state: {state:?}, board: {self:?}, from: {}, to: {}, attacker: {}",
                            m & FROM_MASK,
                            (m & TO_MASK) >> TO_SHIFT,
                            moving_piece(*m),
                        )
                    }
                    _ => unreachable!("piece {enemy}, board: {self:?}"),
                }
            } else {
                match enemy {
                    BLACK_BISHOP_U32 => {
                        *evaluation += BISHOP_VALUE;
                        *evaluation += BLACK_BISHOP_HEURISTICS[to_sq];
                        (&mut self.black_bishops, &mut self.black_occupancy, 9)
                    }
                    BLACK_KNIGHT_U32 => {
                        *evaluation += KNIGHT_VALUE;
                        *evaluation += BLACK_KNIGHT_HEURISTICS[to_sq];
                        (&mut self.black_knights, &mut self.black_occupancy, 8)
                    }
                    BLACK_PAWN_U32 => {
                        *evaluation += PAWN_VALUE;
                        *evaluation += BLACK_PAWN_HEURISTICS[to_sq];
                        (&mut self.black_pawns, &mut self.black_occupancy, 7)
                    }
                    BLACK_QUEEN_U32 => {
                        *evaluation += QUEEN_VALUE;
                        *evaluation += BLACK_QUEEN_HEURISTICS[to_sq];
                        (&mut self.black_queens, &mut self.black_occupancy, 11)
                    }
                    BLACK_ROOK_U32 => {
                        *evaluation += ROOK_VALUE;
                        *evaluation += BLACK_ROOK_HEURISTICS[to_sq];
                        if to_sq == 63 {
                            previous_move.previous_castling_rights =
                                Some(state.castling_rights.clone());
                            state.castling_rights &= !BLACK_SHORT;
                        } else if to_sq == 56 {
                            previous_move.previous_castling_rights =
                                Some(state.castling_rights.clone());
                            state.castling_rights &= !BLACK_LONG;
                        }
                        (&mut self.black_rooks, &mut self.black_occupancy, 10)
                    }
                    BLACK_KING_U32 => {
                        panic!(
                            "attemped to capture black king. state: {state:?}, board: {self:?}, from: {}, to: {}, attacker: {}",
                            m & FROM_MASK,
                            (m & TO_MASK) >> TO_SHIFT,
                            moving_piece(*m),
                        )
                    }
                    _ => unreachable!("piece: {enemy}, color {color}"),
                }
            };
        previous_move.moved_piece |= enemy << CAPTURED_PIECE_TYPE_SHIFT;
        let capture: u64 = !BIT_MASKS[to_sq as usize];
        *occupancy &= capture;
        *bitboard_for_capture &= capture;
        *current_hash ^= ZOBRIST_HASH_TABLE[(to_sq + 1) * encoded_capture - 1];
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
        self.reset_bit(rook, rook_from, rook_to, color);

        let total_occupancy: &mut u64 = &mut self.total_occupancy;
        let (start, end): (u64, u64) = (!BIT_MASKS[rook_from], BIT_MASKS[rook_to]);
        *total_occupancy &= start;
        *total_occupancy |= end;
        let (occupancy, (rook_from_heuristic, rook_to_heuristic)) = match color {
            8 => (
                &mut self.white_occupancy,
                (
                    -WHITE_ROOK_HEURISTICS[rook_from],
                    WHITE_ROOK_HEURISTICS[rook_to],
                ),
            ),
            16 => (
                &mut self.black_occupancy,
                (
                    BLACK_ROOK_HEURISTICS[rook_from],
                    -BLACK_ROOK_HEURISTICS[rook_to],
                ),
            ),
            _ => unreachable!(),
        };
        *occupancy &= start;
        *occupancy |= end;
        *eval += rook_from_heuristic + rook_to_heuristic;

        let rook_encoding: usize = if color == 8 {
            rook as usize - 8
        } else {
            rook as usize - 10
        };
        *current_hash ^= ZOBRIST_HASH_TABLE[(rook_from + 1) * rook_encoding - 1];
        *current_hash ^= ZOBRIST_HASH_TABLE[(rook_to + 1) * rook_encoding - 1];
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
                &mut self.black_pawns,
                &mut self.black_occupancy,
                e_p - 8,
                PAWN_VALUE,
            ),
            16 => (
                &mut self.white_pawns,
                &mut self.white_occupancy,
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
        *&mut self.total_occupancy &= capture;
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

        let moving_piece: u32 = moving_piece(piece_move);
        let captured_piece: u32 = captured_piece(piece_move);

        let moving_piece_encoding: usize = if color == 8 {
            moving_piece as usize - 8
        } else {
            moving_piece as usize - 10
        };
        let to_sq_index_base_one: usize = to_sq_index + 1;

        *current_hash ^= ZOBRIST_HASH_TABLE[(from_sq_index + 1) * moving_piece_encoding - 1];
        *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one * moving_piece_encoding - 1];

        let e_p: u8 = en_passant(piece_move);
        let castling: u8 = castling(piece_move);

        self.cached_pieces[to_sq as usize] = moving_piece;
        self.cached_pieces[from_sq as usize] = 0;

        let promotion_choice: u8 = promotion(piece_move);
        let mut previous_move: PreviousMove = PreviousMove {
            moved_piece: piece_move,
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: None,
            material_difference: 0,
        };
        if captured_piece != 0 {
            self.perform_capture(
                state,
                captured_piece,
                &mut previous_move,
                to_sq_index,
                color,
                &piece_move,
                evaluation,
                current_hash,
            );
        }
        if castling != 0 {
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
            *current_hash ^= ZOBRIST_HASH_TABLE[if color == 8 { 16 } else { 8 }
                * (state.en_passant_target.unwrap() + 1) as usize
                - 1];
        }
        let (occupancy, (moving_piece_bb, from_heuristic, to_heuristic)) = if color == 8 {
            (
                &mut self.white_occupancy,
                match moving_piece {
                    WHITE_BISHOP_U32 => (
                        &mut self.white_bishops,
                        -WHITE_BISHOP_HEURISTICS[from_sq_index],
                        WHITE_BISHOP_HEURISTICS[to_sq_index],
                    ),
                    WHITE_KNIGHT_U32 => (
                        &mut self.white_knights,
                        -WHITE_KNIGHT_HEURISTICS[from_sq_index],
                        WHITE_KNIGHT_HEURISTICS[to_sq_index],
                    ),
                    WHITE_PAWN_U32 => (
                        &mut self.white_pawns,
                        -WHITE_PAWN_HEURISTICS[from_sq_index],
                        WHITE_PAWN_HEURISTICS[to_sq_index],
                    ),
                    WHITE_QUEEN_U32 => (
                        &mut self.white_queens,
                        -WHITE_QUEEN_HEURISTICS[from_sq_index],
                        WHITE_QUEEN_HEURISTICS[to_sq_index],
                    ),
                    WHITE_ROOK_U32 => {
                        previous_move.previous_castling_rights = Some(state.castling_rights);
                        if from_sq == 0 {
                            state.castling_rights &= !WHITE_LONG_MASK;
                        } else if from_sq == 7 {
                            state.castling_rights &= !WHITE_SHORT_MASK;
                        }
                        (
                            &mut self.white_rooks,
                            -WHITE_ROOK_HEURISTICS[from_sq_index],
                            WHITE_ROOK_HEURISTICS[to_sq_index],
                        )
                    }
                    WHITE_KING_U32 => {
                        previous_move.previous_castling_rights = Some(state.castling_rights);
                        self.white_king_square = to_sq as u8;
                        state.castling_rights &= !(WHITE_LONG_MASK | WHITE_SHORT_MASK);
                        (
                            &mut self.white_king,
                            -WHITE_KING_HEURISTICS[from_sq_index],
                            WHITE_KING_HEURISTICS[to_sq_index],
                        )
                    }
                    _ => unreachable!(),
                },
            )
        } else {
            (
                &mut self.black_occupancy,
                match moving_piece {
                    BLACK_BISHOP_U32 => (
                        &mut self.black_bishops,
                        BLACK_BISHOP_HEURISTICS[from_sq_index],
                        -BLACK_BISHOP_HEURISTICS[to_sq_index],
                    ),
                    BLACK_KNIGHT_U32 => (
                        &mut self.black_knights,
                        BLACK_KNIGHT_HEURISTICS[from_sq_index],
                        -BLACK_KNIGHT_HEURISTICS[to_sq_index],
                    ),
                    BLACK_PAWN_U32 => (
                        &mut self.black_pawns,
                        BLACK_PAWN_HEURISTICS[from_sq_index],
                        -BLACK_PAWN_HEURISTICS[to_sq_index],
                    ),
                    BLACK_QUEEN_U32 => (
                        &mut self.black_queens,
                        BLACK_QUEEN_HEURISTICS[from_sq_index],
                        -BLACK_QUEEN_HEURISTICS[to_sq_index],
                    ),
                    BLACK_ROOK_U32 => {
                        previous_move.previous_castling_rights = Some(state.castling_rights);
                        if from_sq == 56 {
                            state.castling_rights &= !BLACK_LONG_MASK;
                        } else if from_sq == 63 {
                            state.castling_rights &= !BLACK_SHORT_MASK;
                        }
                        (
                            &mut self.black_rooks,
                            BLACK_ROOK_HEURISTICS[from_sq_index],
                            -BLACK_ROOK_HEURISTICS[to_sq_index],
                        )
                    }
                    BLACK_KING_U32 => {
                        previous_move.previous_castling_rights = Some(state.castling_rights);
                        self.black_king_square = to_sq as u8;
                        state.castling_rights &= !(BLACK_LONG_MASK | BLACK_SHORT_MASK);
                        (
                            &mut self.black_king,
                            BLACK_KING_HEURISTICS[from_sq_index],
                            -BLACK_KING_HEURISTICS[to_sq_index],
                        )
                    }
                    _ => unreachable!(),
                },
            )
        };

        let (start, end): (u64, u64) = (!BIT_MASKS[from_sq_index], BIT_MASKS[to_sq_index]);
        self.total_occupancy &= start;
        self.total_occupancy |= end;
        *occupancy &= start;
        *occupancy |= end;

        *evaluation += from_heuristic + to_heuristic;

        match promotion_choice {
            0 => {
                *moving_piece_bb |= end;
                *moving_piece_bb &= start;
            }
            4 => {
                if color == 8 {
                    *&mut self.white_pawns &= start;
                    *&mut self.white_queens |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[5 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = WHITE_QUEEN_U32;
                } else {
                    *&mut self.black_pawns &= start;
                    *&mut self.black_queens |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[7 * to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[11 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = BLACK_QUEEN_U32;
                }
            }
            3 => {
                if color == 8 {
                    *&mut self.white_pawns &= start;
                    *&mut self.white_rooks |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[4 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = WHITE_ROOK_U32;
                } else {
                    *&mut self.black_pawns &= start;
                    *&mut self.black_rooks |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[10 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = BLACK_ROOK_U32;
                }
            }
            2 => {
                if color == 8 {
                    *&mut self.white_pawns &= start;
                    *&mut self.white_bishops |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[3 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = WHITE_BISHOP_U32;
                } else {
                    *&mut self.black_pawns &= start;
                    *&mut self.black_bishops |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[9 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = BLACK_BISHOP_U32;
                }
            }
            1 => {
                if color == 8 {
                    *&mut self.white_pawns &= start;
                    *&mut self.white_knights |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[2 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = WHITE_KNIGHT_U32;
                } else {
                    *&mut self.black_pawns &= start;
                    *&mut self.black_knights |= end;
                    *current_hash ^= ZOBRIST_HASH_TABLE[to_sq_index_base_one - 1];
                    *current_hash ^= ZOBRIST_HASH_TABLE[8 * to_sq_index_base_one - 1];
                    self.cached_pieces[to_sq as usize] = BLACK_KNIGHT_U32;
                }
            }
            _ => unreachable!("promotion square is {promotion_choice}, should be 0..=4"),
        };
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
                moving_piece(m),
                promotion(m),
                captured_piece(m),
                castling(m),
                en_passant(m),
            );
            let (start_index, end_index): (usize, usize) = (start as usize, end as usize);
            let end_index_base_one: usize = end_index + 1; // 1..=64

            let main_piece_encoding: usize = if color == 8 {
                main_piece as usize - 8
            } else {
                main_piece as usize - 10
            };
            *current_hash ^= ZOBRIST_HASH_TABLE[(start_index + 1) * main_piece_encoding - 1];

            self.cached_pieces[end_index] = 0;

            let (moved_piece_bitboard, color_occupancy): (&mut u64, &mut u64) = if color == 8 {
                (
                    match main_piece {
                        WHITE_PAWN_U32 => &mut self.white_pawns,
                        WHITE_KNIGHT_U32 => &mut self.white_knights,
                        WHITE_BISHOP_U32 => &mut self.white_bishops,
                        WHITE_QUEEN_U32 => &mut self.white_queens,
                        WHITE_KING_U32 => {
                            self.white_king_square = start;
                            &mut self.white_king
                        }
                        WHITE_ROOK_U32 => &mut self.white_rooks,
                        other => unreachable!("piece {other}, color {color}"),
                    },
                    &mut self.white_occupancy,
                )
            } else {
                (
                    match main_piece {
                        BLACK_PAWN_U32 => &mut self.black_pawns,
                        BLACK_KNIGHT_U32 => &mut self.black_knights,
                        BLACK_BISHOP_U32 => &mut self.black_bishops,
                        BLACK_QUEEN_U32 => &mut self.black_queens,
                        BLACK_KING_U32 => {
                            self.black_king_square = start;
                            &mut self.black_king
                        }
                        BLACK_ROOK_U32 => &mut self.black_rooks,
                        other => unreachable!("piece {other}",),
                    },
                    &mut self.black_occupancy,
                )
            };
            let start_bb: u64 = BIT_MASKS[start_index];
            let not_end_bb: u64 = !BIT_MASKS[end_index];
            self.cached_pieces[start_index] = main_piece;
            *moved_piece_bitboard &= not_end_bb;
            if promotion == 0 {
                *moved_piece_bitboard |= start_bb;
                *current_hash ^= ZOBRIST_HASH_TABLE[end_index_base_one * main_piece_encoding - 1];
            } else {
                let (pawns, promoted_piece, pawn, promotion_piece_encoding) =
                    match (color, promotion) {
                        (8, 1) => (
                            &mut self.white_pawns,
                            &mut self.white_knights,
                            WHITE_PAWN_U32,
                            2,
                        ),
                        (8, 2) => (
                            &mut self.white_pawns,
                            &mut self.white_bishops,
                            WHITE_PAWN_U32,
                            3,
                        ),
                        (8, 3) => (
                            &mut self.white_pawns,
                            &mut self.white_rooks,
                            WHITE_PAWN_U32,
                            4,
                        ),
                        (8, 4) => (
                            &mut self.white_pawns,
                            &mut self.white_queens,
                            WHITE_PAWN_U32,
                            5,
                        ),
                        (16, 1) => (
                            &mut self.black_pawns,
                            &mut self.black_knights,
                            BLACK_PAWN_U32,
                            8,
                        ),
                        (16, 2) => (
                            &mut self.black_pawns,
                            &mut self.black_bishops,
                            BLACK_PAWN_U32,
                            9,
                        ),
                        (16, 3) => (
                            &mut self.black_pawns,
                            &mut self.black_rooks,
                            BLACK_PAWN_U32,
                            10,
                        ),
                        (16, 4) => (
                            &mut self.black_pawns,
                            &mut self.black_queens,
                            BLACK_PAWN_U32,
                            11,
                        ),
                        _ => unreachable!(),
                    };
                *pawns |= start_bb;
                *promoted_piece &= not_end_bb;
                self.cached_pieces[start_index] = pawn;
                *current_hash ^=
                    ZOBRIST_HASH_TABLE[promotion_piece_encoding * end_index_base_one - 1];
            }

            *color_occupancy |= start_bb;
            *color_occupancy &= not_end_bb;
            self.total_occupancy |= start_bb;
            if captured_piece != 0 {
                let end_bb: u64 = BIT_MASKS[end_index];
                *if color == 8 {
                    self.black_occupancy |= end_bb;
                    *current_hash ^=
                        ZOBRIST_HASH_TABLE[(captured_piece as usize - 10) * end_index_base_one - 1];
                    match captured_piece {
                        BLACK_PAWN_U32 => &mut self.black_pawns,
                        BLACK_KNIGHT_U32 => &mut self.black_knights,
                        BLACK_BISHOP_U32 => &mut self.black_bishops,
                        BLACK_QUEEN_U32 => &mut self.black_queens,
                        BLACK_ROOK_U32 => &mut self.black_rooks,
                        _ => unreachable!(),
                    }
                } else {
                    self.white_occupancy |= end_bb;
                    *current_hash ^=
                        ZOBRIST_HASH_TABLE[(captured_piece as usize - 8) * end_index_base_one - 1];
                    match captured_piece {
                        WHITE_PAWN_U32 => &mut self.white_pawns,
                        WHITE_KNIGHT_U32 => &mut self.white_knights,
                        WHITE_BISHOP_U32 => &mut self.white_bishops,
                        WHITE_QUEEN_U32 => &mut self.white_queens,
                        WHITE_ROOK_U32 => &mut self.white_rooks,
                        _ => unreachable!(),
                    }
                } |= end_bb;
                self.cached_pieces[end_index] = captured_piece;

                self.total_occupancy |= end_bb;
            } else {
                self.cached_pieces[end_index] = 0;
                self.total_occupancy &= not_end_bb;
            }

            if en_passant != 0 {
                let (pawn, enemy_pawns, enemy_occupancy, taken_pawn_square): (
                    u32,
                    &mut u64,
                    &mut u64,
                    usize,
                ) = match color {
                    8 => {
                        *current_hash ^= ZOBRIST_HASH_TABLE[end_index_base_one - 9];
                        (
                            BLACK_PAWN_U32,
                            &mut self.black_pawns,
                            &mut self.black_occupancy,
                            end_index - 8,
                        )
                    }
                    16 => {
                        *current_hash ^= ZOBRIST_HASH_TABLE[7 * end_index_base_one + 7];
                        (
                            WHITE_PAWN_U32,
                            &mut self.white_pawns,
                            &mut self.white_occupancy,
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
                    (4, 2) => (&mut self.white_rooks, &mut self.white_occupancy, 0, 3, 5),
                    (4, 6) => (&mut self.white_rooks, &mut self.white_occupancy, 7, 5, 5),
                    (60, 58) => (
                        &mut self.black_rooks,
                        &mut self.black_occupancy,
                        56 as usize,
                        59 as usize,
                        10,
                    ),
                    (60, 62) => (
                        &mut self.black_rooks,
                        &mut self.black_occupancy,
                        63 as usize,
                        61 as usize,
                        10,
                    ),
                    _ => unreachable!("from: {start}, to {end}"),
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

                *current_hash ^= ZOBRIST_HASH_TABLE[rook_encoding * (rook_start + 1) - 1];
                *current_hash ^= ZOBRIST_HASH_TABLE[rook_encoding * (rook_end + 1) - 1];
            }

            if let Some(castling_rights) = previous_move.previous_castling_rights {
                state.castling_rights = castling_rights;
            }
            state.en_passant_target = previous_move.previous_en_passant;
        }
    }
}
