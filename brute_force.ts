// Brute force solver
// It is not feasible to brute force Sudoku
// (takes more than 1 million years...) so
// this is just to experiment with exhaustively
// searching up to an index.
// Call `brute_force_solve(index)` to iterate
// through all possible board states from index
// 0 to index.

import { new_board } from "./board.ts";
import type { Board } from "./board.ts";

// Increments the board state by one. This
// means finding the first non 9 index and
// incrementing it. Each time we increment
// and index we reset all previous indexes
// Example: [1,1,1] -> [2,1,1] (returns false)
// Example: [9,9,3] -> [1,1,4] (returns false)
// Example [9,9,9] -> [9,9,9] (returns true)
// Returns true when called with a board
// that cannot be incremented any further.
function inc_state(b: Board, max_index: number): boolean {
	if (max_index < 0 || 80 < max_index) {
		throw Error(
			"cannot increment past index 81 as its outside the board object",
		);
	}

	let i = 0;
	while (i <= max_index) {
		if (b[i] === 9) {
			i++;
			continue;
		}
		b.fill(1, 0, i);
		b[i] += 1;
		return false;
	}

	return true;
}
// Make random changes to the board until we have
// exaustively searched all posibilities for solutions
function brute_force_solve(max_index: number = 8) {
	let counter = 1;
	const board = new_board().fill(1) as Board;
	while (true) {
		const r = inc_state(board, max_index);
		if (r) {
			break;
		}
		counter++;
	}
	console.log("Counter: ", counter);
}

if (import.meta.main) {
	console.log("Brute forcing the first row of the table (indexes 0-8)");
	console.time("brute_force_search");
	brute_force_solve(8);
	console.timeEnd("brute_force_search");
}

export { brute_force_solve };
