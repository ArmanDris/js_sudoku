import { check_constraints, new_board, print_board, set_box } from "./board.ts";
import type { Board, Box } from "./board.ts";

let solutions: Array<Board> = [];
let failures: number = 0;

// Retuns the index of the first zero index
// -1 if all elements are non-zero
function next_zero_index(b: Board): number {
	for (let i = 0; i < b.length; i++) {
		if (b[i] === 0) return i;
	}
	return -1;
}

// There are 6.7x10^(21) solutions so stop_number
// tells dfs when to stop generating solutions
// Returns the array of solutions
function dfs_prune(
	b: Board = new_board(),
	stop_number: number = 1000,
): Array<Board> {
	if (solutions.length >= stop_number) return solutions;

	const domain: Array<Box> = [1, 2, 3, 4, 5, 6, 7, 8, 9];

	const next_index = next_zero_index(b);

	if (next_index === -1) {
		solutions.push([...b]);
		return;
	}

	for (const e of domain) {
		b[next_index] = e;
		if (check_constraints(b)) {
			dfs_prune(b);
		} else {
			failures += 1;
		}
		b[next_index] = 0;
	}
	return solutions;
}

export { dfs_prune };
