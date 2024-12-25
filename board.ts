type Box = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

function new_board(): Array<Box> {
	return new Array(81).fill(0);
}

function set_box(board: Array<Box>, x: number, y: number, value: Box) {
	const new_board = [...board];
	new_board[y * 9 + x] = value;
	return new_board;
}

function get_box(board: Array<Box>, x: number, y: number): Box {
	return board[y * 9 + x];
}

// True if board is solved, false otherwise
function is_board_solved(board: Array<Box>): boolean {
	// Returns true if arr is an array with
	// the numbers 1-9. Very useful helper
	function contains_all_box_elements(arr: Array<Box>): boolean {
		const required_elements = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9]);
		for (const num of arr) {
			required_elements.delete(num);
		}
		return required_elements.size === 0;
	}
	// Check if each row is solved
	for (let y = 0; y < 9; y++) {
		const row: Array<Box> = [];
		for (let x = 0; x < 9; x++) {
			row.push(get_box(board, x, y));
		}
		console.log("checking row: " + row);
		if (!contains_all_box_elements(row)) {
			return false;
		}
	}
	return true;
}

function print_board(board: Array<Box>) {
	for (let x = 0; x < 9; x++) {
		if (x % 3 == 0) {
			console.log("-------------------------");
		}
		let row = "";
		for (let y = 0; y < 9; y++) {
			if (y % 3 == 0) {
				row += "| ";
			}
			row += board[y * 9 + x] + " ";
		}
		console.log(row + "|");
	}
	console.log("-------------------------");
}

if (import.meta.main) {
	print_board(new_board());
}

export { get_box, is_board_solved, new_board, print_board, set_box };
export type { Box };
