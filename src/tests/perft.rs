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
                    "from: {}, to: {}, flag: {}, results: {:?}",
                    from_square(m),
                    to_square(m),
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

#[test]
fn run_perft() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let mut board: Board = Board::set();
    let mut state: GameState = GameState::new(&board);

    board.total_occupancy();
    board.update_full_cache();

    board.calculate_check_restrictions(&mut state, 8);
    let mut engine: Engine = Engine::new(8, 6);

    engine.evaluate(&board);
    let node_counter: NodeCounter =
        engine.perft_test(engine.depth as usize, &mut board, &mut state, 8);
    assert_eq!(
        node_counter.total_nodes,
        match engine.depth {
            1 => 20,
            2 => 400,
            3 => 8902,
            4 => 197281,
            5 => 4865609,
            6 => 119060324,
            7 => 3195901860,
            _ => unimplemented!(),
        }
    );
    if engine.depth >= 3 {
        assert_eq!(
            node_counter.captures,
            match engine.depth {
                3 => 34,
                4 => 1576,
                5 => 82719,
                6 => 2812008,
                7 => 108329926,
                _ => unimplemented!(),
            }
        );
        if engine.depth >= 5 {
            assert_eq!(
                node_counter.en_passants,
                match engine.depth {
                    5 => 258,
                    6 => 5248,
                    7 => 319617,
                    _ => unimplemented!(),
                }
            );
            if engine.depth == 7 {
                assert_eq!(node_counter.castlings, 883453);
            }
        }
    }

    let (mut pos3_board, mut pos3_state) =
        fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    pos3_board.total_occupancy();
    pos3_board.update_full_cache();

    let mut engine: Engine = Engine::new(8, 10);

    engine.evaluate(&pos3_board);

    for d in 1..8 {
        println!("depth {d}");
        engine.depth = d as u8;
        let node_counter: NodeCounter = engine.perft_test(d, &mut pos3_board, &mut pos3_state, 8);
        let (correct_nodes, correct_captures, correct_e_p, correct_promotions) = match d {
            1 => (14, 1, 0, 0),
            2 => (191, 14, 0, 0),
            3 => (2812, 209, 2, 0),
            4 => (43238, 3348, 123, 0),
            5 => (674624, 52051, 1165, 0),
            6 => (11030083, 940350, 33325, 7552),
            7 => (178633661, 14519036, 294874, 140024),
            _ => unreachable!(),
        };
        assert_eq!(node_counter.captures, correct_captures, "depth {}", d);
        assert_eq!(node_counter.promotions, correct_promotions, "depth {}", d);
        assert_eq!(node_counter.en_passants, correct_e_p, "depth {}", d);
        assert_eq!(node_counter.total_nodes, correct_nodes, "depth {}", d);
        assert_eq!(node_counter.castlings, 0, "depth {}", d);
        //println!("{:?}", node_counter);
    }
}
