use std::{
    io::BufRead,
    time::{Duration, Instant},
};

use crate::{
    board::Board,
    board_geometry_templates::{FROM_MASK, MARK_MASK, MARK_SHIFT, TO_MASK, TO_SHIFT},
    constants::attacks::INDICES_TO_COORDS,
    converters::fen_converter::fen_to_board,
    gamestate::GameState,
    search::Engine,
    tests,
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

            _ => {
                let mut perft: bool = false;
                if command.starts_with("position fen ") {
                    command.split_whitespace().next();
                    let fen_position: &str = command.strip_prefix("position fen ").unwrap();
                    let (raw_board, raw_state) = fen_to_board(fen_position);
                    engine.side = raw_state.whose_turn;
                    (board, state) = (Some(raw_board), Some(raw_state));
                } else if command.starts_with("go") {
                    let engine_move: u16 = if let Some(mut b) = board.clone()
                        && let Some(mut s) = state.clone()
                    {
                        b.total_occupancy();
                        b.update_full_cache();
                        let mut split_command: std::str::SplitWhitespace<'_> =
                            command.split_whitespace();
                        split_command.next();
                        let (restriction, amount) = (split_command.next(), split_command.next());
                        let (max_depth, time_constrainst) = if let Some(r) = restriction
                            && let Some(a) = amount
                        {
                            match r {
                                "depth" => (a.parse().unwrap(), Duration::from_mins(10)),
                                "movetime" => (64, Duration::from_millis(a.parse().unwrap())),
                                "perft" => {
                                    perft = true;
                                    (a.parse().unwrap(), Duration::from_mins(10))
                                }
                                _ => unimplemented!(),
                            }
                        } else {
                            (10, Duration::from_mins(10))
                        };
                        if !perft {
                            engine.depth = max_depth;
                            let start_time: Instant = Instant::now();
                            let engine_move: u16 = engine
                                .find_best_move(
                                    b.clone(),
                                    s.clone(),
                                    time_constrainst,
                                    max_depth as usize,
                                )
                                .unwrap();
                            println!("time spent: {}", start_time.elapsed().as_millis());
                            engine_move
                        } else {
                            let color = s.whose_turn;
                            b.calculate_check_restrictions(&mut s, color);
                            tests::perft::run_perft(b, s, max_depth);
                            0
                        }
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
                            0..3 | 7..10 => "",
                            3 | 10 => "n",
                            4 | 11 => "b",
                            5 | 12 => "r",
                            6 | 13 => "q",
                            _ => unreachable!(),
                        },
                    )
                    .replace("\"", "");
                    if !perft {
                        println!("bestmove {}", uci_move_string.as_str());
                    }
                } else {
                    unimplemented!();
                }
            }
        }
    }
}
