import { assertEquals } from "@std/assert/equals";
import { Board, launch_algorithm_x } from "./mod.ts";

Deno.test("Can manipulate a WASM board object", () => {
  const board = new Board();
  board.set(5, 8, 9);
  assertEquals(board.get(5, 8), 9);
});

Deno.test("Can use algorithm_x to solve a board", () => {
  const board = new Board();
  board.set(0, 0, 9);
  board.set(8, 8, 9);
  const solution = launch_algorithm_x(board, null, 1)[0];

  let num_zeros = 0;
  for (let col_idx = 0; col_idx < 9; col_idx++) {
    for (let row_idx = 0; row_idx < 9; row_idx++) {
      if (solution.get(col_idx, row_idx) == 0) {
        num_zeros += 1;
      }
    }
  }
  assertEquals(num_zeros, 0);
  assertEquals(solution.get(0, 0), 9);
  assertEquals(solution.get(8, 8), 9);
});
