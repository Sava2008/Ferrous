use std::io::BufRead;

use crate::{
    board::Board,
    board_geometry_templates::{FROM_MASK, MARK_MASK, MARK_SHIFT, TO_MASK, TO_SHIFT},
    constants::attacks::INDICES_TO_COORDS,
    converters::fen_converter::fen_to_board,
    gamestate::GameState,
    search::Engine,
    transposition::TranspositionTable,
};

pub fn uci_output(engine: &mut Engine) -> () {
    let input: std::io::Lines<std::io::StdinLock<'_>> = std::io::stdin().lock().lines();
    let (mut board, mut state): (Option<Board>, Option<GameState>) = (None, None);
    for l in input {
        let string_command: String = l.unwrap();
        let command: &str = string_command.as_str();
        match command {
            "uci" => println!("uciok"),
            "ucinewgame" => {
                for i in 0..engine.history_heuristics.len() {
                    engine.history_heuristics[i] = 0;
                }
                engine.transposition_table = TranspositionTable::new();
            }
            "isready" => println!("readyok"),
            "quit" => break,
            "go" => {
                let engine_move: u16 = if let Some(mut b) = board.clone()
                    && let Some(mut s) = state.clone()
                {
                    b.total_occupancy();
                    b.update_full_cache();
                    engine.find_best_move(&b, &mut s, false).unwrap()
                } else {
                    panic!("uninitialized board");
                };
                let uci_move_string: String = format!(
                    "{:?}{:?}{}",
                    INDICES_TO_COORDS
                        .get(&((engine_move & FROM_MASK) as u8))
                        .unwrap(),
                    INDICES_TO_COORDS
                        .get(&(((engine_move & TO_MASK) >> TO_SHIFT) as u8))
                        .unwrap(),
                    match (engine_move & MARK_MASK) >> MARK_SHIFT {
                        0..3 => "",
                        3 => "n",
                        4 => "b",
                        5 => "r",
                        6 => "q",
                        _ => unreachable!(),
                    },
                )
                .replace("\"", "");
                println!("bestmove {}", uci_move_string.as_str());
            }
            _ => {
                if command.starts_with("position fen ") {
                    command.split_whitespace().next();
                    let fen_position: &str = command.strip_prefix("position fen ").unwrap();
                    let (raw_board, raw_state) = fen_to_board(fen_position);
                    engine.side = raw_state.whose_turn;
                    (board, state) = (Some(raw_board), Some(raw_state));
                } else {
                    unimplemented!();
                }
            }
        }
    }
}
