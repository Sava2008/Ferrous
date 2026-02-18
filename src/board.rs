use crate::{
    board_geometry_templates::*,
    constants::{
        attacks::RAYS_BETWEEN,
        heuristics::*,
        piece_values::{BISHOP_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE},
    },
    enums::{InclusiveRange, PieceColor, PieceType},
    gamestate::{GameState, PieceMove, PreviousMove},
};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_rooks: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_rooks: Bitboard,
    pub black_king: Bitboard,

    pub white_occupancy: Bitboard,
    pub black_occupancy: Bitboard,
    pub total_occupancy: Bitboard,

    pub cached_pieces: [Option<(PieceColor, PieceType)>; 64],

    pub material: i32,
}

impl Board {
    // default starting position
    pub fn set() -> Self {
        return Board {
            white_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            white_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            white_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            white_queens: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            white_king: 0b0000000000000000000000000000000000000000000000000000000000010000,
            black_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            black_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            black_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            black_queens: 0b0000100000000000000000000000000000000000000000000000000000000000,
            black_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
            white_occupancy: 0,
            black_occupancy: 0,
            total_occupancy: 0,
            cached_pieces: [None; 64],
            material: 0, // equal material
        };
    }
    pub fn count_material(&mut self) -> () {
        self.material = (self.white_pawns.count_ones() as i32) * PAWN_VALUE
            + (self.white_knights.count_ones() as i32) * KNIGHT_VALUE
            + (self.white_bishops.count_ones() as i32) * BISHOP_VALUE
            + (self.white_rooks.count_ones() as i32) * ROOK_VALUE
            + (self.white_queens.count_ones() as i32) * QUEEN_VALUE
            - (self.black_pawns.count_ones() as i32) * PAWN_VALUE
            - (self.black_knights.count_ones() as i32) * KNIGHT_VALUE
            - (self.black_bishops.count_ones() as i32) * BISHOP_VALUE
            - (self.black_rooks.count_ones() as i32) * ROOK_VALUE
            - (self.black_queens.count_ones() as i32) * QUEEN_VALUE;
    }
    pub fn update_full_cache(&mut self) {
        for square in 0..64 {
            let mask: Bitboard = 1 << square;
            if self.white_pawns & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::Pawn));
            } else if self.white_knights & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::Knight));
            } else if self.white_bishops & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::Bishop));
            } else if self.white_rooks & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::Rook));
            } else if self.white_queens & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::Queen));
            } else if self.white_king & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::White, PieceType::King));
            } else if self.black_pawns & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::Pawn));
            } else if self.black_knights & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::Knight));
            } else if self.black_bishops & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::Bishop));
            } else if self.black_rooks & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::Rook));
            } else if self.black_queens & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::Queen));
            } else if self.black_king & mask != 0 {
                self.cached_pieces[square] = Some((PieceColor::Black, PieceType::King));
            }
        }
    }

    #[inline(always)]
    pub fn piece_at(&self, square: u8) -> Option<(PieceColor, PieceType)> {
        return unsafe { *self.cached_pieces.get_unchecked(square as usize) }; // square always in range 0-63
    }

    pub fn white_occupancy(&mut self) -> () {
        self.white_occupancy = self.white_bishops
            | self.white_king
            | self.white_knights
            | self.white_pawns
            | self.white_queens
            | self.white_rooks;
    }

    pub fn black_occupancy(&mut self) -> () {
        self.black_occupancy = self.black_bishops
            | self.black_king
            | self.black_knights
            | self.black_pawns
            | self.black_queens
            | self.black_rooks;
    }

    pub fn total_occupancy(&mut self) -> () {
        self.white_occupancy();
        self.black_occupancy();
        self.total_occupancy = self.white_occupancy | self.black_occupancy;
    }

    pub fn bitboard_to_indices(mut bitboard: Bitboard) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        while bitboard != 0 {
            let index: usize = bitboard.trailing_zeros() as usize;
            indices.push(index);
            bitboard &= bitboard - 1;
        }
        return indices;
    }

    pub fn indices_to_bitboard(indices: &Vec<usize>) -> Bitboard {
        let mut bitboard: Bitboard = 0;
        for index in indices {
            if *index > 63 {
                panic!(
                    "can annotate a chess board only with indices from 0 (inclusive) to 63 (inclusive)"
                );
            }
            bitboard |= 1 << index;
        }
        return bitboard;
    }

    #[inline]
    fn reset_bit(
        &mut self,
        bitboard: (PieceColor, PieceType),
        bit_position1: u8,
        bit_position2: u8,
    ) -> () {
        let from_mask: Bitboard = !(1 << bit_position1);
        let to_mask: Bitboard = 1 << bit_position2;
        let bitboard_to_mutate: &mut Bitboard = match bitboard {
            (PieceColor::White, PieceType::Bishop) => &mut self.white_bishops,
            (PieceColor::White, PieceType::Knight) => &mut self.white_knights,
            (PieceColor::White, PieceType::Pawn) => &mut self.white_pawns,
            (PieceColor::White, PieceType::Rook) => &mut self.white_rooks,
            (PieceColor::White, PieceType::Queen) => &mut self.white_queens,
            (PieceColor::White, PieceType::King) => &mut self.white_king,
            (PieceColor::Black, PieceType::Bishop) => &mut self.black_bishops,
            (PieceColor::Black, PieceType::Knight) => &mut self.black_knights,
            (PieceColor::Black, PieceType::Pawn) => &mut self.black_pawns,
            (PieceColor::Black, PieceType::Rook) => &mut self.black_rooks,
            (PieceColor::Black, PieceType::Queen) => &mut self.black_queens,
            (PieceColor::Black, PieceType::King) => &mut self.black_king,
        };

        *bitboard_to_mutate &= from_mask;
        *bitboard_to_mutate |= to_mask;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(&mut self, from_to: &PieceMove, state: &mut GameState) -> () {
        let moving_piece: (PieceColor, PieceType) = self.piece_at(from_to.from).unwrap();
        let captured_piece: Option<(PieceColor, PieceType)> = self.piece_at(from_to.to);
        let mut previous_move: PreviousMove = PreviousMove {
            changed_cache_indices: [(None, None); 3],
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: None,
            previous_check_info: state.check_info.clone(),
            previous_pin_info: state.pin_info.clone(),
            previous_check_constraints: if state.check_info.checked_king.is_some() {
                state.check_contraints
            } else {
                0
            },
            material_difference: self.material,
        };
        if let Some(enemy) = captured_piece {
            let (bitboard_for_capture, occupancy): (&mut Bitboard, &mut Bitboard) = match enemy {
                (PieceColor::White, PieceType::Bishop) => {
                    previous_move.material_difference += BISHOP_VALUE;
                    self.material -= BISHOP_VALUE;
                    (&mut self.white_bishops, &mut self.white_occupancy)
                }
                (PieceColor::White, PieceType::Knight) => {
                    previous_move.material_difference += KNIGHT_VALUE;
                    self.material -= KNIGHT_VALUE;
                    (&mut self.white_knights, &mut self.white_occupancy)
                }
                (PieceColor::White, PieceType::Pawn) => {
                    previous_move.material_difference += PAWN_VALUE;
                    self.material -= PAWN_VALUE;
                    (&mut self.white_pawns, &mut self.white_occupancy)
                }
                (PieceColor::White, PieceType::Queen) => {
                    previous_move.material_difference += QUEEN_VALUE;
                    self.material -= QUEEN_VALUE;
                    (&mut self.white_queens, &mut self.white_occupancy)
                }
                (PieceColor::White, PieceType::Rook) => {
                    previous_move.material_difference += ROOK_VALUE;
                    self.material -= ROOK_VALUE;
                    if from_to.to == 7 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.white_two_zeros = false;
                    } else if from_to.to == 0 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.white_three_zeros = false;
                    }
                    (&mut self.white_rooks, &mut self.white_occupancy)
                }
                (PieceColor::White, PieceType::King) => {
                    panic!("attemped to capture white king. state: {state:?}, board: {self:?}")
                }
                (PieceColor::Black, PieceType::Bishop) => {
                    previous_move.material_difference -= BISHOP_VALUE;
                    self.material += BISHOP_VALUE;
                    (&mut self.black_bishops, &mut self.black_occupancy)
                }
                (PieceColor::Black, PieceType::Knight) => {
                    previous_move.material_difference -= KNIGHT_VALUE;
                    self.material += KNIGHT_VALUE;
                    (&mut self.black_knights, &mut self.black_occupancy)
                }
                (PieceColor::Black, PieceType::Pawn) => {
                    previous_move.material_difference -= PAWN_VALUE;
                    self.material += PAWN_VALUE;
                    (&mut self.black_pawns, &mut self.black_occupancy)
                }
                (PieceColor::Black, PieceType::Queen) => {
                    previous_move.material_difference -= QUEEN_VALUE;
                    self.material += QUEEN_VALUE;
                    (&mut self.black_queens, &mut self.black_occupancy)
                }
                (PieceColor::Black, PieceType::Rook) => {
                    previous_move.material_difference -= ROOK_VALUE;
                    self.material += ROOK_VALUE;
                    if from_to.to == 63 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.black_two_zeros = false;
                    } else if from_to.to == 56 {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        state.castling_rights.black_three_zeros = false;
                    }
                    (&mut self.black_rooks, &mut self.black_occupancy)
                }
                (PieceColor::Black, PieceType::King) => {
                    panic!("attemped to capture black king. state: {state:?}, board: {self:?}")
                }
            };
            previous_move.changed_cache_indices[1] = (Some((from_to.to, from_to.to)), Some(enemy));
            let capture: Bitboard = !(1 << from_to.to);
            *occupancy &= capture;
            *bitboard_for_capture &= capture;
        }

        if let Some(piece) = self.piece_at(from_to.from) {
            let occupancy: &mut u64 = match piece.0 {
                PieceColor::White => match piece.1 {
                    PieceType::King => {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        match (from_to.from, from_to.to) {
                            (4, 2) => {
                                previous_move.changed_cache_indices[1] =
                                    (Some((0, 3)), Some((PieceColor::White, PieceType::Rook)));
                                self.cached_pieces[0] = None;
                                self.cached_pieces[3] = Some((PieceColor::White, PieceType::Rook));
                                self.reset_bit((PieceColor::White, PieceType::Rook), 0, 3);
                                let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
                                let (start, end): (Bitboard, Bitboard) = (!(1 << 0), 1 << 3);
                                *total_occupancy &= start;
                                *total_occupancy |= end;
                                let occupancy: &mut Bitboard = &mut self.white_occupancy;
                                *occupancy &= start;
                                *occupancy |= end;
                            }
                            (4, 6) => {
                                previous_move.changed_cache_indices[1] =
                                    (Some((7, 5)), Some((PieceColor::White, PieceType::Rook)));
                                self.cached_pieces[7] = None;
                                self.cached_pieces[5] = Some((PieceColor::White, PieceType::Rook));
                                self.reset_bit((PieceColor::White, PieceType::Rook), 7, 5);
                                let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
                                let (start, end): (Bitboard, Bitboard) = (!(1 << 7), 1 << 5);
                                *total_occupancy &= start;
                                *total_occupancy |= end;
                                let occupancy: &mut Bitboard = &mut self.white_occupancy;
                                *occupancy &= start;
                                *occupancy |= end;
                            }
                            _ => (),
                        };

                        (
                            state.castling_rights.white_three_zeros,
                            state.castling_rights.white_two_zeros,
                        ) = (false, false);
                        state.en_passant_target = None;
                        &mut self.white_occupancy
                    }
                    PieceType::Rook => {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        if from_to.from == 0 {
                            state.castling_rights.white_three_zeros = false;
                        } else if from_to.from == 7 {
                            state.castling_rights.white_two_zeros = false;
                        }
                        state.en_passant_target = None;
                        &mut self.white_occupancy
                    }
                    PieceType::Pawn => {
                        match from_to {
                            &PieceMove { from, to }
                                if (8..=15).contains(&from) && (24..=31).contains(&to) =>
                            {
                                state.en_passant_target = Some(from_to.to - 8); // en passant square behind the pawn
                            }
                            _ => {
                                if let Some(e_p) = state.en_passant_target {
                                    if from_to.to == e_p {
                                        let black_pawns: &mut Bitboard = &mut self.black_pawns;
                                        let captured_pawn_square: u8 = e_p - 8;
                                        previous_move.changed_cache_indices[1] = (
                                            Some((captured_pawn_square, captured_pawn_square)),
                                            Some((PieceColor::Black, PieceType::Pawn)),
                                        );
                                        self.cached_pieces[captured_pawn_square as usize] = None;
                                        let capture: Bitboard = !(1 << captured_pawn_square);
                                        *black_pawns &= capture;
                                        *&mut self.total_occupancy &= capture;
                                        *&mut self.black_occupancy &= capture;
                                        previous_move.material_difference += PAWN_VALUE;
                                        self.material += PAWN_VALUE;
                                    }
                                }
                                state.en_passant_target = None;
                            }
                        };
                        &mut self.white_occupancy
                    }
                    _ => {
                        state.en_passant_target = None;
                        &mut self.white_occupancy
                    }
                },
                PieceColor::Black => match piece.1 {
                    PieceType::King => {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        match (from_to.from, from_to.to) {
                            (60, 58) => {
                                previous_move.changed_cache_indices[1] =
                                    (Some((56, 59)), Some((PieceColor::Black, PieceType::Rook)));
                                self.cached_pieces[56] = None;
                                self.cached_pieces[59] = Some((PieceColor::Black, PieceType::Rook));
                                self.reset_bit((PieceColor::Black, PieceType::Rook), 56, 59);
                                let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
                                let (start, end): (Bitboard, Bitboard) = (!(1 << 56), 1 << 59);
                                *total_occupancy &= start;
                                *total_occupancy |= end;
                                let occupancy: &mut Bitboard = &mut self.black_occupancy;
                                *occupancy &= start;
                                *occupancy |= end;
                            }
                            (60, 62) => {
                                previous_move.changed_cache_indices[1] =
                                    (Some((63, 61)), Some((PieceColor::Black, PieceType::Rook)));
                                self.cached_pieces[63] = None;
                                self.cached_pieces[61] = Some((PieceColor::Black, PieceType::Rook));
                                self.reset_bit((PieceColor::Black, PieceType::Rook), 63, 61);
                                let total_occupancy: &mut Bitboard = &mut self.total_occupancy;
                                let (start, end): (Bitboard, Bitboard) = (!(1 << 63), 1 << 61);
                                *total_occupancy &= start;
                                *total_occupancy |= end;
                                let occupancy: &mut Bitboard = &mut self.black_occupancy;
                                *occupancy &= start;
                                *occupancy |= end;
                            }
                            _ => (),
                        };

                        (
                            state.castling_rights.black_three_zeros,
                            state.castling_rights.black_two_zeros,
                        ) = (false, false);
                        state.en_passant_target = None;
                        &mut self.black_occupancy
                    }
                    PieceType::Rook => {
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());
                        if from_to.from == 56 {
                            state.castling_rights.black_three_zeros = false;
                        } else if from_to.from == 63 {
                            state.castling_rights.black_two_zeros = false;
                        }
                        state.en_passant_target = None;
                        &mut self.black_occupancy
                    }
                    PieceType::Pawn => {
                        match from_to {
                            &PieceMove { from, to }
                                if (48..=55).contains(&from) && (32..=39).contains(&to) =>
                            {
                                state.en_passant_target = Some(from_to.to + 8); // en passant square behind the pawn
                            }
                            _ => {
                                if let Some(e_p) = state.en_passant_target {
                                    if from_to.to == e_p {
                                        let white_pawns: &mut Bitboard = &mut self.white_pawns;
                                        let captured_pawn_square: u8 = e_p + 8;
                                        previous_move.changed_cache_indices[1] = (
                                            Some((captured_pawn_square, captured_pawn_square)),
                                            Some((PieceColor::White, PieceType::Pawn)),
                                        );
                                        self.cached_pieces[captured_pawn_square as usize] = None;
                                        let capture: Bitboard = !(1 << captured_pawn_square);
                                        *white_pawns &= capture;
                                        *&mut self.total_occupancy &= capture;
                                        *&mut self.white_occupancy &= capture;
                                        previous_move.material_difference -= PAWN_VALUE;
                                        self.material -= PAWN_VALUE;
                                    }
                                }
                                state.en_passant_target = None;
                            }
                        };
                        &mut self.black_occupancy
                    }
                    _ => {
                        state.en_passant_target = None;
                        &mut self.black_occupancy
                    }
                },
            };
            previous_move.changed_cache_indices[0] =
                (Some((from_to.from, from_to.to)), Some(piece));
            self.cached_pieces[from_to.to as usize] = Some(moving_piece);
            self.cached_pieces[from_to.from as usize] = None;
            let (start, end): (Bitboard, Bitboard) = (!(1 << from_to.from), 1 << from_to.to);
            *&mut self.total_occupancy &= start;
            *&mut self.total_occupancy |= end;
            *occupancy &= start;
            *occupancy |= end;
            self.reset_bit(piece, from_to.from, from_to.to);
        }
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(&mut self, state: &mut GameState) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            self.material -= previous_move.material_difference;
            for (cached_index, piece) in previous_move.changed_cache_indices {
                if let Some(p) = piece {
                    let c_i: (u8, u8) = cached_index.unwrap();
                    let (bitboard_to_restore, occupancy): (&mut u64, &mut u64) = match p.1 {
                        PieceType::Bishop => match p.0 {
                            PieceColor::White => {
                                (&mut self.white_bishops, &mut self.white_occupancy)
                            }
                            PieceColor::Black => {
                                (&mut self.black_bishops, &mut self.black_occupancy)
                            }
                        },
                        PieceType::Knight => match p.0 {
                            PieceColor::White => {
                                (&mut self.white_knights, &mut self.white_occupancy)
                            }
                            PieceColor::Black => {
                                (&mut self.black_knights, &mut self.black_occupancy)
                            }
                        },
                        PieceType::Pawn => match p.0 {
                            PieceColor::White => (&mut self.white_pawns, &mut self.white_occupancy),
                            PieceColor::Black => (&mut self.black_pawns, &mut self.black_occupancy),
                        },
                        PieceType::Queen => match p.0 {
                            PieceColor::White => {
                                (&mut self.white_queens, &mut self.white_occupancy)
                            }
                            PieceColor::Black => {
                                (&mut self.black_queens, &mut self.black_occupancy)
                            }
                        },
                        PieceType::Rook => match p.0 {
                            PieceColor::White => (&mut self.white_rooks, &mut self.white_occupancy),
                            PieceColor::Black => (&mut self.black_rooks, &mut self.black_occupancy),
                        },
                        PieceType::King => match p.0 {
                            PieceColor::White => (&mut self.white_king, &mut self.white_occupancy),
                            PieceColor::Black => (&mut self.black_king, &mut self.black_occupancy),
                        },
                    };
                    self.cached_pieces[c_i.0 as usize] = Some(p);
                    let total: &mut Bitboard = &mut self.total_occupancy;
                    let first_square: Bitboard = 1 << c_i.0;
                    *bitboard_to_restore |= first_square;
                    *total |= first_square;
                    *occupancy |= first_square;
                    if c_i.0 != c_i.1 {
                        self.cached_pieces[c_i.1 as usize] = None;
                        let last_square: Bitboard = !(1 << c_i.1);
                        *total &= last_square;
                        *bitboard_to_restore &= last_square;
                        *occupancy &= last_square;
                    }
                } else {
                    break;
                }
            }
            if let Some(castling_rights) = previous_move.previous_castling_rights {
                state.castling_rights = castling_rights;
            }
            state.en_passant_target = previous_move.previous_en_passant;
            state.check_info = previous_move.previous_check_info;
            state.pin_info = previous_move.previous_pin_info;
            state.check_contraints = previous_move.previous_check_constraints;
        }
    }

    pub fn generate_range(square1: u8, square2: u8, inclusion: &InclusiveRange) -> Bitboard {
        let mut rng: Bitboard = unsafe { RAYS_BETWEEN[square1 as usize][square2 as usize] };

        match inclusion {
            &InclusiveRange::Both => rng |= (1 << square1) | (1 << square2),
            &InclusiveRange::FirstOnly => rng |= 1 << square1,
            &InclusiveRange::LastOnly => rng |= 1 << square2,
            &InclusiveRange::None => (),
        }
        return rng;
    }

    pub fn move_priority(&self, m: &PieceMove) -> u16 {
        let mut priority_key: u16 = 0;
        let (initial_pos, final_pos): ((PieceColor, PieceType), Option<(PieceColor, PieceType)>) =
            (self.piece_at(m.from).unwrap(), self.piece_at(m.to));
        if let Some(dest) = final_pos {
            let victim_value: u16 = self.get_piece_value(dest.1) as u16;
            let attacker_value: u16 = self.get_piece_value(initial_pos.1) as u16;
            priority_key += (victim_value * 6 + (5 - attacker_value)) as u16;
        }
        if self.does_improve_piece(&initial_pos, &m) {
            priority_key += 1;
        }
        return priority_key;
    }
    fn get_piece_value(&self, piece_type: PieceType) -> u8 {
        match piece_type {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }

    fn does_improve_piece(&self, piece: &(PieceColor, PieceType), m: &PieceMove) -> bool {
        match piece.1 {
            PieceType::Bishop => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_BISHOP_HEURISTICS[m.from as usize]
                            > BLACK_BISHOP_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_BISHOP_HEURISTICS[m.from as usize]
                            < WHITE_BISHOP_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
            PieceType::Pawn => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_PAWN_HEURISTICS[m.from as usize]
                            > BLACK_PAWN_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_PAWN_HEURISTICS[m.from as usize]
                            < WHITE_PAWN_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
            PieceType::Knight => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_KNIGHT_HEURISTICS[m.from as usize]
                            > BLACK_KNIGHT_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_KNIGHT_HEURISTICS[m.from as usize]
                            < WHITE_KNIGHT_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
            PieceType::Queen => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_QUEEN_HEURISTICS[m.from as usize]
                            > BLACK_QUEEN_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_QUEEN_HEURISTICS[m.from as usize]
                            < WHITE_QUEEN_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
            PieceType::Rook => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_ROOK_HEURISTICS[m.from as usize]
                            > BLACK_ROOK_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_ROOK_HEURISTICS[m.from as usize]
                            < WHITE_ROOK_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
            PieceType::King => {
                if match piece.0 {
                    PieceColor::Black => {
                        BLACK_KING_HEURISTICS[m.from as usize]
                            > BLACK_KING_HEURISTICS[m.to as usize]
                    }
                    PieceColor::White => {
                        WHITE_KING_HEURISTICS[m.from as usize]
                            < WHITE_KING_HEURISTICS[m.to as usize]
                    }
                } {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn is_king_attacked(&self, color: &PieceColor) -> bool {
        let king_square = match color {
            PieceColor::White => self.white_king.trailing_zeros() as u8,
            PieceColor::Black => self.black_king.trailing_zeros() as u8,
        };
        return self.is_square_attacked(king_square, &!color.clone());
    }
}
