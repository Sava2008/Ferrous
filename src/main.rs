use crate::{
    alpha_beta_pruning::Engine,
    board::Board,
    constants::attacks::{INDICES_TO_COORDS, initialize_sliding_attack_tables},
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
    let mut state: GameState = GameState::new();
    let mut engine: Engine = Engine {
        side: PieceColor::White,
        best_possible_score: i32::MAX,
        worst_possible_score: i32::MIN,
        depth: 6,
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
        board.white_occupancy();
        board.black_occupancy();
        board.total_occupancy();
        println!("input a move, for example:\ne2 e4");
        let mut user_move: String = String::with_capacity(7);
        io::stdin().read_line(&mut user_move)?;

        if user_move.len() != 7 {
            println!("the move should contain a starting square and the destination square");
            continue;
        }
        let piece_move: Vec<u8> = user_move
            .split_whitespace()
            .filter_map(|pos| INDICES_TO_COORDS.get(pos).cloned())
            .collect::<Vec<u8>>();
        let piece = if let Some(m) = board.bitboard_contains(piece_move[0]) {
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
            PieceType::Knight => board.knight_moves(&state, &piece.0).unwrap(),
            PieceType::Bishop => board.bishop_moves(&state, &piece.0).unwrap(),
            PieceType::Rook => board.rook_moves(&state, &piece.0).unwrap(),
            PieceType::Pawn => board.pawn_moves(&state, &piece.0).unwrap(),
            PieceType::Queen => board.queen_moves(&state, &piece.0).unwrap(),
            PieceType::King => board.king_moves(&state, &piece.0).unwrap(),
        })
        .into_iter()
        .filter(|p_m: &PieceMove| p_m.from == piece_move[0])
        .collect();
        println!("{:?}", board.queen_moves(&state, &piece.0).unwrap());

        println!("legal_moves = {legal_moves:?}, piece_move = {piece_move:?}");

        if legal_moves.iter().any(|mv| mv.to == piece_move[1]) {
            board.perform_move(PieceMove {
                from: piece_move[0],
                to: piece_move[1],
            });
        } else {
            println!("illegal move");
            continue;
        }

        println!("board = {:?}", board);

        match state.result {
            GameResult::Going => continue,
            _ => break,
        };
    }
    return Ok(());
}
