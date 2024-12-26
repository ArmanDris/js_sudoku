import { get_box, is_board_solved, new_board, set_box } from "./board.ts";

// Make random changes to the board until we have
// exaustively search all posibilities for solutions
function brute_force_solve() {
	let board = new_board();

	for (let x = 0; x < 9; x++) {
		for (let y = 0; y < 9; y++) {
			// Try all combinations of 1-9 for this cell
			for (let i = 1; i <= 9; i++) {
				set_box(x, y, i);
			}
		}
	}
}
