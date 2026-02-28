### Currently actualized
- bitboard generation
- occupancy generation
- heuristics & piece values
- attack, lines + diagonals and ray tables
- fen-to-board and board-to-fen converters
- magic bitboards
- actual move generation (with checks and pin detection)
- evaluation function
- alpha beta pruning algorithm
- basic moves tuning
- moves make-unmake system
- move encoded in u16 now instead of being stored as a struct

### Planned on being carried out
- pgn converter
- opening book
- terminal-based UI
- UCI protocol
- syzygy database
- zobrist hashing
- transposition tables
- aggressive move ordering for better pruning
- micro optimizations

### History
I ([Sava2008](https://github.com/Sava2008)) am an advanced chess player, and I have always admired how a machine can play better than any human being. I'd been considering the idea of making my own engine for a few months then, and had decided that I had had enough competence to fullfil the dream, so, I had embarked on building Ferrous, a functional chess engine that came up with a move in any position by simply evaluating the material, and piece positioning. This is the second version, which unlike the array-based first version, uses bitboards to look through thousands of positions per second and reach better depth

### Comments
currently the performance is estimated to be around 900k-1m nodes/sec, but Ferrous searches widely, not deeply due to bad move tuning. estimated elo approx. 1600-1650 on lichess.org

### Instruction on usage
0. Rust 1.93.0 is required on your computer (should work with older versions, but I did not check that)
1. download the zip file with the engine
2. unpack it anywhere
3. open the terminal on your computer
4. copy the directory of the folder with Ferrous
5. run the following command in the terminal `cd path/to/Ferrous && cargo build --release` for maximum optimizations
6. locate to target folder, then release, and cut out the executable file from there
7. paste the executable in the main directory of Ferrous and double click it with LMB
Currently works only for existing releases

### References
look up Ferrous's games on ([lichess study I created](https://lichess.org/study/m5IPaoy8))

#### Credits
developer: Sava2008
# Ferrous
Chess engine which plays in a local environment
