# js_sudoku

This is a jsr package to create and solve sudoku boards.

### Deno implementation:

`dfs_prune.ts`'s `dfs_prune` function will generate `stop_number` board(s).
Generating a board takes about 1.10ms.

### Rust implementation:

`rs_lib/src/algorithm_x.rs`'s `launch_algorithm_x` function generates a Sudoku
board using Donald Knuth's Algorithm X. It does this by converting the board
into an absolute cover problem, solving the abolute cover problem, and then 
converting the solution back into a Sudoku board. Generating a board with this
method takes about 112.547ms.

### Quickstart

Running the tests will give a quick overview of the important components for
generating a Sudoku board.

#### Deno tests
```js
deno test
```

#### Rust tests
```rs
cd rs_lib
cargo test
````

