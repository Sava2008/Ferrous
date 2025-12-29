use crate::enums::{PieceColor, PieceType};
#[derive(Debug)]
pub struct GameState {
    pub en_passant_target: Option<u8>, // the square BEHIND the pawn that has moved two squares
    pub castling_rights: CastlingRights,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enfocres 50-move rule
    pub check_info: CheckInfo,
    pub pin_info: PinInfo,
    pub moves_history: Vec<PreviousMove>,
    pub total_moves_amount: u16,
    pub whose_turn: PieceColor,
}

#[derive(Debug)]
pub struct CastlingRights {
    pub white_three_zeros: bool,
    pub white_two_zeros: bool,
    pub black_three_zeros: bool,
    pub black_two_zeros: bool,
}

#[derive(Debug)]
pub struct CheckInfo {
    pub checked_king: Option<u8>,
    pub first_checker: Option<u8>,
    pub second_checker: Option<u8>, // most of the times will be None, exists for double checks only
}

#[derive(Debug)]
pub struct PinInfo {
    pub white_king: u8,
    pub black_king: u8,
    pub white_pinned_pieces: Vec<u8>,
    pub black_pinned_pieces: Vec<u8>,
}

#[derive(Debug)]
pub struct PreviousMove {
    pub previous_en_passant: Option<u8>,
    pub previous_castling_rights: CastlingRights,
    pub previous_fifty_moves_rule_counter: u8,
    pub previous_check_info: CheckInfo,
    pub previous_pin_info: PinInfo,
    pub captured_piece_type: Option<PieceType>,
}

#[derive(Debug)]
pub struct PieceMove {
    pub from: u8,
    pub to: u8,
}

impl GameState {
    pub fn update_check_info(&mut self) -> Result<(), String> {
        todo!();
    }

    pub fn update_pin_info(&mut self) -> Result<(), String> {
        todo!();
    }
}
