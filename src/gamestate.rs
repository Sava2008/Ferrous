use crate::{
    board::Board,
    board_geometry_templates::Bitboard,
    constants::attacks::{
        BLACK_PAWN_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWN_ATTACKS, bishop_attacks, rook_attacks,
    },
    enums::{GameResult, InclusiveRange, PieceColor, PieceType},
};
use smallvec::SmallVec;

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
    pub castling_rights: CastlingRights,
    pub fifty_moves_rule_counter: u8, // how many moves since the last capture/pawn advancement. enforces 50-move rule
    pub check_info: CheckInfo,
    pub pin_info: PinInfo,
    pub moves_history: Vec<PreviousMove>,
    pub total_moves_amount: u16,
    pub whose_turn: PieceColor,
    pub result: GameResult,
    pub check_contraints: Bitboard, // all the allowed squares for friendly pieces except the king during a check
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

    pub fn update(&mut self, board: &Board, whose_turn: &PieceColor) -> () {
        self.checked_king = None;
        self.first_checker = None;
        self.second_checker = None;
        let king_square: usize = match whose_turn {
            &PieceColor::White => board.white_king,
            &PieceColor::Black => board.black_king,
        }
        .trailing_zeros() as usize;

        let (diagonals, lines, knight_deltas, pawn_deltas) = (
            bishop_attacks(king_square, board.total_occupancy),
            rook_attacks(king_square, board.total_occupancy),
            KNIGHT_ATTACKS[king_square],
            match whose_turn {
                &PieceColor::Black => BLACK_PAWN_ATTACKS[king_square],
                &PieceColor::White => WHITE_PAWN_ATTACKS[king_square],
            },
        );
        let enemy_pieces: [&u64; 5] = match whose_turn {
            PieceColor::White => [
                &(&board.black_queens & (&lines | &diagonals)),
                &(&board.black_rooks & &lines),
                &(&board.black_bishops & &diagonals),
                &(&board.black_knights & &knight_deltas),
                &(&board.black_pawns & &pawn_deltas),
            ],
            PieceColor::Black => [
                &(&board.white_queens & (&lines | &diagonals)),
                &(&board.white_rooks & &lines),
                &(&board.white_bishops & &diagonals),
                &(&board.white_knights & &knight_deltas),
                &(&board.white_pawns & &pawn_deltas),
            ],
        };
        for enemy_bitboard in enemy_pieces {
            if *enemy_bitboard == 0 {
                continue;
            }
            let checker: u8 = enemy_bitboard.trailing_zeros() as u8;
            if checker == 64 {
                continue;
            }
            self.checked_king = Some(king_square as u8);
            match (self.first_checker, self.second_checker) {
                (Some(_), None) => {
                    self.second_checker = Some(checker);
                    return ();
                }
                (None, None) => self.first_checker = Some(checker),
                _ => unreachable!(),
            };
            if
            /*  i == 0  &&*/
            enemy_bitboard.count_ones() > 1 {
                let mut bb: Bitboard = *enemy_bitboard;
                bb &= bb - 1;
                let checker: u8 = (bb & (diagonals | lines)).trailing_zeros() as u8;

                if checker == 64 {
                    continue;
                }
                self.second_checker = Some(checker);
                return ();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PinInfo {
    pub white_king: u8,
    pub black_king: u8,
    pub pinned_pieces: SmallVec<[PinnedPiece; 8]>,
}
impl PinInfo {
    #[inline]
    pub fn new() -> Self {
        return Self {
            white_king: 4,  // e1
            black_king: 60, // e8
            pinned_pieces: SmallVec::new(),
        };
    }

    pub fn update(&mut self, board: &Board, color: &PieceColor) -> () {
        self.white_king = board.white_king.trailing_zeros() as u8;
        self.black_king = board.black_king.trailing_zeros() as u8;

        let (lines, diagonals): (Bitboard, Bitboard) = match color {
            &PieceColor::White => (
                rook_attacks(self.white_king as usize, board.black_occupancy),
                bishop_attacks(self.white_king as usize, board.black_occupancy),
            ),
            &PieceColor::Black => (
                rook_attacks(self.black_king as usize, board.white_occupancy),
                bishop_attacks(self.black_king as usize, board.white_occupancy),
            ),
        };
        let (mut linear_attackers, mut diagonal_attackers, king, friendly_occupancy): (
            Bitboard,
            Bitboard,
            &u8,
            &Bitboard,
        ) = match color {
            PieceColor::White => (
                lines & (&board.black_queens | &board.black_rooks),
                diagonals & (&board.black_queens | &board.black_bishops),
                &self.white_king,
                &board.white_occupancy,
            ),
            PieceColor::Black => (
                lines & (&board.white_queens | &board.white_rooks),
                diagonals & (&board.white_queens | &board.white_bishops),
                &self.black_king,
                &board.black_occupancy,
            ),
        };
        self.pinned_pieces.clear();
        while linear_attackers != 0 {
            let pin_ray: Bitboard = Board::generate_range(
                *king,
                linear_attackers.trailing_zeros() as u8,
                &InclusiveRange::LastOnly,
            );
            linear_attackers &= linear_attackers - 1;
            let teammates_in_between: Bitboard = pin_ray & friendly_occupancy;
            if teammates_in_between.count_ones() != 1 {
                continue;
            }
            self.pinned_pieces.push(PinnedPiece {
                square: teammates_in_between.trailing_zeros() as u8,
                pin_ray,
            });
        }
        while diagonal_attackers != 0 {
            let pin_ray: Bitboard = Board::generate_range(
                *king,
                diagonal_attackers.trailing_zeros() as u8,
                &InclusiveRange::LastOnly,
            );
            diagonal_attackers &= diagonal_attackers - 1;
            let teammates_in_between: Bitboard = pin_ray & friendly_occupancy;
            if teammates_in_between.count_ones() != 1 {
                continue;
            }
            self.pinned_pieces.push(PinnedPiece {
                square: teammates_in_between.trailing_zeros() as u8,
                pin_ray,
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PinnedPiece {
    pub square: u8,
    pub pin_ray: Bitboard, // all available squares for the pinned piece
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreviousMove {
    // normal move changes 1 bitboard, castling or capture changes 2 and promotion with capture changes 3
    pub changed_bitboards: [(Option<(PieceColor, PieceType)>, Option<Bitboard>); 3],
    pub previous_en_passant: Option<u8>,
    pub previous_castling_rights: Option<CastlingRights>, // if None, not to be restored
    // pub previous_fifty_moves_rule_counter: u8,
    pub previous_check_info: CheckInfo,
    pub previous_pin_info: PinInfo,
    pub previous_check_constraints: Bitboard,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PieceMove {
    pub from: u8,
    pub to: u8,
}

impl GameState {
    #[inline]
    pub fn new(board: &Board) -> Self {
        return Self {
            en_passant_target: None,
            castling_rights: match (board.white_king, board.black_king) {
                (4, 60) => CastlingRights::new(),
                (4, _) => CastlingRights {
                    white_three_zeros: true,
                    white_two_zeros: true,
                    black_three_zeros: false,
                    black_two_zeros: false,
                },
                (_, 60) => CastlingRights {
                    white_three_zeros: false,
                    white_two_zeros: false,
                    black_three_zeros: true,
                    black_two_zeros: true,
                },
                (_, _) => CastlingRights {
                    white_three_zeros: false,
                    white_two_zeros: false,
                    black_three_zeros: false,
                    black_two_zeros: false,
                },
            },
            fifty_moves_rule_counter: 0,
            check_info: CheckInfo::new(),
            pin_info: PinInfo::new(),
            moves_history: Vec::new(),
            total_moves_amount: 0,
            whose_turn: PieceColor::White,
            result: GameResult::Going,
            check_contraints: 0,
        };
    }

    pub fn update_check_constraints(&mut self, board: &Board) -> () {
        if self.check_info.checked_king.is_none() || self.check_info.second_checker.is_some() {
            self.check_contraints = 0;
            return;
        }
        let checker_index: u8 = self.check_info.first_checker.unwrap();
        let piece: (PieceColor, PieceType) = board.bitboard_contains(checker_index).unwrap();
        let color: PieceColor = board
            .bitboard_contains(self.check_info.checked_king.unwrap())
            .unwrap()
            .0;
        if piece.0 == color {
            println!(
                "piece: {:?}, check_info: {:?}, position: {:?}",
                piece, self.check_info, board
            );
            panic!("irrelevant color");
        }
        self.check_contraints = match board.bitboard_contains(checker_index).unwrap().1 {
            PieceType::Bishop | PieceType::Queen | PieceType::Rook => Board::generate_range(
                self.check_info.checked_king.unwrap(),
                checker_index,
                &InclusiveRange::LastOnly,
            ),
            PieceType::Knight | PieceType::Pawn => 1 << checker_index,
            _ => panic!("irrelevant piece color or piece type"),
        };
    }
}
