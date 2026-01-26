#[allow(unused_imports)]
use crate::board::Board;

#[test]
fn bitboard_to_indices_test() -> () {
    assert_eq!(
        Board::bitboard_to_indices(
            0b0000000000000000000000010000000000001100000000000000100000001000
        ),
        vec![3, 11, 26, 27, 40]
    );
    assert_eq!(
        Board::bitboard_to_indices(
            0b0000001010100001000000000000001000000000001000000000000000000000
        ),
        vec![21, 33, 48, 53, 55, 57]
    );
    assert_eq!(
        Board::bitboard_to_indices(
            0b1000000000000000000000000011000000000000000000000000010000001110
        ),
        vec![1, 2, 3, 10, 36, 37, 63]
    );
}

#[test]
fn indices_to_bitboard_test() -> () {
    assert_eq!(
        Board::indices_to_bitboard(&vec![3, 11, 26, 27, 40]),
        0b0000000000000000000000010000000000001100000000000000100000001000
    );
    assert_eq!(
        Board::indices_to_bitboard(&vec![21, 33, 48, 53, 55, 57]),
        0b0000001010100001000000000000001000000000001000000000000000000000
    );
    assert_eq!(
        Board::indices_to_bitboard(&vec![1, 2, 3, 10, 36, 37, 63]),
        0b1000000000000000000000000011000000000000000000000000010000001110
    );
}
