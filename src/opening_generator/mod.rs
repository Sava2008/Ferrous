use crate::{
    converters::fen_converter::{board_to_fen, fen_to_board},
    search::Engine,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::{
    collections::{HashMap, VecDeque},
    fs::OpenOptions,
    io::BufWriter,
};

const DEFAULT_OPENING_DEPTH: u8 = 11;
const MAX_PLIES: u8 = 6; // how many plies (halfmoves) to search from STARTING_POS
const AVERAGE_MOVES_AMOUNT: usize = 2; // amount of moves per position (can be fewer)

const JSON_PATH: &'static str = "opening_book.json";

#[derive(Serialize, Deserialize)]
struct OpeningResponse {
    responses: [Option<u16>; 5], // up to 5 moves
    for_position: u64,           // zobrist hash
}

fn push_single_response(
    opening_map: &mut HashMap<u64, [Option<u16>; 5]>,
    response: OpeningResponse,
) -> () {
    if response.responses.iter().all(|r: &Option<u16>| r.is_none()) {
        panic!("reponses are all empty for {} hash", response.for_position);
    }
    opening_map.insert(response.for_position, response.responses);
}

struct BookEntry {
    fen: String,
    ply: u8,
    color: u16,
}

impl Engine {
    pub fn fill_opening_book_iterative(
        &mut self,
        opening_map: &mut HashMap<u64, [Option<u16>; 5]>,
    ) -> () {
        let mut total_nodes: u32 = 0;
        let mut stack: VecDeque<BookEntry> = VecDeque::new();

        stack.push_back(BookEntry {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            ply: 0,
            color: 8,
        });

        while let Some(entry) = stack.pop_back() {
            println!("positions explored: {total_nodes}\nposition: {}", entry.fen);

            if entry.ply >= MAX_PLIES {
                total_nodes += 1;
                continue;
            }

            let (mut board, mut state) = fen_to_board(&entry.fen);
            self.side = entry.color;
            let first_move: bool = entry.ply < 2;

            let mut best_moves = self.find_multiple_moves(
                &mut board,
                &mut state,
                DEFAULT_OPENING_DEPTH,
                if first_move { 5 } else { AVERAGE_MOVES_AMOUNT },
                entry.color,
                first_move,
            );
            let moves_len: usize = best_moves.len();
            if moves_len < 5 {
                best_moves.extend(vec![None; 5 - moves_len].into_iter());
            }

            push_single_response(
                opening_map,
                OpeningResponse {
                    responses: best_moves.clone().try_into().unwrap(),
                    for_position: Engine::rebuild_hash(&board, entry.color),
                },
            );

            let enemy_color = if entry.color == 8 { 16 } else { 8 };

            for mv in best_moves.into_iter().rev() {
                if let Some(m) = mv {
                    println!("move: {m}, color: {}, ply: {}", entry.color, entry.ply);

                    let (mut copied_board, mut copied_state) = (board.clone(), state.clone());
                    copied_board.perform_move(m, &mut copied_state, entry.color, &mut 0, &mut 0);

                    let new_fen = board_to_fen(&copied_board, &copied_state, &(enemy_color as u8));

                    stack.push_back(BookEntry {
                        fen: new_fen,
                        ply: entry.ply + 1,
                        color: enemy_color,
                    });
                }
            }
        }

        let opening_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(JSON_PATH)
            .expect("Failed to open or create the JSON file");

        to_writer_pretty(BufWriter::new(opening_file), &opening_map).expect("could not dump data");
    }
}
