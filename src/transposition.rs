#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: u8,
    pub best_move: u32,
}

const TT_LEN: usize = 16_777_216;

pub struct TranspositionTable {
    pub entries: [TTEntry; TT_LEN],
}

impl TranspositionTable {
    pub fn new() -> Self {
        return Self {
            entries: [TTEntry {
                hash: 0,
                score: 0,
                depth: 0,
                flag: 0,
                best_move: 0,
            }; TT_LEN],
        };
    }

    pub fn get_entry(&self, hash_num: &u64) -> TTEntry {
        return self.entries[(*hash_num as usize) & (TT_LEN - 1)];
    }
}
