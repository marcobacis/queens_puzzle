# N Queens Puzzle

Solution to the N queens puzzle in rust.

For a given size of the board (by default is 8), the program will count the number of possible solutions to the puzzle.

## Usage

To build, run `cargo build --release`.

After that, to run it, run `target/release/queens` to solve the puzzle with 8 queens, or pass a number to indicate the number of queens and board size for which to solve.

Example: 
```
$ target/release/queens 9
Found 352 solutions
```


