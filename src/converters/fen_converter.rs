use crate::enums::PieceColor;
use crate::{board::Board, gamestate::GameState};

#[derive(Clone, Copy, Debug)]
enum ChessPiece {
    WP,
    WN,
    WB,
    WR,
    WQ,
    WK,
    BP,
    BN,
    BB,
    BR,
    BQ,
    BK,
    Void,
}
fn board_to_array(board: &Board) -> [ChessPiece; 64] {
    let mut array_board: [ChessPiece; 64] = [ChessPiece::Void; 64];
    for (bitboard, piecetype) in [
        (board.white_pawns, ChessPiece::WP),
        (board.white_knights, ChessPiece::WN),
        (board.white_bishops, ChessPiece::WB),
        (board.white_rooks, ChessPiece::WR),
        (board.white_queens, ChessPiece::WQ),
        (board.white_king, ChessPiece::WK),
        (board.black_pawns, ChessPiece::BP),
        (board.black_knights, ChessPiece::BN),
        (board.black_bishops, ChessPiece::BB),
        (board.black_rooks, ChessPiece::BR),
        (board.black_queens, ChessPiece::BQ),
        (board.black_king, ChessPiece::BK),
    ]
    .iter()
    {
        let mut bb: u64 = *bitboard;
        while bb != 0 {
            let abscised_bitboard: u8 = bb.trailing_zeros() as u8;
            array_board[63 - abscised_bitboard as usize] = *piecetype;
            bb &= bb - 1;
        }
    }
    return array_board;
}

fn index_to_chess_notation(idx: u8) -> String {
    let coords: (u8, u8) = (idx / 8, idx % 8);
    return format!(
        "{},{}",
        match coords.0 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(),
        },
        coords.1 + 1
    );
}

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

pub fn board_to_fen(board: &Board, state: &GameState) -> String {
    let mut fen: String = String::with_capacity(100);
    let mut empty_square_counter: u8 = 0;
    let mut previous_empty: bool = false;
    let board: [ChessPiece; 64] = board_to_array(board);

    for (iteration, square) in board.iter().enumerate() {
        if iteration % 8 == 0 && iteration != 0 {
            if empty_square_counter != 0 {
                fen.push_str(empty_square_counter.to_string().as_str());
            }
            fen.push('/');
            empty_square_counter = 0
        }
        match square {
            &ChessPiece::Void => {
                empty_square_counter += 1;
                previous_empty = true;
            }
            piece => {
                let symbol: char = match piece {
                    &ChessPiece::BB => 'b',
                    &ChessPiece::BN => 'n',
                    &ChessPiece::BQ => 'k',
                    &ChessPiece::BR => 'r',
                    &ChessPiece::BP => 'p',
                    &ChessPiece::BK => 'q',
                    &ChessPiece::WB => 'B',
                    &ChessPiece::WN => 'N',
                    &ChessPiece::WQ => 'K',
                    &ChessPiece::WR => 'R',
                    &ChessPiece::WP => 'P',
                    &ChessPiece::WK => 'Q',
                    _ => unreachable!(),
                };
                (previous_empty, empty_square_counter) =
                    update_fen_with_piece(&mut fen, empty_square_counter, previous_empty, symbol);
            }
        }
    }
    if previous_empty {
        fen.push_str(&empty_square_counter.to_string());
    }

    fen.push_str(match state.whose_turn {
        PieceColor::Black => " b ",
        PieceColor::White => " w ",
    });

    let castling: String = "KQkq"
        .char_indices()
        .filter(|(i, _)| state.castling_rights.to_array()[*i])
        .map(|(_, c)| c)
        .collect::<String>();
    fen.push_str(if castling.is_empty() { "-" } else { &castling });

    if let Some(i) = state.en_passant_target {
        fen.push_str(&format!(" {}", index_to_chess_notation(i)));
    } else {
        fen.push_str(" -");
    }
    fen = fen.chars().filter(|x: &char| *x != '0').collect::<String>();

    fen.push_str(&format!(
        " {} {}",
        state.fifty_moves_rule_counter, state.total_moves_amount
    ));
    return fen;
}
