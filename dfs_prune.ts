import { check_constraints, new_board } from "./board.ts";
import type { Board, Box } from "./board.ts";

function shuffle_array(arr: Array<Box>) {
	arr.sort(() => Math.random() - 0.5);
}

// Retuns the index of the first zero index
// -1 if all elements are non-zero
function next_zero_index(b: Board): number {
	for (let i = 0; i < b.length; i++) {
		if (b[i] === 0) return i;
	}
	return -1;
}

// b: The board to generate solutions for
// stop_number: When to stop generating solutions and return
//              there are 6.7x10^(21) solutions total.
// randomize_solutions: whether to randomize the numbers we
//                      attempt. When this is false the
//                      first half of most boards will be
//                      identical.
function dfs_prune(
	b: Board = new_board(),
	stop_number: number = 1,
	randomize_solutions: boolean = true,
): Array<Board> {
	return _dfs_prune_internal(b, stop_number, randomize_solutions, []);
}

function _dfs_prune_internal(
	b: Board,
	stop_number: number,
	randomize_solutions: boolean,
	solutions: Array<Board>,
) {
	if (solutions.length >= stop_number) return solutions;

	const domain: Array<Box> = [1, 2, 3, 4, 5, 6, 7, 8, 9];
	if (randomize_solutions) {
		shuffle_array(domain);
	}

	const next_index = next_zero_index(b);

	if (next_index === -1) {
		solutions.push([...b]);
		return solutions;
	}

	for (const e of domain) {
		b[next_index] = e;
		if (check_constraints(b)) {
			_dfs_prune_internal(b, stop_number, randomize_solutions, solutions);
		}
		// Invalid assignment, undo and return
		b[next_index] = 0;
	}
	return solutions;
}

export { dfs_prune };
