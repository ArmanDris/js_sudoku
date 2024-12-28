import { new_board, print_board } from "./board.ts";
import { dfs_prune } from "./dfs_prune.ts";

console.time("dfs_prune");
const solutions = dfs_prune(new_board(), 1, true);
console.timeEnd("dfs_prune");

print_board(solutions[0]);
