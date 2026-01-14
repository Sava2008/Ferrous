use crate::{
    board::Board,
    enums::{PieceColor, PieceType},
};
#[derive(Debug)]
pub struct GameState {
    pub en_passant_target: Option<u8>, // the square BEHIND the pawn that has moved two squares
    pub castling_rights: CastlingRights,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enforces 50-move rule
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

#[derive(Debug)]
pub struct CheckInfo {
    pub checked_king: Option<u8>,
    pub first_checker: Option<u8>,
    pub second_checker: Option<u8>, // most of the times will be None, exists for double checks only
}
impl CheckInfo {
    #[inline]
    pub fn new() -> Self {
        return Self {
            checked_king: None,
            first_checker: None,
            second_checker: None,
        };
    }

    pub fn update(&mut self, _board: &Board) -> () {
        todo!();
    }
}

#[derive(Debug)]
pub struct PinInfo {
    pub white_king: u8,
    pub black_king: u8,
    pub white_pinned_pieces: Vec<u8>,
    pub black_pinned_pieces: Vec<u8>,
}
impl PinInfo {
    #[inline]
    pub fn new() -> Self {
        return Self {
            white_king: 4,  // e1
            black_king: 60, // e8
            white_pinned_pieces: Vec::new(),
            black_pinned_pieces: Vec::new(),
        };
    }

    pub fn update(&mut self, board: &Board) -> () {
        self.white_king = board.white_king.trailing_zeros() as u8;
        self.black_king = board.black_king.trailing_zeros() as u8;
        self.white_pinned_pieces = Vec::new(); // temporary solutions instead of calculating pins
        self.black_pinned_pieces = Vec::new();
    }
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
    #[inline]
    pub fn new() -> Self {
        return Self {
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            fifty_moves_rule_counter: 0,
            check_info: CheckInfo::new(),
            pin_info: PinInfo::new(),
            moves_history: Vec::new(),
            total_moves_amount: 0,
            whose_turn: PieceColor::White,
        };
    }
}
