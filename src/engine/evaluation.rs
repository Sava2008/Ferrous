use crate::{
    constants::{
        BISHOP_HEURISTICS, KNIGHT_HEURISTICS, OPENING_MIDDLEGAME_KING_HEURISTICS, PAWN_HEURISTICS,
        QUEEN_HEURISTICS, ROOK_HEURISTICS,
    },
    engine::Engine,
    game_logic::{Board, pieces::ChessPiece, state_enums::GameMode},
    helper_functions::reverse_idx,
};

impl Engine {
    fn locate_piece_heuristics(piece: &ChessPiece, index: usize) -> i16 {
        return match piece {
            ChessPiece::P(_) => PAWN_HEURISTICS[index],
            ChessPiece::N(_) => KNIGHT_HEURISTICS[index],
            ChessPiece::B(_) => BISHOP_HEURISTICS[index],
            ChessPiece::K(_) => OPENING_MIDDLEGAME_KING_HEURISTICS[index],
            ChessPiece::Q(_) => QUEEN_HEURISTICS[index],
            ChessPiece::R(_) => ROOK_HEURISTICS[index],
            ChessPiece::Square(_) => unreachable!(),
        };
    }

    pub fn evaluate(&mut self, board: &Board) -> () {
        match board.gamemode {
            GameMode::BlackWin => {
                self.evaluation = self.worst_possible_score;
                return ();
            }
            GameMode::WhiteWin => {
                self.evaluation = self.best_possible_score;
                return ();
            }
            _ => (),
        };
        let white_material: &i16 = &board
            .white_locations
            .values()
            .map(|idx: &usize| {
                let piece: &ChessPiece = &board.squares[*idx];
                piece.value().unwrap() as i16 + Self::locate_piece_heuristics(piece, *idx)
            })
            .sum();
        let black_material: &i16 = &board
            .black_locations
            .values()
            .map(|idx: &usize| {
                let piece: &ChessPiece = &board.squares[*idx];
                piece.value().unwrap() as i16
                    + Self::locate_piece_heuristics(piece, reverse_idx(*idx).unwrap())
            })
            .sum();

        self.evaluation = *white_material - *black_material;
    }
}
