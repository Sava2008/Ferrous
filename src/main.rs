use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::attacks::{COORDS_TO_INDICES, INDICES_TO_COORDS, initialize_sliding_attack_tables},
    enums::{GameResult, PieceColor, PieceType},
    gamestate::{GameState, PieceMove},
};
use std::io;
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
    let mut engine: Engine = Engine {
        side: PieceColor::White,
        depth: 4,
        evaluation: 0,
    };
    game_control(&mut state, &mut board, &mut engine).unwrap();
}

// the main loop
fn game_control(
    state: &mut GameState,
    board: &mut Board,
    engine: &mut Engine,
) -> Result<(), io::Error> {
    loop {
        board.total_occupancy();
        state.check_info.update(&board, &PieceColor::White);
        state.pin_info.update(&board, &PieceColor::White);
        state.update_check_constraints(&board);

        // white's move
        let engine_move: Option<PieceMove> = engine.find_best_move(&board, state);
        if let Some(m) = engine_move {
            board.perform_move(&m, state);
            println!(
                "Ferrous's move: {:?} {:?}",
                INDICES_TO_COORDS.get(&m.from).unwrap(),
                INDICES_TO_COORDS.get(&m.to).unwrap(),
            );
        } else {
            println!("game ended");
            break;
        }
        println!("board: {board:?}");

        board.total_occupancy();
        state.check_info.update(&board, &PieceColor::Black);
        state.pin_info.update(&board, &PieceColor::Black);
        state.update_check_constraints(&board);

        // black's move

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
        let piece: (PieceColor, PieceType) = if let Some(m) = board.bitboard_contains(piece_move[0])
        {
            m
        } else {
            println!("no piece stands on the starting square");
            continue;
        };

        if piece.0 == engine.side {
            println!("the starting square is occupied by your opponent's piece");
            continue;
        }

        let legal_moves: Vec<PieceMove> = (match piece.1 {
            PieceType::Knight => board.knight_moves(&state, &piece.0),
            PieceType::Bishop => board.bishop_moves(&state, &piece.0),
            PieceType::Rook => board.rook_moves(&state, &piece.0),
            PieceType::Pawn => board.pawn_moves(&state, &piece.0),
            PieceType::Queen => board.queen_moves(&state, &piece.0),
            PieceType::King => board.king_moves(&state, &piece.0),
        })
        .into_iter()
        .filter(|p_m: &PieceMove| p_m.from == piece_move[0])
        .collect();

        println!("legal_moves = {legal_moves:?}, piece_move = {piece_move:?}");

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

        match state.result {
            GameResult::Going => continue,
            _ => break,
        };
    }
    return Ok(());
}
