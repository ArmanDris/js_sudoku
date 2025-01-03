type Box = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

type Board = [
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
	Box,
];

function new_board(): Board {
	return new Array(81).fill(0) as Board;
}

function set_box(board: Board, x: number, y: number, value: Box): Board {
	const new_board = [...board] as Board;
	new_board[y * 9 + x] = value;
	return new_board;
}

function get_box(board: Board, x: number, y: number): Box {
	return board[y * 9 + x];
}

// True if board is solved, false otherwise
function is_board_solved(board: Board): boolean {
	// Returns true if arr is an array with
	// the numbers 1-9. Very useful helper
	function contains_all_box_elements(arr: Array<Box>): boolean {
		const required_elements = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9]);
		for (const num of arr) {
			required_elements.delete(num);
		}
		return required_elements.size === 0;
	}

	// Check every row
	for (let y = 0; y < 9; y++) {
		const row: Array<Box> = [];
		for (let x = 0; x < 9; x++) {
			row.push(get_box(board, x, y));
		}
		if (!contains_all_box_elements(row)) {
			return false;
		}
	}

	// Check every column
	for (let x = 0; x < 9; x++) {
		const column: Array<Box> = [];
		for (let y = 0; y < 9; y++) {
			column.push(get_box(board, x, y));
		}
		if (!contains_all_box_elements(column)) {
			return false;
		}
	}

	// Check if a 3x3 grid whose top left corner
	// sits at offset_x, offset_y has the numbers
	// 1-9. For safety only 0, 3, and 6 are allowed
	// for offset_x and offset_y.
	function check_sub_grid(offset_x: Box, offset_y: Box): boolean {
		if (![0, 3, 6].includes(offset_x) || ![0, 3, 6].includes(offset_y)) {
			throw Error("Invalid offset_x/offset_y for check_sub_grid");
		}

		const sub_grid: Array<Box> = [];
		for (let y = offset_y; y < offset_y + 3; y++) {
			for (let x = offset_x; x < offset_x + 3; x++) {
				sub_grid.push(get_box(board, x, y));
			}
		}
		if (!contains_all_box_elements(sub_grid)) {
			return false;
		}

		return true;
	}

	// Check all 9 sub-grids
	if (!check_sub_grid(0, 0)) return false;
	if (!check_sub_grid(0, 3)) return false;
	if (!check_sub_grid(0, 6)) return false;
	if (!check_sub_grid(3, 0)) return false;
	if (!check_sub_grid(3, 3)) return false;
	if (!check_sub_grid(3, 6)) return false;
	if (!check_sub_grid(6, 0)) return false;
	if (!check_sub_grid(6, 3)) return false;
	if (!check_sub_grid(6, 6)) return false;

	return true;
}

// True if the board does not violate any
// constraint. DOES NOT MEAN BOARD IS
// SOLVED. Some Box's may be empty (0)
function check_constraints(b: Board): boolean {
	// True if the given array already contains value
	function does_arr_already_contain(arr: Array<Box>, value: Box): boolean {
		for (const e of arr) {
			if (e === value) {
				return true;
			}
		}
		return false;
	}
	// Check that in each row there is
	// no duplicate, non 0 values
	for (let y = 0; y < 9; y++) {
		const row: Array<Box> = [];
		for (let x = 0; x < 9; x++) {
			const val = get_box(b, x, y);
			if (val === 0) continue;

			// If val is already in this row return false
			if (does_arr_already_contain(row, val)) return false;

			row.push(val);
		}
	}
	// Check that in each column there is no duplicate non 0 values
	for (let x = 0; x < 9; x++) {
		const column: Array<Box> = [];
		for (let y = 0; y < 9; y++) {
			const val = get_box(b, x, y);
			if (val === 0) continue;

			if (does_arr_already_contain(column, val)) return false;

			column.push(val);
		}
	}

	// Check if a 3x3 grid whose top left corner
	// sits at offset_x, offset_y has any duplicates
	// of the letters 1-9. For safety only 0, 3, and
	// 6 are allowed for offset_x and offset_y.
	function check_sub_grid(offset_x: Box, offset_y: Box): boolean {
		if (![0, 3, 6].includes(offset_x) || ![0, 3, 6].includes(offset_y)) {
			throw Error("Invalid offset_x/offset_y for check_sub_grid");
		}

		const sub_grid: Array<Box> = [];
		for (let y = offset_y; y < offset_y + 3; y++) {
			for (let x = offset_x; x < offset_x + 3; x++) {
				const val = get_box(b, x, y);
				if (val === 0) continue;
				if (does_arr_already_contain(sub_grid, val)) return false;
				sub_grid.push(val);
			}
		}
		return true;
	}

	if (!check_sub_grid(0, 0)) return false;
	if (!check_sub_grid(0, 3)) return false;
	if (!check_sub_grid(0, 6)) return false;
	if (!check_sub_grid(3, 0)) return false;
	if (!check_sub_grid(3, 3)) return false;
	if (!check_sub_grid(3, 6)) return false;
	if (!check_sub_grid(6, 0)) return false;
	if (!check_sub_grid(6, 3)) return false;
	if (!check_sub_grid(6, 6)) return false;

	return true;
}

function print_board(board: Board) {
	for (let y = 0; y < 9; y++) {
		if (y % 3 == 0) {
			console.log("-------------------------");
		}
		let row = "";
		for (let x = 0; x < 9; x++) {
			if (x % 3 == 0) {
				row += "| ";
			}
			row += board[y * 9 + x] + " ";
		}
		console.log(row + "|");
	}
	console.log("-------------------------");
}

export {
	check_constraints,
	get_box,
	is_board_solved,
	new_board,
	print_board,
	set_box,
};
export type { Board, Box };
