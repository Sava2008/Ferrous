use crate::board_geometry_templates::*;
use crate::enums::GameResult;
use crate::{board::Board, gamestate::GameState};

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

fn chess_notation_to_index(c_n: &str) -> u8 {
    let row_col: Vec<u8> = c_n
        .chars()
        .map(|c: char| {
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap() as u8 - 1
            } else {
                match c {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => unreachable!(),
                }
            }
        })
        .collect();
    return row_col[1] * 8 + row_col[0];
}

pub fn board_to_fen(board: &Board, state: &GameState, whose_move: &u8) -> String {
    let mut fen: String = String::with_capacity(100);
    for rank in (0..8).rev() {
        let mut empty_counter: u8 = 0;

        for file in 0..8 {
            let square: u16 = rank * 8 + file;
            let mask: u64 = 1 << square;

            if board.total_occupancy & mask == 0 {
                empty_counter += 1;
            } else {
                if empty_counter > 0 {
                    fen.push_str(&empty_counter.to_string());
                    empty_counter = 0;
                }

                let piece: u16 = board.piece_at(square);
                let piece_char: char = match piece {
                    WHITE_KING_U16 => 'K',
                    WHITE_QUEEN_U16 => 'Q',
                    WHITE_ROOK_U16 => 'R',
                    WHITE_BISHOP_U16 => 'B',
                    WHITE_KNIGHT_U16 => 'N',
                    WHITE_PAWN_U16 => 'P',
                    BLACK_KING_U16 => 'k',
                    BLACK_QUEEN_U16 => 'q',
                    BLACK_ROOK_U16 => 'r',
                    BLACK_BISHOP_U16 => 'b',
                    BLACK_KNIGHT_U16 => 'n',
                    _ => 'p',
                };
                fen.push(piece_char);
            }
        }

        if empty_counter > 0 {
            fen.push_str(&empty_counter.to_string());
        }

        if rank > 0 {
            fen.push('/');
        }
    }

    fen.push_str(match whose_move {
        8 => " w ",
        _ => " b ",
    });

    let castling_rights: [bool; 4] = match state.castling_rights {
        3 => [true, true, false, false],
        7 => [true, true, true, false],
        11 => [true, true, false, true],
        12 => [false, false, true, true],
        13 => [true, false, true, true],
        14 => [false, true, true, true],
        15 => [true; 4],
        _ => [false; 4],
    };
    let mut castling_str: String = String::new();

    if castling_rights[0] {
        castling_str.push('K');
    }
    if castling_rights[1] {
        castling_str.push('Q');
    }
    if castling_rights[2] {
        castling_str.push('k');
    }
    if castling_rights[3] {
        castling_str.push('q');
    }

    fen.push_str(if castling_str.is_empty() {
        "-"
    } else {
        &castling_str
    });

    if let Some(sq) = state.en_passant_target {
        fen.push_str(&format!(" {}", index_to_chess_notation(sq)));
    } else {
        fen.push_str(" -");
    }

    fen.push_str(&format!(
        " {} {}",
        state.fifty_moves_rule_counter, state.total_moves_amount
    ));

    return fen;
}

pub fn fen_to_board(fen: &str) -> (Board, GameState) {
    let mut board: Board = Board {
        bitboards: [0; 12],
        occupancies: [0; 2],
        total_occupancy: 0,
        cached_pieces: [0; 64],
        white_king_square: 0,
        black_king_square: 0,
    };
    let mut state: GameState = GameState {
        en_passant_target: None,
        castling_rights: 0,
        fifty_moves_rule_counter: 0,
        moves_history: Vec::new(),
        total_moves_amount: 0,
        whose_turn: 8,
        result: GameResult::Going,
        irreversible_moves: Vec::new(),
        white_legal_squares_mask: u64::MAX,
        black_legal_squares_mask: u64::MAX,
    };
    let mut index: u8 = 63;
    let mut split_fen: std::str::SplitWhitespace<'_> = fen.split_whitespace();
    let (pieces, color, castling, en_passant, fifty_moves, total_moves): (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    ) = (
        split_fen.next().unwrap(),
        split_fen.next().unwrap(),
        split_fen.next().unwrap(),
        split_fen.next().unwrap(),
        split_fen.next().unwrap(),
        split_fen.next().unwrap(),
    );

    for c in pieces.chars() {
        if c.is_ascii_digit() {
            let empty_squares: u8 = c.to_digit(10).unwrap() as u8;
            if index < empty_squares {
                break;
            }
            index -= empty_squares;
        } else if c.is_ascii_alphabetic() {
            let mirrored_index: u8 = ((index / 8) * 8 + (8 - (index % 8))) - 1;
            match c {
                'P' => board.bitboards[0] |= 1 << mirrored_index,
                'p' => board.bitboards[6] |= 1 << mirrored_index,
                'B' => board.bitboards[2] |= 1 << mirrored_index,
                'b' => board.bitboards[8] |= 1 << mirrored_index,
                'N' => board.bitboards[1] |= 1 << mirrored_index,
                'n' => board.bitboards[7] |= 1 << mirrored_index,
                'R' => board.bitboards[3] |= 1 << mirrored_index,
                'r' => board.bitboards[9] |= 1 << mirrored_index,
                'Q' => board.bitboards[4] |= 1 << mirrored_index,
                'q' => board.bitboards[10] |= 1 << mirrored_index,
                'K' => board.bitboards[5] |= 1 << mirrored_index,
                'k' => board.bitboards[11] |= 1 << mirrored_index,
                _ => unreachable!(),
            };
            if index == 0 {
                break;
            }
            index -= 1;
        } else if c == '/' {
            continue;
        }
    }

    state.whose_turn = match color {
        "w" => 8,
        "b" => 16,
        _ => unreachable!(),
    };

    for right in castling.chars() {
        match right {
            'Q' => state.castling_rights |= WHITE_LONG,
            'q' => state.castling_rights |= BLACK_LONG,
            'K' => state.castling_rights |= WHITE_SHORT,
            'k' => state.castling_rights |= BLACK_SHORT,
            '-' => (),
            _ => unreachable!(),
        };
    }
    match en_passant {
        // wrong logic, because en passant should be capturable
        "-" => (),
        c_n => {
            state.en_passant_target = Some(chess_notation_to_index(c_n));
        }
    }

    (state.fifty_moves_rule_counter, state.total_moves_amount) =
        (fifty_moves.parse().unwrap(), total_moves.parse().unwrap());

    board.total_occupancy();
    board.update_full_cache();
    board.white_king_square = board.bitboards[5].trailing_zeros() as u8;
    board.black_king_square = board.bitboards[11].trailing_zeros() as u8;

    return (board, state);
}
