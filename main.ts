import { Board, launch_algorithm_x } from "./mod.ts";

const solved_board = launch_algorithm_x()[0];
console.log("output of solved board:");
for (let col = 0; col < 9; col++) {
  let output = "";
  for (let row = 0; row < 9; row++) {
    output += solved_board.get(row, col) + " "
  }
  console.log(output)
}

export {
  Board,
  launch_algorithm_x,
}
