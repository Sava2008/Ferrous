use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    board_geometry_templates::{
        FROM_MASK, NO_PIECE_BLACK, NO_PIECE_WHITE, PROMOTION_SHIFT, TO_MASK, TO_SHIFT,
    },
    constants::attacks::{
        COORDS_TO_INDICES, INDICES_TO_COORDS, compute_all_lines, compute_all_rays,
        compute_all_rays_from, compute_mvvlva, initialize_sliding_attack_tables,
    },
    enums::GameResult,
    gamestate::GameState,
};
use std::{
    io::{self, Write},
    time::Instant,
};
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

enum MoveResult {
    Win,
    Draw,
    Continue,
    None,
}

fn main() -> () {
    /* initialize_sliding_attack_tables(), compute_all_rays(),
    compute_all_lines, compute_mvvlva
    and compute_all_rays_from() have to be called
    in the beginning of program and tests */
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let mut board: Board = Board::set();
    let mut state: GameState = GameState::new(&board);

    board.total_occupancy();
    board.update_full_cache();
    board.count_material();

    print!("choose the color: ");
    io::stdout().flush().unwrap();

    let mut engine_side: String = String::new();
    io::stdin().read_line(&mut engine_side).unwrap();

    let mut engine: Engine = Engine {
        side: match engine_side.trim() {
            "b" => 8,
            "w" => 16,
            _ => panic!("w or b should be chosen"),
        },
        depth: 6,
        evaluation: 0,
        killer_moves: [[None; 2]; 16],
    };
    if engine.side == 16 {
        engine.depth += 1;
    }

    game_control(&mut state, &mut board, &mut engine).unwrap();

    let mut no_focus_loss: String = String::new();
    io::stdin().read_line(&mut no_focus_loss).unwrap();
}

// the main loop
fn game_control(
    state: &mut GameState,
    board: &mut Board,
    engine: &mut Engine,
) -> Result<(), io::Error> {
    loop {
        match engine.side {
            8 => {
                match make_engine_move(engine, board, state) {
                    MoveResult::Draw | MoveResult::Win => break,
                    MoveResult::None => (),
                    _ => unreachable!(),
                };
                match make_player_move(board, state, &NO_PIECE_BLACK) {
                    MoveResult::Continue => continue,
                    MoveResult::Draw | MoveResult::Win => break,
                    MoveResult::None => (),
                };
            }
            16 => {
                match make_player_move(board, state, &NO_PIECE_WHITE) {
                    MoveResult::Continue => continue,
                    MoveResult::Draw | MoveResult::Win => break,
                    MoveResult::None => (),
                };
                match make_engine_move(engine, board, state) {
                    MoveResult::Draw | MoveResult::Win => break,
                    MoveResult::None => (),
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        }
    }
    return Ok(());
}

fn make_engine_move(engine: &mut Engine, board: &mut Board, state: &mut GameState) -> MoveResult {
    board.total_occupancy();
    board.update_full_cache();
    board.count_material();

    let time: Instant = Instant::now();
    let engine_move: Option<u16> = engine.find_best_move(&board, state);
    println!("time elapsed: {:.6?}", time.elapsed());
    if let Some(m) = engine_move {
        board.perform_move(&m, state);
        println!(
            "Ferrous's move: {:?} {:?}",
            INDICES_TO_COORDS.get(&((m & FROM_MASK) as u8)).unwrap(),
            INDICES_TO_COORDS
                .get(&(((m & TO_MASK) >> TO_SHIFT) as u8))
                .unwrap(),
        );
    } else {
        if board.is_square_attacked(board.black_king.trailing_zeros() as u8, &8)
            || board.is_square_attacked(board.white_king.trailing_zeros() as u8, &16)
        {
            state.result = GameResult::BlackWins;
            println!("you won by checkmate");
            return MoveResult::Win;
        } else {
            state.result = GameResult::Draw;
            println!("stalemate");
            return MoveResult::Draw;
        }
    }
    return MoveResult::None;
}

fn make_player_move(board: &mut Board, state: &mut GameState, player_color: &u8) -> MoveResult {
    board.total_occupancy();
    board.update_full_cache();
    board.count_material();
    println!("input a move, for example: e2 e4; or with promotion: e7 e8 q");

    let mut user_move = String::new();
    io::stdin().read_line(&mut user_move).unwrap();
    let parts: Vec<&str> = user_move.trim().split_whitespace().collect();

    if parts.len() < 2 || parts.len() > 3 {
        println!("Invalid format. Use: 'e2 e4' or 'e7 e8 q'");
        return MoveResult::Continue;
    }

    let from_sq: u16 = match COORDS_TO_INDICES.get(parts[0]) {
        Some(&sq) => sq as u16,
        None => {
            println!("Invalid starting square: {}", parts[0]);
            return MoveResult::Continue;
        }
    };

    let to_sq: u16 = match COORDS_TO_INDICES.get(parts[1]) {
        Some(&sq) => sq as u16,
        None => {
            println!("Invalid destination square: {}", parts[1]);
            return MoveResult::Continue;
        }
    };

    let mut parsed_move: u16 = from_sq | (to_sq << TO_SHIFT);

    if parts.len() == 3 {
        let promo_piece: u16 = match parts[2].to_lowercase().as_str() {
            "q" => 4,
            "r" => 3,
            "b" => 2,
            "n" => 1,
            _ => {
                println!("Invalid promotion piece. Use q, r, b, or n");
                return MoveResult::Continue;
            }
        };
        parsed_move |= promo_piece << PROMOTION_SHIFT;
    }

    let mut legal_moves: Vec<u16> = board.knight_moves(&state, player_color);
    legal_moves.extend(board.bishop_moves(&state, player_color));
    legal_moves.extend(board.rook_moves(&state, player_color));
    legal_moves.extend(board.pawn_moves(&state, player_color));
    legal_moves.extend(board.queen_moves(&state, player_color));
    legal_moves.extend(board.king_moves(&state, player_color));

    if legal_moves.is_empty() {
        if board.is_square_attacked(board.black_king.trailing_zeros() as u8, &8)
            || board.is_square_attacked(board.white_king.trailing_zeros() as u8, &16)
        {
            state.result = if *player_color == NO_PIECE_WHITE {
                GameResult::BlackWins
            } else {
                GameResult::WhiteWins
            };
            println!("You are checkmated");
            return MoveResult::Win;
        } else {
            state.result = GameResult::Draw;
            println!("Stalemate");
            return MoveResult::Draw;
        }
    }

    if legal_moves.iter().any(|&mv| mv == parsed_move) {
        board.perform_move(&parsed_move, state);
        return MoveResult::None;
    } else {
        println!("Illegal move");
        return MoveResult::Continue;
    }
}
