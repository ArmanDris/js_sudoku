use crate::board::Board;

struct ConstraintTable {
  table: [[bool; 243]; 728],
}

impl Default for ConstraintTable {
  fn default() -> Self {
    Self {
      table: [[false; 243]; 728],
    }
  }
}

enum ConstraintType {
  Row,
  Column,
  SubGrid,
}

impl ConstraintType {
  fn get_offset(&self) -> usize {
    match self {
      ConstraintType::Row => 0,
      ConstraintType::Column => 80,
      ConstraintType::SubGrid => 161,
    }
  }
}

/// This function is responsible for mapping all 243 constraints
/// to a predicable index.
///
/// Describing the method with words sucks. This writeup:
/// https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/#section-4
/// is the inspiration for my method.
///
/// constraint_broad_value is the zero indexed row or column
/// eg. row 1 would be zero, column 5 would be 4.
///
/// constraint_cell_value is the zero indexed cell value
/// eg. row 1 would be zero, column 5 would be 4.
fn map_constraint_to_column_idx(
  constraint_type: ConstraintType,
  constraint_broad_value: usize,
  constraint_cell_value: usize,
) -> usize {
  constraint_type.get_offset()
    + (constraint_broad_value * 9)
    + constraint_cell_value
}

/// Generates a row in the constraint table using the choices represented by
/// the board parameter.
/// These constraints take up the first 81 indexes in the constraint table
fn generate_constraint_row(board: &Board) -> [bool; 81] {
  let mut constraint_section = [false; 81];

  for row_idx in 0..9 {
    let board_row = board.get_row(row_idx);
    for cell_idx in 0..9 {
      let required_number = cell_idx as i32 + 1;
      if board_row.contains(&required_number) {
        let index =
          map_constraint_to_column_idx(ConstraintType::Row, row_idx, cell_idx);
        constraint_section[index] = true;
      }
    }
  }

  constraint_section
}

fn generate_constraint_column(board: &Board) -> [bool; 81] {
  let mut constraint_section = [false; 81];

  for column_idx in 0..9 {
    let board_column = board.get_column(column_idx);
    for cell_idx in 0..9 {
      let required_number = cell_idx as i32 + 1;
      if board_column.contains(&required_number) {
        let index = map_constraint_to_column_idx(
          ConstraintType::Column,
          column_idx,
          cell_idx,
        );
        constraint_section[index] = true;
      }
    }
  }
  constraint_section
}

pub fn launch_algorithm_x(board: &Board) -> Board {
  // Convert to exact cover problem

  // Constraints:
  //  - all rows must contain 1-9 (81)
  //  - all columns must contain 1-9 (81)
  //  - each subgrid must contain 1-9 (81)

  // 243 constraints

  // Choices:
  //  - each cell has a choice between 1-9

  // 728 choices

  // In total the table will have 243 * 728 = 176 904 cells
  generate_constraint_row(board);
  generate_constraint_column(board);
  Board::from_board(board)
}

// ==========
// Tests
// ==========
#[cfg(test)]
mod tests {
  use super::*;

  // ==========
  // Test generate_constratint_row
  // ==========
  #[test]
  fn on_an_empty_board_it_returns_the_correct_constraints() {
    let board = Board::new();

    let constraints_row = generate_constraint_row(&board);

    let all_false = constraints_row
      .iter()
      .all(|element| -> bool { element == &false });

    assert!(all_false);
  }

  #[test]
  fn correctly_resolves_first_row() {
    let mut board = Board::new();
    board.set(0, 0, 1);
    board.set(1, 0, 2);
    board.set(2, 0, 3);
    board.set(3, 0, 4);

    let constraint_row = generate_constraint_row(&board);

    let first_four_true = &constraint_row[0..4]
      .iter()
      .all(|element| -> bool { element == &true });
    let last_five_false = &constraint_row[4..9]
      .iter()
      .all(|element| -> bool { element == &false });

    assert!(first_four_true);
    assert!(last_five_false);
  }

  #[test]
  fn correctly_resolves_two_rows() {
    // fourth row will be from index [3*9, 4*9)
    let mut board = Board::new();
    board.set(0, 3, 5);
    board.set(4, 3, 6);
    board.set(7, 3, 7);
    board.set(8, 3, 8);

    board.set(0, 5, 1);
    board.set(1, 5, 4);
    board.set(2, 5, 5);
    board.set(3, 5, 8);

    let fourth_row_section_of_constraints =
      &generate_constraint_row(&board)[27..36];
    let expected_fourth_row =
      [false, false, false, false, true, true, true, true, false];
    assert_eq!(fourth_row_section_of_constraints, expected_fourth_row);

    let sixth_row_of_section_of_constraints =
      &generate_constraint_row(&board)[45..54];
    let expected_sixth_row =
      [true, false, false, true, true, false, false, true, false];
    assert_eq!(sixth_row_of_section_of_constraints, expected_sixth_row);
  }

  // ==========
  // Test generate_constraint_column
  // ==========
  #[test]
  fn test_no_false_positives() {
    let board = Board::new();
    let constraint_section = generate_constraint_column(&board);
    let all_false = constraint_section
      .iter()
      .all(|element| -> bool { element == &false });
    assert!(all_false);
  }

  #[test]
  fn test_detects_first_column_constraints() {
    let mut board = Board::new();
    let constraint_section = generate_constraint_column(&board);
    board.set(3, 0, 9);
    board.set(3, 0, 8);
    board.set(3, 0, 1);

    let first_col_slice = &constraint_section[0..9];

    // Found a serious bug. Each generate_constraint_... function returns an array of length
    // 81. But the offset will calculate indexes way farther than that. So for this it just
    // tries to write way past where it should, and we get all false.
    //
    // To fix this I should first create the entire 243 length row as the caller and then pass it in for the function to write on.
    // This will be way more efficent to because then i just need to create/write each constraint column once.
    let expected_first_column_constraints =
      [true, false, false, false, false, false, false, true, true];
    assert_eq!(first_col_slice, expected_first_column_constraints);
  }

  #[test]
  fn test_detects_two_random_column_constraints() {}

  #[test]
  fn test_detects_all_constraints_satisfied() {}
}
