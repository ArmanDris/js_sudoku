import { create_board, print_board } from "./board.ts";
import type { Board } from "./board.ts";
import { dfs_prune } from "./dfs_prune.ts";
import { stochastic_search } from "./stochastic_search.ts";

const nyt_hardest_sudoku = create_board([
	8,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	3,
	6,
	0,
	0,
	0,
	0,
	0,
	0,
	7,
	0,
	0,
	9,
	0,
	2,
	0,
	0,
	0,
	5,
	0,
	0,
	0,
	7,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	4,
	5,
	7,
	0,
	0,
	0,
	0,
	0,
	1,
	0,
	0,
	0,
	3,
	0,
	0,
	0,
	1,
	0,
	0,
	0,
	0,
	6,
	8,
	0,
	0,
	8,
	5,
	0,
	0,
	0,
	1,
	0,
	0,
	9,
	0,
	0,
	0,
	0,
	4,
	0,
	0,
]);

// // Running dfs_prune:
// console.time("dfs_prune");
// const solutions = dfs_prune(new_board(), 1, true);
// console.timeEnd("dfs_prune");
// print_board(solutions[0]);

// // Running stochastic_search
// stochastic_search();

// Running dfs_prune to solve the hardest sudoku board:
print_board(nyt_hardest_sudoku);
console.time("Solve hardest Sudoku");
const solutions = dfs_prune(nyt_hardest_sudoku, 1, true);
console.timeEnd("Solve hardest Sudoku");
print_board(solutions[0]);
