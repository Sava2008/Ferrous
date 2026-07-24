#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub score: i32,
    pub depth: usize,
    pub flag: u8,
    pub best_move: u16,
}

const TT_LEN: usize = 8 * 1024 * 1024; // 32 MB
const BUCKET_LEN: usize = 2;

pub struct TranspositionTable {
    pub entries: Box<[[TTEntry; BUCKET_LEN]]>,
    pub collisions: u64,
    pub replacements: u64,
    pub occupied: u64,
    pub hits: u64,
}

impl TranspositionTable {
    pub fn new() -> Self {
        return Self {
            entries: vec![
                [TTEntry {
                    hash: 0,
                    score: 0,
                    depth: 0,
                    flag: 3, // no flag
                    best_move: 0,
                }; BUCKET_LEN];
                TT_LEN
            ]
            .into_boxed_slice(),
            collisions: 0,
            replacements: 0,
            occupied: 0,
            hits: 0,
        };
    }

    pub fn get_entry(&mut self, hash_num: &u64) -> Option<TTEntry> {
        let bucket: &[TTEntry; BUCKET_LEN] = &self.entries[(*hash_num as usize) & (TT_LEN - 1)];

        for entry in bucket {
            if entry.hash == *hash_num {
                return Some(*entry);
            }
        }
        return None;
    }

    pub fn record_entry(&mut self, hash_num: &u64, entry: TTEntry) -> () {
        let entry_index: usize = (*hash_num as usize) & (TT_LEN - 1);
        let old_entries_bucket: &mut [TTEntry; BUCKET_LEN] = &mut self.entries[entry_index];
        let mut worst_entry_idx: usize = 0;

        for idx in 0..BUCKET_LEN {
            {
                let old_entry: &mut TTEntry = &mut old_entries_bucket[idx];
                if *hash_num == old_entry.hash {
                    old_entries_bucket[idx] = entry;
                    return ();
                }
            }
            let old_entry: &TTEntry = &old_entries_bucket[idx];
            let old_worst_entry: &TTEntry = &old_entries_bucket[worst_entry_idx];
            if old_entry.depth < old_worst_entry.depth {
                worst_entry_idx = idx;
            }
        }
        let old_entry: &mut TTEntry = &mut old_entries_bucket[worst_entry_idx];
        if old_entry.hash == 0 {
            self.occupied += 1
        } else {
            if entry.depth < old_entry.depth {
                return ();
            }
            self.replacements += 1;
        }
        *old_entry = entry;
    }
}
