# Ferrous
Ferrous is a brute-force calculating chess engine that plays in a local environment (already partially UCI-compliant). The way it works is it looks through positions by generating best moves for both sides in its "mind" and determines which one of the root moves is the best for it. Ferrous works on bitboards, which means it has room for massive optimizations and potential to become better than all human chess players on Earth.
supported UCI commands:
1. `uci`
2. `ucinewgame`
3. `isready`
4. `position fen [position]`
5. `go`
6. `go depth [depth]`
7. `go movetime [time_in_ms]`
8. `quit`
9. `go perft [depth]` (new to Ferrous v0.4.1)

### What has been actualized
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

### Planned on being carried out
- pgn converter
- opening book
- null move pruning
- futility search
- razor pruning
- late move reduction
- aggressive move ordering for better pruning
- syzygy database

### History
I ([Sava2008](https://github.com/Sava2008)) am an advanced chess player, and I have always admired how a machine can play better than any human being. I'd been considering the idea of making my own engine for a few months then, and had decided that I had had enough competence to fullfil the dream, so, I had embarked on building Ferrous, a functional chess engine that came up with a move in any position by simply evaluating the material, and piece positioning. This is the second version, which unlike the array-based first version, uses bitboards to look through thousands of positions per second and reach better depth

### Comments
currently the performance is estimated to be around 1.0-1.8 MNPS. estimated elo approx. 1700 on lichess.org and < 1500 among engines

### Instruction on usage
0. Rust 1.96.1 is required on your computer (should work with older versions, but I did not check that)
1. download the zip file with the engine
2. unpack it anywhere
3. open the terminal on your computer
4. copy the directory of the folder with Ferrous
5. run the following command in the terminal `cd path/to/Ferrous && cargo build --release` for maximum optimizations
6. locate to target folder, then release, and double click the executable file with LMB.
Currently works only for existing releases

### References
also check out PerftLab [PerftLab repo](https://github.com/Sava2008/PerftLab)

#### Credits
developer: Sava2008
