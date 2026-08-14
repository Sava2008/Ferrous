use crate::{
    board::Board,
    board_geometry_templates::{
        BLACK_BISHOP_U16, BLACK_KING_U16, BLACK_KNIGHT_U16, BLACK_PAWN_U16, BLACK_QUEEN_U16,
        BLACK_ROOK_U16, WHITE_BISHOP_U16, WHITE_KING_U16, WHITE_KNIGHT_U16, WHITE_PAWN_U16,
        WHITE_QUEEN_U16, WHITE_ROOK_U16,
    },
    constants::attacks::{
        BLACK_PAWN_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWN_ATTACKS, bishop_attacks,
        rook_attacks,
    },
};

pub struct PGNMove {
    pub move_string: String, // e.g. Qxb6# (queen takes b6 checkmate)
    pub from: u16,
    pub to_: u16,
}
enum PieceType {
    P,
    N,
    B,
    R,
    Q,
    K,
}
// returns a starting square of the moved piece
fn locate_moving_piece(
    piece_type: PieceType,
    piece_color: u8,
    end_square: usize,
    board: &Board,
) -> u16 {
    let mut potential_start_sq: u16 = 64;

    potential_start_sq = match piece_type {
        PieceType::P => locate_pawn(
            if piece_color == 8 {
                &BLACK_PAWN_ATTACKS
            } else {
                &WHITE_PAWN_ATTACKS
            },
            end_square,
            board,
        ),
        PieceType::B => locate_bishop(end_square, board),
        PieceType::N => locate_knight(end_square, board),
        PieceType::R => locate_rook(end_square, board),
        PieceType::Q => locate_queen(end_square, board),
        PieceType::K => locate_king(end_square, board),
    };

    return potential_start_sq;
}

fn locate_pawn(pawn_atk_table: &[u64; 64], end_square: usize, board: &Board) -> u16 {
    // todo: capture, ep and promotion verification
    let start_sq: usize = pawn_atk_table[end_square].trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_PAWN_U16 && piece_at_start != BLACK_PAWN_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}
fn locate_knight(end_square: usize, board: &Board) -> u16 {
    // todo: make distinctions between two pieces of the same type that can reach intersecting square
    let start_sq: usize = KNIGHT_ATTACKS[end_square].trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_KNIGHT_U16 && piece_at_start != BLACK_KNIGHT_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}
fn locate_king(end_square: usize, board: &Board) -> u16 {
    let start_sq: usize = KING_ATTACKS[end_square].trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_KING_U16 && piece_at_start != BLACK_KING_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}

fn locate_bishop(end_square: usize, board: &Board) -> u16 {
    // todo: make distinctions between two pieces of the same type that can reach intersecting square
    let start_sq: usize =
        bishop_attacks(end_square, board.total_occupancy).trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_BISHOP_U16 && piece_at_start != BLACK_BISHOP_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}
fn locate_rook(end_square: usize, board: &Board) -> u16 {
    // todo: make distinctions between two pieces of the same type that can reach intersecting square
    let start_sq: usize = rook_attacks(end_square, board.total_occupancy).trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_ROOK_U16 && piece_at_start != BLACK_ROOK_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}
fn locate_queen(end_square: usize, board: &Board) -> u16 {
    // todo: make distinctions between two pieces of the same type that can reach intersecting square
    let start_sq: usize = (bishop_attacks(end_square, board.total_occupancy)
        | rook_attacks(end_square, board.total_occupancy))
    .trailing_zeros() as usize;
    let piece_at_start: u16 = board.cached_pieces[start_sq];
    if piece_at_start != WHITE_QUEEN_U16 && piece_at_start != BLACK_QUEEN_U16 {
        panic!("weird piece {piece_at_start} on the square a pawn should occupy");
    }
    return start_sq as u16;
}
impl PGNMove {
    pub fn from_u16_move(&mut self, _mv: u16, _board: &Board) -> () {
        unimplemented!();
    }

    pub fn to_u16_move(&self, _board: &Board) -> u16 {
        unimplemented!();
    }
}
