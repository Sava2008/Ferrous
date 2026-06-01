use crate::{
    board::Board,
    board_geometry_templates::{BLACK_LONG, BLACK_SHORT, WHITE_LONG, WHITE_SHORT},
    enums::GameResult,
};

/* order of updating the fields:
1. whose_turn
2. result
3. fifty_move_rule_counter
4. total_moves_amount
5. check_info, pin_info
6. check_contraints  */

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub en_passant_target: Option<u8>, // the square BEHIND the pawn that has moved two squares
    pub castling_rights: u8,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enforces 50-move rule
    pub moves_history: Vec<PreviousMove>,
    pub total_moves_amount: u16,
    pub whose_turn: u16,
    pub result: GameResult,
    pub irreversible_moves: Vec<u64>,
    pub white_legal_squares_mask: u64,
    pub black_legal_squares_mask: u64,
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
    pub check_restrictions: u64,
}

impl GameState {
    #[inline]
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
            result: GameResult::Going,
            irreversible_moves: Vec::new(),
            white_legal_squares_mask: u64::MAX, // all squares available
            black_legal_squares_mask: u64::MAX, // all squares available
        };
    }

    pub fn is_repetition(&self, current_hash: u64) -> bool {
        let mut repetition_counter: u8 = 1;
        for pos in &self.irreversible_moves {
            if *pos == current_hash {
                repetition_counter += 1;
            }
        }
        if repetition_counter > 2 {
            return true;
        }
        return false;
    }
}
