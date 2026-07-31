use crate::{
    converters::fen_converter::{board_to_fen, fen_to_board},
    search::Engine,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::{collections::HashMap, fs::OpenOptions, io::BufWriter};

const DEFAULT_OPENING_DEPTH: u8 = 6;
const STARTING_POS: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const MAX_PLIES: u8 = 6; // how many plies (halfmoves) to search from STARTING_POS
const AVERAGE_MOVES_AMOUNT: usize = 3;

const JSON_PATH: &'static str = "opening_book.json";

#[derive(Serialize, Deserialize)]
struct OpeningResponse {
    responses: [Option<u16>; AVERAGE_MOVES_AMOUNT], // up to 5 moves
    for_position: u64,                              // zobrist hash
}

struct SearchProgress {
    pub current_ply: u8,
    pub current_color: u16,
    pub current_position: String,
    pub current_hash: u64,
    pub approximate_total_positions: u16,
    pub positions_explored: u16,
    pub percent_finished: f64,
}

impl SearchProgress {
    fn new() -> Self {
        return Self {
            current_ply: 0,
            current_color: 8,
            current_position: STARTING_POS.to_string(),
            current_hash: 0,
            approximate_total_positions: 0,
            positions_explored: 0,
            percent_finished: 0.,
        };
    }
}

fn push_single_response(
    opening_map: &mut HashMap<u64, [Option<u16>; AVERAGE_MOVES_AMOUNT]>,
    response: OpeningResponse,
) -> () {
    if response.responses.iter().all(|r: &Option<u16>| r.is_none()) {
        panic!("reponses are all empty for {} hash", response.for_position);
    }
    opening_map.insert(response.for_position, response.responses);
}

impl Engine {
    pub fn fill_opening_book(
        &mut self,
        opening_map: &mut HashMap<u64, [Option<u16>; AVERAGE_MOVES_AMOUNT]>,
        current_ply: u8,
        total_nodes: &mut u32,
        current_fen: &str,
        current_color: u16,
    ) -> () {
        print!("\x1B[2J\x1B[1;1H");
        println!("positions explored: {total_nodes}\nposition: {current_fen}");
        *total_nodes += 1;

        if current_ply >= MAX_PLIES {
            return;
        }

        let (mut board, mut state) = fen_to_board(&current_fen);
        self.side = current_color;

        let best_moves = self.find_multiple_moves(
            &mut board,
            &mut state,
            DEFAULT_OPENING_DEPTH,
            AVERAGE_MOVES_AMOUNT,
            if current_ply % 2 == 0 { 8 } else { 16 },
        );

        push_single_response(
            opening_map,
            OpeningResponse {
                responses: best_moves.clone().try_into().unwrap(),
                for_position: Engine::rebuild_hash(&board, current_color),
            },
        );

        let enemy_color = if current_color == 8 { 16 } else { 8 };

        for mv in best_moves {
            if let Some(m) = mv {
                println!("move: {m}, color: {current_color}, ply: {current_ply}");
                let (mut copied_board, mut copied_state) = (board.clone(), state.clone());
                copied_board.perform_move(m, &mut copied_state, current_color, &mut 0, &mut 0);

                self.fill_opening_book(
                    opening_map,
                    current_ply + 1,
                    total_nodes,
                    &board_to_fen(&copied_board, &copied_state, &(enemy_color as u8)),
                    enemy_color,
                );
            }
        }

        if current_ply == 0 {
            let opening_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(JSON_PATH)
                .expect("Failed to open or create the JSON file");

            to_writer_pretty(BufWriter::new(opening_file), &opening_map)
                .expect("could not dump data");
        }
    }
}
