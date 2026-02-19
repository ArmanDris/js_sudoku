import { Board, launch_algorithm_x } from "./lib/rs_lib.js";

/**
 * WASM representation of a board. When initilized the board's cells are all 0
 *
 * @static from_board static method that initializes a new board object from an
 *                    existing board object
 * @method set Sets the value at x,y. The top left corner is 0,0
 * @method get Returns the value at x,y. The top left corner is 0,0
 * @method free frees the board from memory
 */
export { Board };

/**
 * Uses algorithm_x to find the specificed number of solutions in the given
 * board.
 *
 * Internally this will map to an absolute cover problem, it will then find
 * the given number of subsets of those rows that satisfy the absolute cover
 * problem. Finally it will convert each of those subsets back into Sudoku
 * boards and return them
 *
 * @param starting_board Optional board to start searcing from, if not passed
 *                       will default to empty board
 * @param decision_strategy Optioanl strategy to use when faces with multiple
 *                          choices of cell values. Can be either first or
 *                          random. If not passed will be random
 * @param desired_solutions Optioanl number of solutions to search for. By
 *                          default 1
 */
export { launch_algorithm_x };
