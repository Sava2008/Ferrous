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
    reverse_bin(
        "0000000000000000000000000000000010100000000100100000010000010000"
    )
)
print(
    reverse_bin(
        "0001000000000100000100101011010000100010000000000010001000010100"
    )
)
