use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::attacks::{INDICES_TO_COORDS, initialize_sliding_attack_tables},
    enums::{GameResult, PieceColor},
    gamestate::{GameState, PieceMove},
};
use std::{io, time::Instant};
pub mod alpha_beta_pruning;
pub mod board;
pub mod board_geometry_templates;
pub mod constants;
pub mod converters;
pub mod enums;
pub mod evaluation;
pub mod gamestate;
pub mod moves;
pub mod tests;

fn main() {
    initialize_sliding_attack_tables();
    let mut board: Board = Board::set();
    board.total_occupancy();
    let mut state: GameState = GameState::new();
    let mut white_engine: Engine = Engine {
        side: PieceColor::White,
        depth: 6,
        evaluation: 0,
    };
    let mut black_engine: Engine = Engine {
        side: PieceColor::Black,
        depth: 5,
        evaluation: 0,
    };
    game_control(&mut state, &mut board, &mut white_engine, &mut black_engine).unwrap();
    let _ = io::stdin();
}

// the main loop
fn game_control(
    state: &mut GameState,
    board: &mut Board,
    engine1: &mut Engine,
    engine2: &mut Engine,
) -> Result<(), io::Error> {
    'outer: loop {
        board.total_occupancy();
        state.check_info.update(&board, &PieceColor::White);
        state.pin_info.update(&board, &PieceColor::White);
        state.update_check_constraints(&board);
        println!(
            "check info for white: {:?}, pin_info for white: {:?}, check constraints {:b}\nboard: {board:?}",
            state.check_info, state.pin_info, state.check_contraints
        );

        // white's move
        let start: Instant = Instant::now();
        let white_engine_move: Option<PieceMove> = engine1.find_best_move(&board, state);
        println!("{:.3?}", start.elapsed());
        if let Some(m) = white_engine_move {
            board.perform_move(&m, state);
            println!(
                "Ferrous's move: {:?} {:?}",
                INDICES_TO_COORDS.get(&m.from).unwrap(),
                INDICES_TO_COORDS.get(&m.to).unwrap(),
            );
        } else {
            if state.check_info.checked_king.is_some() {
                state.result = GameResult::BlackWins;
                println!("you won by checkmate");
            } else {
                state.result = GameResult::Draw;
                println!("stalemate");
            }
            break;
        }
        board.total_occupancy();
        state.check_info.update(&board, &PieceColor::Black);
        state.pin_info.update(&board, &PieceColor::Black);
        state.update_check_constraints(&board);
        println!(
            "check info for black: {:?}, pin_info for black: {:?}, check constraints {:b}\nboard: {board:?}",
            state.check_info, state.pin_info, state.check_contraints
        );

        // black's move
        let start: Instant = Instant::now();
        let black_engine_move: Option<PieceMove> = engine2.find_best_move(&board, state);
        println!("{:.3?}", start.elapsed());
        if let Some(m) = black_engine_move {
            board.perform_move(&m, state);
            println!(
                "Ferrous's move: {:?} {:?}",
                INDICES_TO_COORDS.get(&m.from).unwrap(),
                INDICES_TO_COORDS.get(&m.to).unwrap(),
            );
        } else {
            if state.check_info.checked_king.is_some() {
                state.result = GameResult::BlackWins;
                println!("you won by checkmate");
            } else {
                state.result = GameResult::Draw;
                println!("stalemate");
            }
            break;
        }
        /*
        loop {
            println!("input a move, for example:\ne2 e4");
            let mut user_move: String = String::with_capacity(7);
            io::stdin().read_line(&mut user_move)?;

            if user_move.len() != 7 {
                println!("the move should contain a starting square and the destination square");
                continue;
            }
            let piece_move: Vec<u8> = user_move
                .split_whitespace()
                .filter_map(|pos: &str| COORDS_TO_INDICES.get(pos).cloned())
                .collect::<Vec<u8>>();
            let piece: (PieceColor, PieceType) =
                if let Some(m) = board.bitboard_contains(piece_move[0]) {
                    m
                } else {
                    println!("no piece stands on the starting square");
                    continue;
                };

            if piece.0 == engine.side {
                println!("the starting square is occupied by your opponent's piece");
                continue;
            }

            let mut legal_moves: Vec<PieceMove> = board.knight_moves(&state, &piece.0);
            legal_moves.extend(board.bishop_moves(&state, &piece.0));
            legal_moves.extend(board.rook_moves(&state, &piece.0));
            legal_moves.extend(board.pawn_moves(&state, &piece.0));
            legal_moves.extend(board.queen_moves(&state, &piece.0));
            legal_moves.extend(board.king_moves(&state, &piece.0));
            if legal_moves.len() == 0 {
                if state.check_info.checked_king.is_some() {
                    state.result = GameResult::WhiteWins;
                    println!("you are checkmated");
                    break 'outer;
                } else {
                    state.result = GameResult::Draw;
                    println!("stalemate");
                    break 'outer;
                }
            }

            if legal_moves
                .iter()
                .any(|mv: &PieceMove| mv.to == piece_move[1])
            {
                board.perform_move(
                    &PieceMove {
                        from: piece_move[0],
                        to: piece_move[1],
                    },
                    state,
                );
            } else {
                println!("illegal move");
                continue;
            }
            break;
        }

        match state.result {
            GameResult::Going => continue,
            _ => break,
        };*/
    }
    return Ok(());
}
