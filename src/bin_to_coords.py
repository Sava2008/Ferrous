from string import ascii_lowercase

idx_to_coords: dict[int, tuple] = dict(
    enumerate(
        (
            "".join((col, str(row)))
            for row in range(8, 0, -1)
            for col in ascii_lowercase[:8][::-1]
        )
    )
)


def bin_to_chess_coords(num: str) -> list[str]:
    length: int = len(num)
    if length > 64:
        raise ValueError("a chess board cannot have more than 64 squares")

    num: str = (
        "".join(["0" for _ in range(64 - length)]) + num
        if length < 64
        else num
    )

    return [idx_to_coords[i] for i, bit in enumerate(num) if bit == "1"]


def reverse_bin(num: str) -> str:
    return num[::-1]


def assemble_bin_from_coords(squares: list[str]) -> str:
    return "".join(
        ("1" if item[1] in squares else "0" for item in idx_to_coords.items())
    )


def bin_to_indices(num: str) -> list[int]:
    return [63 - i for i, digit in enumerate(num) if digit == "1"][::-1]


def indices_to_bin(indices: list[int]) -> str:
    return assemble_bin_from_coords([idx_to_coords[i] for i in indices])


print(f"white bishops: {bin_to_chess_coords(bin(8589934628)[2:])}")
print(f"black pawns: {bin_to_chess_coords(bin(69524319247532032)[2:])}")
print(f"black king: {bin_to_chess_coords(bin(1152921504606846976)[2:])}")
