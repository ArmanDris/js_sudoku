# js_sudoku

This is a jsr package to efficently create and solve sudoku boards.

### Quickstart

The `launch_algorithm_x` function and the `Board` class are the only two 
exports of this package.

`launch_algorithm_x`: finds the specified number of solutions for the passed 
Sudoku board.
 - **starting_board**: the board to begin searching from (empty board by default).
 - **decision_strategy**: whether to solve sequentially (eg. will attempt 1, then 2
   then 3, etc...) or to solve randomly (pick random numbers to fill the cells)
   (will pick randomly by default.)
 - **desired_solutions**: the number of solution algorithm_x should find before 
   returning


### Deno implementation:

`dfs_prune.ts`'s `dfs_prune` function will generate `stop_number` board(s).
Generating a board takes about 1.10ms.

### Rust implementation:

`rs_lib/src/algorithm_x.rs`'s `launch_algorithm_x` function generates a Sudoku
board using Donald Knuth's Algorithm X. It does this by converting the board
into an absolute cover problem, solving the abolute cover problem, and then 
converting the solution back into a Sudoku board. Generating a board with this
method takes about 112.547ms.

#### Deno tests
```js
deno test
```

#### Rust tests
```rs
cd rs_lib
cargo test
````

