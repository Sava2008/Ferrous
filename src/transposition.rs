#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: u8,
    pub best_move: u32,
}

pub struct TranspositionTable<const LENGTH: usize> {
    pub entries: [TTEntry; LENGTH],
}

impl<const LENGTH: usize> TranspositionTable<LENGTH> {
    pub fn new() -> Self {
        return Self {
            entries: [TTEntry {
                hash: 0,
                score: 0,
                depth: 0,
                flag: 0,
                best_move: 0,
            }; LENGTH],
        };
    }
}
