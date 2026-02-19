import { add, Board, launch_algorithm_x } from "./lib/rs_lib.js";

export { Board, launch_algorithm_x };

// adds
console.log(add(1, 1));

// greets
const greeter = new Board();
console.log(greeter.print_board());
