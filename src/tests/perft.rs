#[allow(unused)]
use crate::{
    board::Board,
    board_geometry_templates::{MARK_MASK, MARK_SHIFT, from_square, to_square},
    constants::attacks::*,
    converters::fen_converter::fen_to_board,
    gamestate::GameState,
    moves::MoveList,
    search::Engine,
    transposition::TranspositionTable,
};

#[derive(Clone, Copy, Debug)]
pub struct NodeCounter {
    pub total_nodes: u64,
    pub captures: u64,
    pub promotions: u64,
    pub en_passants: u64,
    pub castlings: u64,
}

impl Engine {
    pub fn perft_test(
        &mut self,
        depth: usize,
        board: &mut Board,
        state: &mut GameState,
        color: u16,
    ) -> NodeCounter {
        if depth == 0 {
            return NodeCounter {
                total_nodes: 1,
                captures: 0,
                promotions: 0,
                en_passants: 0,
                castlings: 0,
            };
        }

        self.generate_pseudo_legal_moves(color, board, state, depth, false);
        let opponent: u16 = if color == 8 { 16 } else { 8 };

        let mut node_counter: NodeCounter = NodeCounter {
            total_nodes: 0,
            captures: 0,
            promotions: 0,
            en_passants: 0,
            castlings: 0,
        };

        let move_count: usize = self.move_lists[depth].first_not_occupied;
        for i in 0..move_count {
            let m: u16 = self.move_lists[depth].pseudo_moves[i];
            let flag: u16 = (m & MARK_MASK) >> MARK_SHIFT;

            let is_capture: bool = board.is_capture(m);
            let is_en_passant: bool = flag == 2;
            let is_castle: bool = flag == 1;
            let is_promotion: bool = flag >= 3 && flag <= 6;

            board.perform_move(m, state, color, &mut 0, &mut 0);

            if depth == self.depth as usize {
                println!(
                    " {}{}, flag: {}, results: {:?}",
                    INDICES_TO_COORDS.get(&from_square(m)).unwrap(),
                    INDICES_TO_COORDS.get(&(to_square(m) as u8)).unwrap(),
                    (m & MARK_MASK) >> MARK_SHIFT,
                    self.perft_divide(depth - 1, board, state, opponent)
                );
            }
            if depth == 1 {
                node_counter.total_nodes += 1;
                if is_capture {
                    node_counter.captures += 1;
                }
                if is_en_passant {
                    node_counter.en_passants += 1;
                    node_counter.captures += 1;
                }
                if is_castle {
                    node_counter.castlings += 1;
                }
                if is_promotion {
                    node_counter.promotions += 1;
                }
            } else {
                let sub_node_counter: NodeCounter =
                    self.perft_divide(depth - 1, board, state, opponent);
                node_counter.total_nodes += sub_node_counter.total_nodes;
                node_counter.captures += sub_node_counter.captures;
                node_counter.en_passants += sub_node_counter.en_passants;
                node_counter.castlings += sub_node_counter.castlings;
                node_counter.promotions += sub_node_counter.promotions;
            }

            board.cancel_move(state, color, &mut 0, &mut 0);
        }

        return node_counter.clone();
    }
    #[allow(unused)]
    fn perft_divide(
        &mut self,
        depth: usize,
        board: &mut Board,
        state: &mut GameState,
        color: u16,
    ) -> NodeCounter {
        if depth == 0 {
            return NodeCounter {
                total_nodes: 1,
                captures: 0,
                promotions: 0,
                en_passants: 0,
                castlings: 0,
            };
        }

        self.generate_pseudo_legal_moves(color, board, state, depth, false);
        let opponent: u16 = if color == 8 { 16 } else { 8 };

        let mut node_counter: NodeCounter = NodeCounter {
            total_nodes: 0,
            captures: 0,
            promotions: 0,
            en_passants: 0,
            castlings: 0,
        };

        let move_count: usize = self.move_lists[depth].first_not_occupied;
        for i in 0..move_count {
            let m: u16 = self.move_lists[depth].pseudo_moves[i];

            let flag: u16 = (m & MARK_MASK) >> MARK_SHIFT;

            let is_capture: bool = board.is_capture(m);
            let is_en_passant: bool = flag == 2;
            let is_castle: bool = flag == 1;
            let is_promotion: bool = flag >= 3 && flag <= 6;

            board.perform_move(m, state, color, &mut 0, &mut 0);
            if board.is_square_attacked(
                if color == 8 {
                    board.white_king_square
                } else {
                    board.black_king_square
                },
                opponent,
            ) {
                board.cancel_move(state, color, &mut 0, &mut 0);
                continue;
            }
            // println!(
            //     "depth: {depth}; move: {}{}, {}",
            //     INDICES_TO_COORDS.get(&from_square(m)).unwrap(),
            //     INDICES_TO_COORDS.get(&(to_square(m) as u8)).unwrap(),
            //     (m & MARK_MASK) >> MARK_SHIFT
            // );

            let king_sq: u8 = if color == 8 {
                board.white_king_square
            } else {
                board.black_king_square
            };

            if depth == 1 {
                node_counter.total_nodes += 1;
                if is_capture {
                    node_counter.captures += 1;
                }
                if is_en_passant {
                    node_counter.en_passants += 1;
                    node_counter.captures += 1;
                }
                if is_castle {
                    node_counter.castlings += 1;
                }
                if is_promotion {
                    node_counter.promotions += 1;
                }
            } else {
                let sub_node_counter: NodeCounter =
                    self.perft_divide(depth - 1, board, state, opponent);
                node_counter.total_nodes += sub_node_counter.total_nodes;
                node_counter.captures += sub_node_counter.captures;
                node_counter.en_passants += sub_node_counter.en_passants;
                node_counter.castlings += sub_node_counter.castlings;
                node_counter.promotions += sub_node_counter.promotions;
            }

            board.cancel_move(state, color, &mut 0, &mut 0);
        }

        return node_counter.clone();
    }
}

pub fn run_perft(mut board: Board, mut state: GameState, depth: u8) -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    board.total_occupancy();
    board.update_full_cache();

    let color: u16 = state.whose_turn;
    board.calculate_check_restrictions(&mut state, color);
    let mut engine: Engine = Engine::new(color, depth);

    engine.evaluate(&board);
    let node_counter: NodeCounter =
        engine.perft_test(engine.depth as usize, &mut board, &mut state, color);
    println!("total nodes: {node_counter:?}");
}
