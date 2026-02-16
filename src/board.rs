use crate::{
    board_geometry_templates::*,
    constants::heuristics::*,
    enums::{InclusiveRange, PieceColor, PieceType},
    gamestate::{GameState, PieceMove, PreviousMove},
};
use std::cmp::{max, min};
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
        };
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

    #[inline]
    pub fn bitboard_contains(&self, index: u8) -> Option<(PieceColor, PieceType)> {
        let mask: Bitboard = 1 << index;
        if &self.total_occupancy & mask == 0 {
            return None;
        }

        if &self.white_pawns & mask != 0 {
            return Some((PieceColor::White, PieceType::Pawn));
        }
        if &self.white_knights & mask != 0 {
            return Some((PieceColor::White, PieceType::Knight));
        }
        if &self.white_bishops & mask != 0 {
            return Some((PieceColor::White, PieceType::Bishop));
        }
        if &self.white_rooks & mask != 0 {
            return Some((PieceColor::White, PieceType::Rook));
        }
        if &self.white_queens & mask != 0 {
            return Some((PieceColor::White, PieceType::Queen));
        }
        if &self.white_king & mask != 0 {
            return Some((PieceColor::White, PieceType::King));
        }

        if &self.black_pawns & mask != 0 {
            return Some((PieceColor::Black, PieceType::Pawn));
        }
        if &self.black_knights & mask != 0 {
            return Some((PieceColor::Black, PieceType::Knight));
        }
        if &self.black_bishops & mask != 0 {
            return Some((PieceColor::Black, PieceType::Bishop));
        }
        if &self.black_rooks & mask != 0 {
            return Some((PieceColor::Black, PieceType::Rook));
        }
        if &self.black_queens & mask != 0 {
            return Some((PieceColor::Black, PieceType::Queen));
        }
        if &self.black_king & mask != 0 {
            return Some((PieceColor::Black, PieceType::King));
        }

        return None;
    }

    // performs verified moves, so there is no need for another verification
    pub fn perform_move(&mut self, from_to: &PieceMove, state: &mut GameState) -> () {
        self.total_occupancy();
        let mut previous_move: PreviousMove = PreviousMove {
            changed_bitboards: [(None, None); 3],
            previous_en_passant: state.en_passant_target,
            previous_castling_rights: None,
            previous_check_info: state.check_info.clone(),
            previous_pin_info: state.pin_info.clone(),
            previous_check_constraints: if state.check_info.checked_king.is_some() {
                state.check_contraints
            } else {
                0
            },
        };

        if [2, 6, 58, 62].iter().any(|sq: &u8| *sq == from_to.to) {
            state.en_passant_target = None;
            let (white_king, black_king): (u8, u8) = (
                self.white_king.trailing_zeros() as u8,
                self.black_king.trailing_zeros() as u8,
            );
            match from_to.from {
                sq if sq == white_king => {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                    previous_move.changed_bitboards[0] = (
                        Some((PieceColor::White, PieceType::King)),
                        Some(self.white_king),
                    );
                    previous_move.changed_bitboards[1] = (
                        Some((PieceColor::White, PieceType::Rook)),
                        Some(self.white_rooks),
                    );

                    let rook_from_to: (u8, u8) = match from_to.to {
                        2 => (0, 3),
                        6 => (7, 5),
                        _ => unreachable!(),
                    };
                    self.reset_bit(
                        (PieceColor::White, PieceType::King),
                        from_to.from,
                        from_to.to,
                    );
                    self.reset_bit(
                        (PieceColor::White, PieceType::Rook),
                        rook_from_to.0,
                        rook_from_to.1,
                    );
                    (
                        state.castling_rights.white_three_zeros,
                        state.castling_rights.white_two_zeros,
                    ) = (false, false);
                    state.moves_history.push(previous_move);
                    return;
                }
                sq if sq == black_king => {
                    previous_move.previous_castling_rights = Some(state.castling_rights.clone());
                    previous_move.changed_bitboards[0] = (
                        Some((PieceColor::Black, PieceType::King)),
                        Some(self.black_king),
                    );
                    previous_move.changed_bitboards[1] = (
                        Some((PieceColor::Black, PieceType::Rook)),
                        Some(self.black_rooks),
                    );

                    let rook_from_to: (u8, u8) = match from_to.to {
                        58 => (56, 59),
                        62 => (63, 61),
                        _ => unreachable!(),
                    };
                    self.reset_bit(
                        (PieceColor::Black, PieceType::King),
                        from_to.from,
                        from_to.to,
                    );
                    self.reset_bit(
                        (PieceColor::Black, PieceType::Rook),
                        rook_from_to.0,
                        rook_from_to.1,
                    );
                    (
                        state.castling_rights.black_three_zeros,
                        state.castling_rights.black_two_zeros,
                    ) = (false, false);
                    state.moves_history.push(previous_move);
                    return;
                }
                _ => (),
            };
        }
        if let Some(enemy) = self.bitboard_contains(from_to.to) {
            let bitboard_for_capture: &mut Bitboard = match enemy {
                (PieceColor::White, PieceType::Bishop) => &mut self.white_bishops,
                (PieceColor::White, PieceType::Knight) => &mut self.white_knights,
                (PieceColor::White, PieceType::Pawn) => &mut self.white_pawns,
                (PieceColor::White, PieceType::Queen) => &mut self.white_queens,
                (PieceColor::White, PieceType::Rook) => {
                    if from_to.to == 7 {
                        state.castling_rights.white_two_zeros = false;
                    } else if from_to.to == 0 {
                        state.castling_rights.white_three_zeros = false;
                    }
                    &mut self.white_rooks
                }
                (PieceColor::White, PieceType::King) => {
                    panic!("attemped to capture white king. state: {state:?}, board: {self:?}")
                }
                (PieceColor::Black, PieceType::Bishop) => &mut self.black_bishops,
                (PieceColor::Black, PieceType::Knight) => &mut self.black_knights,
                (PieceColor::Black, PieceType::Pawn) => &mut self.black_pawns,
                (PieceColor::Black, PieceType::Queen) => &mut self.black_queens,
                (PieceColor::Black, PieceType::Rook) => {
                    if from_to.to == 63 {
                        state.castling_rights.black_two_zeros = false;
                    } else if from_to.to == 56 {
                        state.castling_rights.black_three_zeros = false;
                    }
                    &mut self.black_rooks
                }
                (PieceColor::Black, PieceType::King) => {
                    panic!("attemped to capture black king. state: {state:?}, board: {self:?}")
                }
            };

            previous_move.changed_bitboards[1] = (Some(enemy), Some(bitboard_for_capture.clone()));

            *bitboard_for_capture &= !(1 << from_to.to);
        }

        if let Some(piece) = self.bitboard_contains(from_to.from) {
            match piece.0 {
                PieceColor::White => match piece.1 {
                    PieceType::King => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.white_king));
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        (
                            state.castling_rights.white_three_zeros,
                            state.castling_rights.white_two_zeros,
                        ) = (false, false);
                        state.en_passant_target = None;
                    }
                    PieceType::Rook => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.white_rooks));
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        if from_to.from == 0 {
                            state.castling_rights.white_three_zeros = false;
                        } else if from_to.from == 7 {
                            state.castling_rights.white_two_zeros = false;
                        }
                        state.en_passant_target = None;
                    }
                    PieceType::Pawn => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.white_pawns));
                        match from_to {
                            &PieceMove { from, to }
                                if (8..=15).contains(&from) && (24..=31).contains(&to) =>
                            {
                                state.en_passant_target = Some(from_to.to - 8); // en passant square behind the pawn
                            }
                            _ => state.en_passant_target = None,
                        };
                        if let Some(e_p) = state.en_passant_target {
                            if from_to.to == e_p {
                                let black_pawns: &mut Bitboard = &mut self.black_pawns;
                                *black_pawns &= !(1 << (e_p - 8));
                                state.en_passant_target = None;
                            }
                        }
                    }
                    PieceType::Bishop => {
                        previous_move.changed_bitboards[0] =
                            (Some(piece), Some(self.white_bishops));
                        state.en_passant_target = None;
                    }
                    PieceType::Knight => {
                        previous_move.changed_bitboards[0] =
                            (Some(piece), Some(self.white_knights));
                        state.en_passant_target = None;
                    }
                    PieceType::Queen => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.white_queens));
                        state.en_passant_target = None;
                    }
                },
                PieceColor::Black => match piece.1 {
                    PieceType::King => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.black_king));
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        (
                            state.castling_rights.black_three_zeros,
                            state.castling_rights.black_two_zeros,
                        ) = (false, false);
                        state.en_passant_target = None;
                    }
                    PieceType::Rook => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.black_rooks));
                        previous_move.previous_castling_rights =
                            Some(state.castling_rights.clone());

                        if from_to.from == 56 {
                            state.castling_rights.black_three_zeros = false;
                        } else if from_to.from == 63 {
                            state.castling_rights.black_two_zeros = false;
                        }
                        state.en_passant_target = None;
                    }
                    PieceType::Pawn => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.black_pawns));

                        match from_to {
                            &PieceMove { from, to }
                                if (48..=55).contains(&from) && (32..=39).contains(&to) =>
                            {
                                state.en_passant_target = Some(from_to.to + 8); // en passant square behind the pawn
                            }
                            _ => state.en_passant_target = None,
                        };
                        if let Some(e_p) = state.en_passant_target {
                            if from_to.to == e_p {
                                let white_pawns: &mut Bitboard = &mut self.white_pawns;
                                *white_pawns &= !(1 << (e_p + 8));
                                state.en_passant_target = None
                            }
                        }
                    }
                    PieceType::Bishop => {
                        previous_move.changed_bitboards[0] =
                            (Some(piece), Some(self.black_bishops));
                        state.en_passant_target = None;
                    }
                    PieceType::Knight => {
                        previous_move.changed_bitboards[0] =
                            (Some(piece), Some(self.black_knights));
                        state.en_passant_target = None;
                    }
                    PieceType::Queen => {
                        previous_move.changed_bitboards[0] = (Some(piece), Some(self.black_queens));
                        state.en_passant_target = None;
                    }
                },
            };
            self.reset_bit(piece, from_to.from, from_to.to);
        }
        state.moves_history.push(previous_move);
    }

    pub fn cancel_move(&mut self, state: &mut GameState) -> () {
        if let Some(previous_move) = state.moves_history.pop() {
            for (piece, bb) in previous_move.changed_bitboards {
                if let Some(p) = piece {
                    match p.1 {
                        PieceType::Bishop => match p.0 {
                            PieceColor::White => self.white_bishops = bb.unwrap(),
                            PieceColor::Black => self.black_bishops = bb.unwrap(),
                        },
                        PieceType::Knight => match p.0 {
                            PieceColor::White => self.white_knights = bb.unwrap(),
                            PieceColor::Black => self.black_knights = bb.unwrap(),
                        },
                        PieceType::Pawn => match p.0 {
                            PieceColor::White => self.white_pawns = bb.unwrap(),
                            PieceColor::Black => self.black_pawns = bb.unwrap(),
                        },
                        PieceType::Queen => match p.0 {
                            PieceColor::White => self.white_queens = bb.unwrap(),
                            PieceColor::Black => self.black_queens = bb.unwrap(),
                        },
                        PieceType::Rook => match p.0 {
                            PieceColor::White => self.white_rooks = bb.unwrap(),
                            PieceColor::Black => self.black_rooks = bb.unwrap(),
                        },
                        PieceType::King => match p.0 {
                            PieceColor::White => self.white_king = bb.unwrap(),
                            PieceColor::Black => self.black_king = bb.unwrap(),
                        },
                    };
                } else {
                    break;
                }
            }
            if let Some(castling_rights) = previous_move.previous_castling_rights {
                state.castling_rights = castling_rights;
            }
            self.total_occupancy();
            state.en_passant_target = previous_move.previous_en_passant;
            state.check_info = previous_move.previous_check_info;
            state.pin_info = previous_move.previous_pin_info;
            state.check_contraints = previous_move.previous_check_constraints;
        }
    }

    pub fn generate_range(square1: u8, square2: u8, inclusion: &InclusiveRange) -> Bitboard {
        let (mut lower_square, higher_square) = (min(square1, square2), max(square1, square2));
        let mut rng: Bitboard = 0;
        let sq1_sq2_range: u8 = higher_square - lower_square;
        let increment: u8 = match sq1_sq2_range {
            num if num % 9 == 0 => 9,
            num if num % 7 == 0 && ((1 << num) & FILE_A) == 0 => 7,
            num if num <= 7 => 1,
            num if num % 8 == 0 => 8,
            _ => panic!("no straight path between {square1} and {square2}"),
        };
        while lower_square < higher_square - increment {
            lower_square += increment;
            rng |= 1 << lower_square;
        }
        match inclusion {
            &InclusiveRange::Both => rng |= 1 << square1 | 1 << square2,
            &InclusiveRange::FirstOnly => rng |= 1 << square1,
            &InclusiveRange::LastOnly => rng |= 1 << square2,
            &InclusiveRange::None => (),
        }
        return rng;
    }

    pub fn is_capture(&self, m: &PieceMove) -> bool {
        return self.bitboard_contains(m.to).is_some();
    }

    pub fn does_improve_piece(&self, m: &PieceMove) -> bool {
        let piece: (PieceColor, PieceType) = self.bitboard_contains(m.from).unwrap();
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
