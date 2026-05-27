use crate::{
    board::Board,
    board_geometry_templates::{MARK_MASK, MARK_SHIFT},
    constants::attacks::*,
    gamestate::GameState,
    moves::MoveList,
    search::Engine,
    transposition::TranspositionTable,
};

impl Engine {
    pub fn perft_test(
        &mut self,
        depth: usize,
        board: &mut Board,
        state: &mut GameState,
        color: u16,
    ) -> (u64, u64, u64, u64, u64) {
        if depth == 0 {
            return (1, 0, 0, 0, 0);
        }

        self.generate_pseudo_legal_moves(color, board, state, depth, false);
        let opponent: u16 = if color == 8 { 16 } else { 8 };

        let mut total: u64 = 0;
        let mut captures: u64 = 0;
        let mut en_passants: u64 = 0;
        let mut castles: u64 = 0;
        let mut promotions: u64 = 0;

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

            if !board.is_square_attacked(king_sq, opponent) {
                if depth == 1 {
                    total += 1;
                    if is_capture {
                        captures += 1;
                    }
                    if is_en_passant {
                        en_passants += 1;
                        captures += 1;
                    }
                    if is_castle {
                        castles += 1;
                    }
                    if is_promotion {
                        promotions += 1;
                    }
                } else {
                    let (sub_total, sub_captures, sub_ep, sub_castles, sub_promos) =
                        self.perft_test(depth - 1, board, state, opponent);
                    total += sub_total;
                    captures += sub_captures;
                    en_passants += sub_ep;
                    castles += sub_castles;
                    promotions += sub_promos;
                }
            }

            board.cancel_move(state, color, &mut 0, &mut 0);
        }

        return (total, captures, en_passants, castles, promotions);
    }
}

pub fn run_perft() -> () {
    initialize_sliding_attack_tables();
    compute_all_rays();
    compute_all_rays_from();
    compute_all_lines();
    compute_mvvlva();

    let mut board: Board = Board::set();
    let mut state: GameState = GameState::new(&board);

    board.total_occupancy();
    board.update_full_cache();

    let mut engine: Engine = Engine {
        side: 8,
        depth: 6,
        evaluation: 0,
        killer_moves: [[None; 2]; 32],
        move_lists: [MoveList {
            pseudo_moves: [0; 192],
            first_not_occupied: 0,
        }; 32],
        history_heuristics: [0; 4096],
        move_scores: [[0; 192]; 32],
        quiescence_limitation: 9,
        current_hash: 0,
        transposition_table: TranspositionTable::new(),
    };

    engine.evaluate(&board);
    let (total, captures, en_passants, castles, _promotions) =
        engine.perft_test(engine.depth as usize, &mut board, &mut state, 8);
    if engine.depth >= 3 {
        assert_eq!(
            captures,
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
                en_passants,
                match engine.depth {
                    5 => 258,
                    6 => 5248,
                    7 => 319617,
                    _ => unimplemented!(),
                }
            );
            if engine.depth == 7 {
                assert_eq!(castles, 883453);
            }
        }
    }
    assert_eq!(
        total,
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
}
