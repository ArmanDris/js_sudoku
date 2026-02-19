// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

export function launch_algorithm_x(
  starting_board?: Board | null,
  decision_strategy?: DecisionStrategy | null,
  desired_solutions?: number | null,
): Board[];
export function add(a: number, b: number): number;
export enum DecisionStrategy {
  First = 0,
  Random = 1,
}
export class Board {
  free(): void;
  constructor();
  static from_board(board: Board): Board;
  set(x: number, y: number, value: number): void;
  get(x: number, y: number): number;
}
