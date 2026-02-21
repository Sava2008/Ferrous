use crate::board_geometry_templates::Bitboard;
use crate::enums::{GameResult, PieceColor, PieceType};
use crate::gamestate::{CastlingRights, CheckInfo, PinInfo};
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
                c.to_digit(10).unwrap() as u8
            } else {
                match c {
                    'a' => 1,
                    'b' => 2,
                    'c' => 3,
                    'd' => 4,
                    'e' => 5,
                    'f' => 6,
                    'g' => 7,
                    'h' => 8,
                    _ => unreachable!(),
                }
            }
        })
        .collect();
    return row_col[1] * 8 + row_col[0];
}

pub fn board_to_fen(board: &Board, state: &GameState, whose_move: &PieceColor) -> String {
    let mut fen: String = String::with_capacity(100);
    for rank in (0..8).rev() {
        let mut empty_counter: u8 = 0;

        for file in 0..8 {
            let square: u8 = rank * 8 + file;
            let mask: Bitboard = 1 << square;

            if board.total_occupancy & mask == 0 {
                empty_counter += 1;
            } else {
                if empty_counter > 0 {
                    fen.push_str(&empty_counter.to_string());
                    empty_counter = 0;
                }

                let piece: (PieceColor, PieceType) = board.piece_at(&(square as u16)).unwrap();
                let piece_char: char = match piece {
                    (PieceColor::White, PieceType::King) => 'K',
                    (PieceColor::White, PieceType::Queen) => 'Q',
                    (PieceColor::White, PieceType::Rook) => 'R',
                    (PieceColor::White, PieceType::Bishop) => 'B',
                    (PieceColor::White, PieceType::Knight) => 'N',
                    (PieceColor::White, PieceType::Pawn) => 'P',
                    (PieceColor::Black, PieceType::King) => 'k',
                    (PieceColor::Black, PieceType::Queen) => 'q',
                    (PieceColor::Black, PieceType::Rook) => 'r',
                    (PieceColor::Black, PieceType::Bishop) => 'b',
                    (PieceColor::Black, PieceType::Knight) => 'n',
                    (PieceColor::Black, PieceType::Pawn) => 'p',
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
        PieceColor::White => " w ",
        PieceColor::Black => " b ",
    });

    let castling_rights: [bool; 4] = state.castling_rights.to_array();
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
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_rooks: 0,
        white_king: 0,
        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_rooks: 0,
        black_king: 0,
        white_occupancy: 0,
        black_occupancy: 0,
        total_occupancy: 0,
        cached_pieces: [None; 64],
        material: 0,
    };
    let mut state: GameState = GameState {
        en_passant_target: None,
        castling_rights: CastlingRights {
            white_three_zeros: false,
            white_two_zeros: false,
            black_three_zeros: false,
            black_two_zeros: false,
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
                'P' => board.white_pawns |= 1 << mirrored_index,
                'p' => board.black_pawns |= 1 << mirrored_index,
                'B' => board.white_bishops |= 1 << mirrored_index,
                'b' => board.black_bishops |= 1 << mirrored_index,
                'N' => board.white_knights |= 1 << mirrored_index,
                'n' => board.black_knights |= 1 << mirrored_index,
                'R' => board.white_rooks |= 1 << mirrored_index,
                'r' => board.black_rooks |= 1 << mirrored_index,
                'Q' => board.white_queens |= 1 << mirrored_index,
                'q' => board.black_queens |= 1 << mirrored_index,
                'K' => board.white_king |= 1 << mirrored_index,
                'k' => board.black_king |= 1 << mirrored_index,
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
        "w" => PieceColor::White,
        "b" => PieceColor::Black,
        _ => unreachable!(),
    };

    for right in castling.chars() {
        match right {
            'Q' => state.castling_rights.white_three_zeros = true,
            'q' => state.castling_rights.black_three_zeros = true,
            'K' => state.castling_rights.white_two_zeros = true,
            'k' => state.castling_rights.black_two_zeros = true,
            '-' => (),
            _ => unreachable!(),
        };
    }
    match en_passant {
        // wrong logic, because en passant should be capturable
        "-" => (),
        c_n => state.en_passant_target = Some(chess_notation_to_index(c_n)),
    }
    (state.fifty_moves_rule_counter, state.total_moves_amount) =
        (fifty_moves.parse().unwrap(), total_moves.parse().unwrap());

    board.total_occupancy();
    board.update_full_cache();

    return (board, state);
}
