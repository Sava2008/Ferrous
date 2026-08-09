# Ferrous

Ferrous is an HCE¹ partially UCI-compliant chess engine. The way it works is it looks through positions by generating best moves for both sides in its "mind" and determines which one of the root moves is the best for it. Ferrous works on bitboards, which means it has room for massive optimizations and potential to become better than all human chess players on Earth.
supported UCI commands:

- `uci`
- `ucinewgame`
- `isready`
- `position fen [position]`
- `go`
- `go depth [depth]`
- `go movetime [time_in_ms]`
- `quit`
- `go perft [depth]`

_1 - HCE stands for hand-crafted evaluation. It's a set of strict rules hardcoded into the engine. Known to be worse than NNUE for quiet positions_

## What has been actualized

- bitboard generation
- occupancy generation
- heuristics & piece values
- fen-to-board and board-to-fen converters
- magic bitboards
- pseudo-legal move generation with later validation
- incremental evaluation
- alpha beta pruning algorithm
- basic moves tuning
- moves make-unmake system
- move encoded in u16, including from and to square, and flag
- UCI protocol
- history heuristics
- quiescence search
- transposition tables
- dymanic depth for time controls
- late move reduction
- opening book (almost)

## Planned on being carried out

- pgn converter
- null move pruning
- futility pruning
- razor pruning
- aggressive move ordering for better pruning
- syzygy datatable

### History

I ([Sava2008](https://github.com/Sava2008)) am an advanced chess player, and I have always admired how a machine can play better than any human being. I'd been considering the idea of making my own engine for a few months then, and had decided that I had had enough competence to fullfil the dream, so, I had embarked on building Ferrous, a functional chess engine that came up with a move in any position by simply evaluating the material, and piece positioning. This is the second version, which unlike the array-based first version, uses bitboards to look through thousands of positions per second and reach better depth

### Comments

Ferrous v0.5.1-dev1_tuningtweak against Ferrous v0.5.0: 104 wins, 98 losses, 38 draws

### Instruction on usage

0. Rust 1.97.1 is required on your computer (should work with older versions, but I did not check that)
1. clone the main branch repo anywhere on your computer
2. run the following command in the terminal `cd path/to/Ferrous && cargo build --release` for maximum optimizations
3. locate to target folder, then release, and double click the executable file with LMB.
Currently works only for existing releases

### References

also check out PerftLab [PerftLab repo](https://github.com/Sava2008/PerftLab)

#### Credits

developer: Sava2008
