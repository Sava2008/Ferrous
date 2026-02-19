use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::heuristics::*,
    enums::{PieceColor, PieceType},
    gamestate::PieceMove,
};

impl Engine {
    pub fn move_priority(&self, board: &Board, m: &PieceMove, depth: usize) -> u16 {
        let mut priority_key: u16 = 0;
        let (initial_pos, final_pos): ((PieceColor, PieceType), Option<(PieceColor, PieceType)>) =
            (board.piece_at(m.from).unwrap(), board.piece_at(m.to));
        if let Some(dest) = final_pos {
            let victim_value: u16 = Self::get_piece_value(dest.1) as u16;
            let attacker_value: u16 = Self::get_piece_value(initial_pos.1) as u16;
            priority_key += (victim_value * 6 + (5 - attacker_value)) as u16;
        }
        if self.killer_moves[depth][0] == Some(*m) || self.killer_moves[depth][1] == Some(*m) {
            priority_key += 10;
        }
        if Self::does_improve_piece(&initial_pos, &m) {
            priority_key += 1;
        }
        return priority_key;
    }
    fn get_piece_value(piece_type: PieceType) -> u8 {
        match piece_type {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }

    fn does_improve_piece(piece: &(PieceColor, PieceType), m: &PieceMove) -> bool {
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
}
