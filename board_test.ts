import { assert, assertEquals, assertFalse } from "jsr:@std/assert";
import {
	get_box,
	is_board_solved,
	new_board,
	print_board,
	set_box,
} from "./board.ts";
import type { Board, Box } from "./board.ts";

const solved_board: Board = [
	7,
	5,
	3,
	6,
	2,
	1,
	8,
	9,
	4,
	2,
	9,
	4,
	3,
	8,
	7,
	6,
	1,
	5,
	1,
	6,
	8,
	4,
	9,
	5,
	7,
	2,
	3,
	4,
	3,
	2,
	5,
	1,
	8,
	9,
	6,
	7,
	8,
	7,
	5,
	2,
	6,
	9,
	3,
	4,
	1,
	9,
	1,
	6,
	7,
	3,
	4,
	5,
	8,
	2,
	3,
	4,
	1,
	9,
	5,
	6,
	2,
	7,
	8,
	6,
	2,
	7,
	8,
	4,
	3,
	1,
	5,
	9,
	5,
	8,
	9,
	1,
	7,
	2,
	4,
	3,
	6,
];

Deno.test("can properly get and set board", () => {
	// Setting the first 9 characters of a
	// board object should result in the expected data structure
	const row_filled = new_board();
	for (let i = 0; i < 9; i++) {
		row_filled[i] = (i + 1) as Box;
	}
	const first_row = [];
	for (let i = 0; i < 9; i++) {
		first_row.push(get_box(row_filled, i, 0));
	}
	const expected_first_row = [1, 2, 3, 4, 5, 6, 7, 8, 9];

	assertEquals(first_row, expected_first_row);

	// Now we should be able to set the first row using set_box
	// as well
	let board_two = new_board();
	let num: Box = 9;
	for (let i = 0; i < 9; i++) {
		board_two = set_box(board_two, i, 0, num as Box);
		num--;
	}
	const board_two_first_row = [];
	for (let i = 0; i < 9; i++) {
		board_two_first_row.push(get_box(board_two, i, 0));
	}
	const expected_board_two_first_row = [9, 8, 7, 6, 5, 4, 3, 2, 1];
	assertEquals(board_two_first_row, expected_board_two_first_row);

	// Now verify that manipulating past the first row works
	// I will test the third to last row. There are 9 rows
	// so that is index 6 * 9 - 1 = 53. Since we are manipulating
	// the sixth row, there are 9 indexes in a row and it is
	// zero indexed. Since we want to manipulate a whole row we
	// are manipulating index 53 to 62.
	const three_b = new_board();
	let b: Box = 1;
	for (let i = 53; i < 62; i++) {
		three_b[i] = b as Box;
		b++;
	}
	const three_values = [];
	for (let i = 53; i < 62; i++) {
		three_values.push(three_b[i]);
	}
	// console.log(three_b);
	assertEquals(three_values, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

	// Testing that column manipulation works as well. We
	// will test assigning the entire fourth column manually.
	// So we will modify 0,3 1,3 2,3 3,3 4,3 5,3 6,3 7,3 8,3.
	// These translate to indexes 3, 12, 21, 30, 39, 48, 57,
	// 66, 75
	const four_b = new_board();
	let four_num = 1;
	for (let i = 3; i < 81; i += 9) {
		four_b[i] = four_num as Box;
		four_num++;
	}
	const found_four_b = [];
	for (let i = 3; i < 81; i += 9) {
		found_four_b.push(four_b[i]);
	}
	assertEquals(found_four_b, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

	// Now check if we can do this using set_box and get_box

	let five_b = new_board();
	for (let i = 0; i < 9; i++) {
		five_b = set_box(five_b, 4, i, i + 1 as Box);
	}
	const found_five_b = [];
	for (let i = 0; i < 9; i++) {
		found_five_b.push(get_box(five_b, 4, i));
	}
	assertEquals(found_five_b, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
});

Deno.test("check board solved", () => {
	assert(is_board_solved(solved_board));
	let wrong_row = set_box(solved_board, 0, 0, 4);
	wrong_row = set_box(wrong_row, 0, 8, 7);
	assertFalse(is_board_solved(wrong_row));
});
