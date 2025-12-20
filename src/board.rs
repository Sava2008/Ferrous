use crate::{board_geometry_templates::*, gamestate::GameState};
// standard representation: 0b0000000000000000000000000000000000000000000000000000000000000000 (binary)
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

    pub white_occupancy: Option<Bitboard>,
    pub black_occupancy: Option<Bitboard>,
    pub total_occupancy: Option<Bitboard>,
}

impl Board {
    pub fn set() -> Self {
        return Board {
            white_pawns: 0b0000000011111111000000000000000000000000000000000000000000000000,
            white_knights: 0b0100001000000000000000000000000000000000000000000000000000000000,
            white_bishops: 0b0010010000000000000000000000000000000000000000000000000000000000,
            white_queens: 0b0001000000000000000000000000000000000000000000000000000000000000,
            white_rooks: 0b1000000100000000000000000000000000000000000000000000000000000000,
            white_king: 0b0000100000000000000000000000000000000000000000000000000000000000,
            black_pawns: 0b0000000000000000000000000000000000000000000000001111111100000000,
            black_knights: 0b0000000000000000000000000000000000000000000000000000000001000010,
            black_bishops: 0b0000000000000000000000000000000000000000000000000000000000100100,
            black_queens: 0b0000000000000000000000000000000000000000000000000000000000010000,
            black_rooks: 0b0000000000000000000000000000000000000000000000000000000010000001,
            black_king: 0b0000000000000000000000000000000000000000000000000000000000001000,
            white_occupancy: None,
            black_occupancy: None,
            total_occupancy: None,
        };
    }

    pub fn white_occupancy(&mut self) -> () {
        self.white_occupancy = Some(
            self.white_bishops
                | self.white_king
                | self.white_knights
                | self.white_pawns
                | self.white_queens
                | self.white_rooks,
        );
    }

    pub fn black_occupancy(&mut self) -> () {
        self.black_occupancy = Some(
            self.black_bishops
                | self.black_king
                | self.black_knights
                | self.black_pawns
                | self.black_queens
                | self.black_rooks,
        );
    }

    pub fn total_occupancy(&mut self) -> () {
        match (self.white_occupancy, self.black_occupancy) {
            (None, None) => {
                self.white_occupancy();
                self.black_occupancy();
            }
            (None, Some(_)) => {
                self.white_occupancy();
            }
            (Some(_), None) => {
                self.black_occupancy();
            }
            (Some(_), Some(_)) => (),
        };
        self.total_occupancy = Some(self.white_occupancy.unwrap() | self.black_occupancy.unwrap());
    }
}
