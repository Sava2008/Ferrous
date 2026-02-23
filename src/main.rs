use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    board_geometry_templates::{FROM_MASK, TO_MASK, TO_SHIFT},
    constants::attacks::{
        INDICES_TO_COORDS, compute_all_lines, compute_all_piece_improvements, compute_all_rays,
        compute_all_rays_from, initialize_sliding_attack_tables,
    },
    enums::{GameResult, PieceColor},
    gamestate::GameState,
};
use std::{io, time::Instant};
pub mod alpha_beta_pruning;
pub mod board;
pub mod board_geometry_templates;
pub mod constants;
pub mod converters;
pub mod enums;
pub mod gamestate;
pub mod move_generation;
pub mod moves;
pub mod tests;
pub mod tuning;

fn main() {
    /* initialize_sliding_attack_tables(), compute_all_rays(),
    compute_all_lines, compute_all_piece_improvements
    and compute_all_rays_from() have to be called
    in the beginning of program and tests */
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_all_piece_improvements();
    let mut board: Board = Board::set();
    board.total_occupancy();
    board.update_full_cache();
    board.count_material();
    let mut state: GameState = GameState::new(&board);
    let mut white_engine: Engine = Engine {
        side: PieceColor::White,
        depth: 6,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
    };
    let mut black_engine: Engine = Engine {
        side: PieceColor::Black,
        depth: 7,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
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
    /*'outer: */
    loop {
        board.total_occupancy();
        board.update_full_cache();
        board.count_material();
        state.check_info.update(&board, &PieceColor::White);
        state.pin_info.update(&board, &PieceColor::White);
        state.update_check_constraints(&board);
        // white's move
        let start: Instant = Instant::now();
        let white_engine_move: Option<u16> = engine1.find_best_move(&board, state);
        println!("{:.3?}", start.elapsed());
        if let Some(m) = white_engine_move {
            board.perform_move(&m, state);
            println!(
                "Ferrous's move: {:?} {:?}",
                INDICES_TO_COORDS.get(&((m & FROM_MASK) as u8)).unwrap(),
                INDICES_TO_COORDS
                    .get(&(((m & TO_MASK) >> TO_SHIFT) as u8))
                    .unwrap(),
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
        board.update_full_cache();
        board.count_material();
        state.check_info.update(&board, &PieceColor::Black);
        state.pin_info.update(&board, &PieceColor::Black);
        state.update_check_constraints(&board);

        // black's move
        let start: Instant = Instant::now();
        let black_engine_move: Option<u16> = engine2.find_best_move(&board, state);
        println!("{:.3?}", start.elapsed());
        if let Some(m) = black_engine_move {
            board.perform_move(&m, state);
            println!(
                "Ferrous's move: {:?} {:?}",
                INDICES_TO_COORDS.get(&((m & FROM_MASK) as u8)).unwrap(),
                INDICES_TO_COORDS
                    .get(&(((m & TO_MASK) >> TO_SHIFT) as u8))
                    .unwrap(),
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
