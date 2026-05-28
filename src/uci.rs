use std::io::BufRead;

use crate::{
    board::Board,
    board_geometry_templates::{FROM_MASK, MARK_MASK, MARK_SHIFT, TO_MASK, TO_SHIFT},
    constants::attacks::INDICES_TO_COORDS,
    gamestate::GameState,
    search::Engine,
};

pub fn uci_output(engine: &mut Engine, board: &Board, state: &mut GameState) -> () {
    let input: std::io::Lines<std::io::StdinLock<'_>> = std::io::stdin().lock().lines();
    for l in input {
        match l.unwrap().as_str() {
            "uci" => println!("uciok"),
            "isready" => println!("readyok"),
            "quit" => break,
            "go" => {
                let engine_move: u16 = engine.find_best_move(board, state, false).unwrap();
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
                );
                println!("bestmove {}", uci_move_string.as_str());
            }
            _ => unimplemented!(),
        }
    }
}
