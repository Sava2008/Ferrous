use crate::{
    board::Board,
    board_geometry_templates::{BLACK_LONG, BLACK_SHORT, WHITE_LONG, WHITE_SHORT},
    constants::attacks::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub en_passant_target: Option<u8>, // the square BEHIND the pawn that has moved two squares
    pub castling_rights: u8,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enforces 50-move rule
    pub moves_history: Vec<PreviousMove>,
    pub total_moves_amount: u16,
    pub whose_turn: u16,
    pub irreversible_moves: Vec<u64>,
    pub check_squares: [u64; 5],
}

#[derive(Debug, Clone, PartialEq)]
pub struct CastlingRights {
    pub white_three_zeros: bool,
    pub white_two_zeros: bool,
    pub black_three_zeros: bool,
    pub black_two_zeros: bool,
}
impl CastlingRights {
    #[inline]
    pub fn new() -> Self {
        return Self {
            white_three_zeros: true,
            white_two_zeros: true,
            black_three_zeros: true,
            black_two_zeros: true,
        };
    }
    pub fn to_array(&self) -> [bool; 4] {
        return [
            self.white_three_zeros,
            self.white_two_zeros,
            self.black_three_zeros,
            self.black_two_zeros,
        ];
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreviousMove {
    pub moved_piece: u16,
    pub captured_piece: u16,
    pub move_flag: u16,
    pub previous_en_passant: Option<u8>,
    pub previous_castling_rights: u8,
    pub material_difference: i32,
    pub check_squares: [u64; 5],
}

impl GameState {
    pub fn new(board: &Board) -> Self {
        return Self {
            en_passant_target: None,
            castling_rights: match (board.white_king_square, board.black_king_square) {
                (4, 60) => 15,
                (4, _) => WHITE_SHORT | WHITE_LONG,
                (_, 60) => BLACK_SHORT | BLACK_LONG,
                (_, _) => 0,
            },
            fifty_moves_rule_counter: 1,
            moves_history: Vec::with_capacity(50),
            total_moves_amount: 0,
            whose_turn: 8,
            irreversible_moves: Vec::new(),
            check_squares: [0; 5],
        };
    }

    pub fn is_repetition(&self, current_hash: u64) -> bool {
        let mut repetition_counter: u8 = 1;
        for pos in &self.irreversible_moves {
            if *pos == current_hash {
                repetition_counter += 1;
            }
        }
        if repetition_counter > 1 {
            return true;
        }
        return false;
    }

    #[inline(always)]
    pub fn calculate_check_squares(
        &mut self,
        king_square: usize,
        total_occ: u64,
        color: u16,
    ) -> () {
        let check_squares: &mut [u64; 5] = &mut self.check_squares;
        let diagonals: u64 = bishop_attacks(king_square, total_occ);
        let lines: u64 = rook_attacks(king_square, total_occ);

        check_squares[0] = if color == 8 {
            WHITE_PAWN_ATTACKS[king_square]
        } else {
            BLACK_PAWN_ATTACKS[king_square]
        };
        check_squares[1] = KNIGHT_ATTACKS[king_square];
        check_squares[2] = diagonals;
        check_squares[3] = lines;
        check_squares[4] = lines | diagonals;
    }
}
