use crate::{
    board::Board,
    board_geometry_templates::{
        CAPTURED_PIECE_TYPE_SHIFT, CASTLING_SHIFT, COLORLESS_KING, EN_PASSANT_SHIFT, FROM_MASK,
        MOVING_PIECE_TYPE_SHIFT, NO_PIECE_BLACK, NO_PIECE_WHITE, PROMOTION_SHIFT, TO_MASK,
        TO_SHIFT, moving_piece_type, promotion,
    },
    constants::attacks::{
        COORDS_TO_INDICES, INDICES_TO_COORDS, compute_all_lines, compute_all_rays,
        compute_all_rays_from, compute_mvvlva, initialize_sliding_attack_tables,
    },
    enums::GameResult,
    gamestate::GameState,
    moves::MoveList,
    search::Engine,
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
        move_scores: [[0; 192]; 32],
        quiescence_limitation: 9,
        current_hash: 0,
    };
    if (engine.side == 16 && engine.depth % 2 == 1) || (engine.side == 8 && engine.depth % 2 == 0) {
        engine.quiescence_limitation -= 1;
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
                        match make_player_move(board, state, NO_PIECE_BLACK) {
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
    color: u32,
) -> MoveResult {
    board.total_occupancy();

    let t: Instant = Instant::now();
    let engine_move: Option<u32> = engine.find_best_move(&board, state);
    println!("time: {:.3?}", t.elapsed());
    if let Some(m) = engine_move {
        board.perform_move(m, state, color, &mut engine.evaluation, &mut 0);
        println!(
            "Ferrous's move: {:?} {:?} {}",
            INDICES_TO_COORDS.get(&((m & FROM_MASK) as u8)).unwrap(),
            INDICES_TO_COORDS
                .get(&(((m & TO_MASK) >> TO_SHIFT) as u8))
                .unwrap(),
            match promotion(m) {
                0 => "",
                1 => "n",
                2 => "b",
                3 => "r",
                4 => "q",
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

fn make_player_move(board: &mut Board, state: &mut GameState, player_color: u32) -> MoveResult {
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

    let mut user_move = String::new();
    io::stdin().read_line(&mut user_move).unwrap();
    let parts: Vec<&str> = user_move.trim().split_whitespace().collect();

    if parts.len() < 2 || parts.len() > 3 {
        println!("Invalid format. Use: 'e2 e4' or 'e7 e8 q'");
        return MoveResult::Continue;
    }

    let from_sq: u32 = match COORDS_TO_INDICES.get(parts[0]) {
        Some(&sq) => sq as u32,
        None => {
            println!("Invalid starting square: {}", parts[0]);
            return MoveResult::Continue;
        }
    };

    let to_sq: u32 = match COORDS_TO_INDICES.get(parts[1]) {
        Some(&sq) => sq as u32,
        None => {
            println!("Invalid destination square: {}", parts[1]);
            return MoveResult::Continue;
        }
    };

    let mut parsed_move: u32 = from_sq | (to_sq << TO_SHIFT);
    parsed_move |= board.piece_at(from_sq) << MOVING_PIECE_TYPE_SHIFT;
    let p: u32 = board.piece_at(to_sq);
    if p != 0 {
        parsed_move |= p << CAPTURED_PIECE_TYPE_SHIFT;
    }
    if moving_piece_type(parsed_move) == COLORLESS_KING
        && std::cmp::max(from_sq, to_sq) - std::cmp::min(from_sq, to_sq) > 1
    {
        parsed_move |= 1 << CASTLING_SHIFT;
    }
    if let Some(e_p) = state.en_passant_target {
        if (e_p as u32) == to_sq {
            parsed_move |= 1 << EN_PASSANT_SHIFT;
        }
    }

    if parts.len() == 3 {
        let promo_piece: u32 = match parts[2].to_lowercase().as_str() {
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
