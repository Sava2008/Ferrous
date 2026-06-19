pub const BIT_MASKS: [u64; 64] = compute_bit_masks();

const fn compute_bit_masks() -> [u64; 64] {
    let mut bit_masks: [u64; 64] = [0; 64];
    let mut i: usize = 0;
    while i < 64 {
        bit_masks[i] = 1 << i;
        i += 1;
    }
    return bit_masks;
}

pub const ISOLATED_PAWNS: [i32; 256] = initialize_isolated_lookup();

const fn initialize_isolated_lookup() -> [i32; 256] {
    let mut table: [i32; 256] = [0; 256];
    let mut pattern: u8 = 0;
    loop {
        let mut isolated: i32 = 0;
        let mut file: i32 = 0;
        while file < 8 {
            let has_pawn: u8 = (pattern >> file) & 1;
            let left_has_pawn: u8 = if file > 0 {
                (pattern >> (file - 1)) & 1
            } else {
                0
            };
            let right_has_pawn: u8 = if file < 7 {
                (pattern >> (file + 1)) & 1
            } else {
                0
            };
            if has_pawn == 1 && left_has_pawn == 0 && right_has_pawn == 0 {
                isolated += 1;
            }
            file += 1;
        }
        table[pattern as usize] = isolated;
        pattern = pattern.wrapping_add(1);
        if pattern == 0 {
            break;
        }
    }
    return table;
}
