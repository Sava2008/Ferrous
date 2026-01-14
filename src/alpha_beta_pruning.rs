use crate::enums::PieceColor;
pub struct Engine {
    pub side: PieceColor, // which color Ferrous plays
    pub best_possible_score: i32,
    pub worst_possible_score: i32,
    pub depth: u8,
    pub evaluation: i32,
}

impl Engine {
    pub fn alpha_beta_pruning() -> () {
        todo!();
    }
}
