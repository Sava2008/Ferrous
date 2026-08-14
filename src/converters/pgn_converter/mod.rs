/*
[Event "rated rapid game"]
[Site "https://lichess.org/cYTQ97Sn"]
[Date "2026.08.05"]
[Round "-"]
[White "ibrohimbobo"]
[Black "Sawa2008"]
[Result "1-0"]
[GameId "cYTQ97Sn"]
[UTCDate "2026.08.05"]
[UTCTime "07:32:53"]
[WhiteElo "2110"]
[BlackElo "2057"]
[WhiteRatingDiff "+4"]
[BlackRatingDiff "-15"]
[Variant "Standard"]
[TimeControl "600+0"]
[ECO "B00"]
[Opening "Pirc Defense"]
[Termination "Normal"]
[Annotator "lichess.org"]

1. e4 d6 { B00 Pirc Defense } 2. Nf3 Nf6 3. Nc3 Bg4 4. d4 c6 5. h3 Bh5 6. g4 Bg6 7. Bd3 e6 8. Nh4 d5 9. Nxg6 hxg6 10. e5 Nfd7 11. Qf3 Be7 12. Bf4 a6 13. O-O-O c5 14. dxc5 Nc6 15. Na4 Ndxe5 16. Bxe5 Nxe5 17. Qe2 Nxd3+ 18. Rxd3 Qc7 19. Qe3 Rd8 20. Rhd1 Qc6 21. b3 Rd7 22. Kb1 Rc7 23. Rc3 Bf6 24. Rcd3 O-O 25. g5 Be7 26. Rc3 Rd8 27. Rd4 Bxc5 28. Rxc5 Qd6 29. Rxc7 Qxc7 30. Qc3 Qh2 31. Qg3 Qh1+ 32. Kb2 Qe1 33. Qc7 Rf8 34. f4 b5 35. Nc5 Qe3 36. Rd3 Qf2 37. Nd7 Re8 38. Ne5 Qxf4 39. Rf3 Qd4+ 40. Ka3 Rf8 41. Rxf7 Rxf7 42. Qxf7+ Kh8 43. Nxg6+ Kh7 44. Nf8+ Kh8 45. Nxe6 Qc3 46. Qe8+ Kh7 47. g6+ Kh6 48. Qh8+ Kxg6 49. Nf4+ Kf7 50. Qh5+ Kg8 51. Qxd5+ Kh7 52. Qd3+ Qxd3 { Black resigns. } 1-0
*/

use std::collections::HashMap;

use crate::{
    board::Board,
    converters::{fen_converter::fen_to_board, pgn_converter::pgn_tags::PGNTag},
    gamestate::GameState,
};

mod pgn_moves;
mod pgn_tags;

pub const MAX_TAGS: usize = 11;

pub struct PGNString {
    pub tags: HashMap<PGNTag, String>,
    pub moves: Vec<u16>, // moves are represente d in u16 format
    pub white_player: String,
    pub black_player: String,
    pub white_elo: u16,
    pub black_elo: u16,
    pub start_pos: String, // fen string
}

impl PGNString {
    pub fn create_empty() -> Self {
        return Self {
            tags: HashMap::new(),
            moves: Vec::new(),
            white_player: "?".to_string(),
            black_player: "?".to_string(),
            white_elo: 0,
            black_elo: 0,
            start_pos: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(), // default startpos
        };
    }
}

pub fn pgn_to_game(pgn: &str) -> (Board, GameState) {
    let mut pgn_string: PGNString = PGNString::create_empty();
    pgn_string.decode_tags(pgn);
    pgn_string.apply_tags();

    let (board, state): (Board, GameState) = fen_to_board(pgn_string.start_pos.as_str());
    unimplemented!();
}
