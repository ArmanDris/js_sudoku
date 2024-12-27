import {
	check_constraints,
	is_board_solved,
	new_board,
	print_board,
	set_box,
} from "./board.ts";
import { dfs_prune } from "./dfs_prune.ts";

const solutions = dfs_prune();

print_board(solutions[0]);
