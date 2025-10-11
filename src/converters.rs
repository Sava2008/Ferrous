use crate::game_logic::{Board, pieces::ChessPiece, state_enums::PieceColor};
use crate::helper_functions::index_to_chess_notation;

fn update_fen_with_piece(
    fen: &mut String,
    empty_square_counter: u8,
    previous_empty: bool,
    piece: char,
) -> (bool, u8) {
    if previous_empty {
        fen.push_str(empty_square_counter.to_string().as_str());
    }
    fen.push(piece);
    return (false, 0);
}

pub fn board_to_fen(
    board: &[ChessPiece; 64],
    turn: PieceColor,
    rights_to_castle: [bool; 4],
    en_passant_possible_on: Option<usize>,
    no_pawn_move_or_capture_for: u8,
    moves_made: u16,
) -> String {
    let mut fen: String = String::with_capacity(100);
    let mut empty_square_counter: u8 = 0;
    let mut previous_empty: bool = false;

    for (iteration, square) in board.iter().enumerate() {
        if iteration % 8 == 0 && iteration != 0 {
            if empty_square_counter != 0 {
                fen.push_str(empty_square_counter.to_string().as_str());
            }
            fen.push('/');
            empty_square_counter = 0
        }
        match (square, square.color()) {
            (ChessPiece::Square(_), None) => {
                empty_square_counter += 1;
                previous_empty = true;
            }
            (piece, Some(c)) => {
                let symbol: char = match piece {
                    ChessPiece::B(_) => 'b',
                    ChessPiece::N(_) => 'n',
                    ChessPiece::Q(_) => 'q',
                    ChessPiece::R(_) => 'r',
                    ChessPiece::P(_) => 'p',
                    ChessPiece::K(_) => 'k',
                    _ => unreachable!(),
                };
                (previous_empty, empty_square_counter) = update_fen_with_piece(
                    &mut fen,
                    empty_square_counter,
                    previous_empty,
                    match c {
                        PieceColor::Black => symbol,
                        PieceColor::White => symbol.to_ascii_uppercase(),
                    },
                );
            }
            (_, None) => unreachable!(),
        }
    }
    if previous_empty {
        fen.push_str(&empty_square_counter.to_string());
    }

    fen.push_str(match turn {
        PieceColor::Black => " b ",
        PieceColor::White => " w ",
    });

    let castling: String = "KQkq"
        .char_indices()
        .filter(|(i, _)| rights_to_castle[*i])
        .map(|(_, c)| c)
        .collect::<String>();
    fen.push_str(if castling.is_empty() { "-" } else { &castling });

    if let Some(i) = en_passant_possible_on {
        fen.push_str(&format!(" {}", index_to_chess_notation(i)));
    } else {
        fen.push_str(" -");
    }

    fen.push_str(&format!(" {} {}", no_pawn_move_or_capture_for, moves_made));
    return fen;
}

pub fn fen_to_board(_board: &Board) -> () {}
