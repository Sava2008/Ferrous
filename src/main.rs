use crate::{
    board::Board,
    board_geometry_templates::*,
    constants::attacks::{
        COORDS_TO_INDICES, INDICES_TO_COORDS, compute_all_lines, compute_all_rays,
        compute_all_rays_from, compute_mvvlva, initialize_sliding_attack_tables,
    },
    enums::GameResult,
    gamestate::GameState,
    moves::MoveList,
    search::Engine,
    transposition::TranspositionTable,
};
use std::{
    io::{self, Write},
    time::Instant,
};
pub mod board;
pub mod board_geometry_templates;
pub mod constants;
pub mod converters;
pub mod enums;
pub mod gamestate;
pub mod move_generation;
pub mod move_make_unmake;
pub mod moves;
pub mod search;
pub mod tests;
pub mod transposition;
pub mod tuning;

enum MoveResult {
    Win,
    Draw,
    Continue,
    None,
}

#[allow(unused)]
fn test_speed() -> () {
    let mut board: Board = Board::set();
    let mut state: GameState = GameState::new(&board);

    board.total_occupancy();
    board.update_full_cache();

    let piece_move: u16 = 12 | (28 << TO_SHIFT);
    let t: Instant = Instant::now();
    board.perform_move(piece_move, &mut state, 8, &mut 0, &mut 0);
    println!("perform_move time: {:?}", t.elapsed().as_micros());
    let t = Instant::now();
    board.cancel_move(&mut state, 8, &mut 0, &mut 0);
    println!("cancelation time: {:?}", t.elapsed().as_micros());

    let mut engine: Engine = Engine {
        side: 8,
        depth: 11,
        evaluation: 0,
        killer_moves: [[None; 2]; 32],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 32],
        history_heuristics: [0; 4096],
        move_scores: [[0; 192]; 32],
        quiescence_limitation: 9,
        current_hash: 0,
        transposition_table: TranspositionTable::new(),
    };
    let t = Instant::now();
    engine.alpha_beta_pruning(&mut board, 1, 4, 1, true, &mut state, &mut 0);
    println!("alpha beta time: {}", t.elapsed().as_micros());
    let t = Instant::now();
    engine.score_all_moves(1, 1, &0, &board.cached_pieces);
    println!("scoring time: {}", t.elapsed().as_micros());
    let t = Instant::now();
    engine.generate_pseudo_legal_moves(8, &board, &state, 1, false);
    println!("movegen time: {}", t.elapsed().as_micros());
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
    println!("board: {:?}", board.cached_pieces);

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
        depth: 9,
        evaluation: 0,
        killer_moves: [[None; 2]; 32],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 32],
        history_heuristics: [0; 4096],
        move_scores: [[0; 192]; 32],
        quiescence_limitation: 9,
        current_hash: 0,
        transposition_table: TranspositionTable::new(),
    };
    if (engine.side == 16 && engine.depth % 2 == 1) || (engine.side == 8 && engine.depth % 2 == 0) {
        engine.quiescence_limitation -= 1;
    }
    engine.evaluate(&board);

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
    'outer: loop {
        match engine.side {
            8 => {
                if state.whose_turn == 8 {
                    match make_engine_move(engine, board, state, engine.side) {
                        MoveResult::Draw | MoveResult::Win => break 'outer,
                        MoveResult::None => (),
                        _ => unreachable!(),
                    };
                    state.whose_turn = 16;
                } else {
                    loop {
                        match make_player_move(board, state, 16) {
                            MoveResult::Continue => continue,
                            MoveResult::Draw | MoveResult::Win => break 'outer,
                            MoveResult::None => break,
                        };
                    }
                    state.whose_turn = 8;
                }
            }
            16 => {
                if state.whose_turn == 8 {
                    loop {
                        match make_player_move(board, state, NO_PIECE_WHITE) {
                            MoveResult::Continue => continue,
                            MoveResult::Draw | MoveResult::Win => break 'outer,
                            MoveResult::None => break,
                        };
                    }
                    state.whose_turn = 16;
                } else {
                    match make_engine_move(engine, board, state, engine.side) {
                        MoveResult::Draw | MoveResult::Win => break,
                        MoveResult::None => (),
                        _ => unreachable!(),
                    };
                    state.whose_turn = 8;
                }
            }
            _ => unreachable!(),
        }
    }
    return Ok(());
}

fn make_engine_move(
    engine: &mut Engine,
    board: &mut Board,
    state: &mut GameState,
    color: u16,
) -> MoveResult {
    board.total_occupancy();

    let t: Instant = Instant::now();
    let engine_move: Option<u16> = engine.find_best_move(&board, state, false);
    println!("time: {:.3?}", t.elapsed());
    if let Some(m) = engine_move {
        board.perform_move(m, state, color, &mut engine.evaluation, &mut 0);
        println!(
            "Ferrous's move: {:?} {:?} {}",
            INDICES_TO_COORDS.get(&((m & FROM_MASK) as u8)).unwrap(),
            INDICES_TO_COORDS
                .get(&(((m & TO_MASK) >> TO_SHIFT) as u8))
                .unwrap(),
            match (m & MARK_MASK) >> MARK_SHIFT {
                0..3 => "",
                3 => "n",
                4 => "b",
                5 => "r",
                6 => "q",
                _ => unreachable!(),
            }
        );
    } else {
        if board.is_square_attacked(board.black_king_square, 8)
            || board.is_square_attacked(board.white_king_square, 16)
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

fn make_player_move(board: &mut Board, state: &mut GameState, player_color: u16) -> MoveResult {
    let mut legal_moves: MoveList = MoveList {
        pseudo_moves: [0; 192],
        first_not_occupied: 0,
    };
    board.total_occupancy();
    board.knight_moves(player_color, &mut legal_moves, false);
    board.bishop_moves(player_color, &mut legal_moves, false);
    board.rook_moves(player_color, &mut legal_moves, false);
    board.pawn_moves(&state, player_color, &mut legal_moves, false);
    board.queen_moves(player_color, &mut legal_moves, false);
    board.king_moves(&state, player_color, &mut legal_moves, false);
    if legal_moves.pseudo_moves.iter().all(|m| *m == 0) {
        if board.is_square_attacked(board.black_king_square, 8)
            || board.is_square_attacked(board.white_king_square, 16)
        {
            state.result = if player_color == NO_PIECE_WHITE {
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
    println!("input a move, for example: e2 e4; or with promotion: e7 e8 q");

    let mut user_move: String = String::new();
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

    let moving_piece = &board.cached_pieces[from_sq as usize];

    if moving_piece == &WHITE_KING_U16
        || moving_piece == &BLACK_KING_U16
            && std::cmp::max(from_sq, to_sq) - std::cmp::min(from_sq, to_sq) > 1
    {
        parsed_move |= 1 << MARK_SHIFT;
    }
    if let Some(e_p) = state.en_passant_target
        && (moving_piece == &WHITE_PAWN_U16 || moving_piece == &BLACK_PAWN_U16)
    {
        if (e_p as u16) == to_sq {
            parsed_move |= 2 << MARK_SHIFT;
        }
    }

    if parts.len() == 3 {
        let promo_piece: u16 = match parts[2].to_lowercase().as_str() {
            "q" => 6,
            "r" => 5,
            "b" => 4,
            "n" => 3,
            _ => {
                println!("Invalid promotion piece. Use q, r, b, or n");
                return MoveResult::Continue;
            }
        };
        parsed_move |= promo_piece << MARK_SHIFT;
    }

    if legal_moves.pseudo_moves.iter().any(|&mv| {
        mv & FROM_MASK == parsed_move & FROM_MASK
            && (mv & TO_MASK) >> TO_SHIFT == (parsed_move & TO_MASK) >> TO_SHIFT
    }) {
        board.perform_move(parsed_move, state, player_color, &mut 0, &mut 0);
        return MoveResult::None;
    } else {
        println!("Illegal move, not in pseudo legal moves");
        return MoveResult::Continue;
    }
}
/*
[1, 2, 3, 5, 6, 3, 2, 4,
1, 1, 1, 0, 1, 1, 1, 1,
0, 0, 0, 0, 0, 0, 0, 0,
0, 9, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
7, 7, 7, 7, 0, 7, 7, 7,
10, 8, 9, 11, 12, 0, 8, 10] */
