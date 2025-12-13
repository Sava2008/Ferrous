from string import ascii_lowercase

idx_to_coords: dict[int, tuple] = dict(
    enumerate(
        (
            (col, str(row))
            for row in range(8, 0, -1)
            for col in ascii_lowercase[:8][::-1]
        )
    )
)


def bin_to_chess_coords(num: str) -> list[str]:
    return [
        "".join(idx_to_coords[i])
        for i, bit in enumerate(num[2:])
        if bit == "1"
    ]


def reverse_bin(num: str) -> str:
    return "0b" + num[::-1]


print(
    bin_to_chess_coords(
        "1110000010111111111000000101000001001000010001000100001001000001"
    )
)
