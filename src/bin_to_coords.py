from string import ascii_lowercase

idx_to_coords: dict[int, tuple] = dict(
    enumerate(
        (
<<<<<<< HEAD
            (col, str(row))
=======
            "".join((col, str(row)))
>>>>>>> bitboard_edition
            for row in range(8, 0, -1)
            for col in ascii_lowercase[:8][::-1]
        )
    )
)


def bin_to_chess_coords(num: str) -> list[str]:
<<<<<<< HEAD
    return [
        "".join(idx_to_coords[i])
        for i, bit in enumerate(num[2:])
        if bit == "1"
    ]
=======
    return [idx_to_coords[i] for i, bit in enumerate(num) if bit == "1"]
>>>>>>> bitboard_edition


def reverse_bin(num: str) -> str:
    return "0b" + num[::-1]


<<<<<<< HEAD
print(
    reverse_bin(
        "0000000000000000000000000000000010100000000100100000010000010000"
    )
)
print(
    reverse_bin(
        "0001000000000100000100101011010000100010000000000010001000010100"
    )
)
=======
def assemble_bin_from_coords(squares: list[str]) -> str:
    """accepts a list of chess coordinates as an input
    and outputs a binary representation of those coordinates"""
    return "".join(
        ("1" if item[1] in squares else "0" for item in idx_to_coords.items())
    )
>>>>>>> bitboard_edition
