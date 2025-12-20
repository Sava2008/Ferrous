use crate::enums::PieceType;
pub struct GameState {
    pub en_passant_target: Option<u8>, // the square BEHIND the pawn that has moved two squares
    pub castling_rights: CastlingRights,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enfocres 50-move rule
    pub check_info: CheckInfo,
    pub pin_info: PinInfo,
    pub moves_history: Vec<Previous_move>,
}

struct CastlingRights {
    pub white_three_zeros: bool,
    pub white_two_zeros: bool,
    pub black_three_zeros: bool,
    pub black_two_zeros: bool,
}

struct CheckInfo {
    pub checked_king: Option<u8>,
    pub first_checker: Option<u8>,
    pub second_checker: Option<u8>, // most of the times will be None, exists for double checks only
}

struct PinInfo {
    white_king: u8,
    black_king: u8,
    white_pinned_pieces: Vec<u8>,
    black_pinned_pieces: Vec<u8>,
}

struct Previous_move {
    pub previous_en_passant: Option<u8>,
    pub previous_castling_rights: CastlingRights,
    pub previous_fifty_moves_rule_counter: u8,
    pub previous_check_info: CheckInfo,
    pub previous_pin_info: PinInfo,
    pub captured_piece_type: Option<PieceType>,
}

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
