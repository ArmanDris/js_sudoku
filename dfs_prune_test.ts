import { assertEquals } from "@std/assert/equals";
import {
	check_constraints,
	is_board_solved,
	new_board,
} from "./board.ts";
import type { Board } from "./board.ts";
import { dfs_prune } from "./dfs_prune.ts";
import { assert } from "@std/assert/assert";
import { nyt_hardest_sudoku, first_soln } from "./example_data.ts";

Deno.test("Returns expected solution", () => {
	const solns = dfs_prune(new_board(), 1, false);
	assertEquals(solns.length, 1, "Should only return one solution");
	assertEquals(solns[0], first_soln);
});

Deno.test("Is able to respond with random boards", () => {
	const solns = dfs_prune(new_board(), 1, true);
	assertEquals(solns.length, 1);
	assert(check_constraints(solns[0]));
	assert(is_board_solved(solns[0]));
});

Deno.test("Is able to create many solutions", () => {
	const solns = dfs_prune(new_board(), 1, true);
	for (const s of solns) {
		assert(is_board_solved(s));
	}
});

// Deno.test("Average time to generate x boards across x trials", () => {
// 	const TRIALS = 10;
// 	const BOARDS_PER_TRIAL = 100;

// 	const trialTimes: number[] = [];

// 	for (let trial = 0; trial < TRIALS; trial++) {
// 		const start = performance.now();

// 		const solns = dfs_prune(new_board(), BOARDS_PER_TRIAL, true);
		
// 		const elapsed = performance.now() - start;
// 		trialTimes.push(elapsed);

// 		assertEquals(solns.length, BOARDS_PER_TRIAL);
// 		assert(check_constraints(solns[0]));
// 		assert(is_board_solved(solns[0]));

// 		console.log(
// 			`Trial ${trial + 1}: ${elapsed.toFixed(2)} ms ` +
// 			`(${(elapsed / BOARDS_PER_TRIAL).toFixed(2)} ms/board)`
// 		);
// 	}

// 	const average =
// 		trialTimes.reduce((a, b) => a + b, 0) / trialTimes.length;

// 	console.log(
// 		`\nAverage over ${TRIALS} trials: ${average.toFixed(2)} ms ` +
// 		`(${(average / BOARDS_PER_TRIAL).toFixed(2)} ms/board)`
// 	);
// });

