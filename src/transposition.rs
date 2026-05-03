#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: u8, // 0 = exact score; 1 = good move; 2 = bad move
    pub best_move: u32,
}

const TT_LEN: usize = 16_777_216;

pub struct TranspositionTable {
    pub entries: Box<[TTEntry]>,
    pub collisions: u64,
    pub replacements: u64,
    pub occupied: u64,
    pub hits: u64,
}

impl TranspositionTable {
    pub fn new() -> Self {
        return Self {
            entries: vec![
                TTEntry {
                    hash: 0,
                    score: 0,
                    depth: 0,
                    flag: 3, // no flag
                    best_move: 0,
                };
                TT_LEN
            ]
            .into_boxed_slice(),
            collisions: 0,
            replacements: 0,
            occupied: 0,
            hits: 0,
        };
    }

    pub fn get_entry(&mut self, hash_num: &u64, depth: u8) -> Option<TTEntry> {
        let entry: TTEntry = self.entries[(*hash_num as usize) & (TT_LEN - 1)];

        if entry.hash == *hash_num {
            if entry.depth >= depth {
                self.hits += 1;
            }
            return Some(entry);
        } else {
            if entry.hash != 0 {
                self.collisions += 1;
            }
            return None;
        }
    }

    pub fn record_entry(&mut self, hash_num: &u64, entry: TTEntry) -> () {
        let entry_index: usize = (*hash_num as usize) & (TT_LEN - 1);
        let old_entry: &mut TTEntry = &mut self.entries[entry_index];
        if entry_index < TT_LEN {
            if old_entry.hash == 0 {
                self.occupied += 1
            } else {
                self.replacements += 1;
                if entry.depth < old_entry.depth {
                    return ();
                }
            }
            *old_entry = entry;
        }
    }
}
