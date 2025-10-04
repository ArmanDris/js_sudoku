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

Deno.test("Gives unique solution for board with unique solution.", () => {
	// This will search the entire solution space since we ask for
	// 1000 solutions even though there is just one.
	const solns = dfs_prune(nyt_hardest_sudoku as Board, 1000, true);
	assertEquals(
		solns.length,
		1,
		"Ensure it only returns the unique solution and nothing else",
	);
});
