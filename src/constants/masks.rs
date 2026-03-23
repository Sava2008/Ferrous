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
