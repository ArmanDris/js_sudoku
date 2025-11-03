use crate::board::Board;

struct ConstraintTable {
  table: [[bool; 242]; 729],
}

impl Default for ConstraintTable {
  fn default() -> Self {
    Self {
      table: [[false; 242]; 729],
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

/// This function is responsible for mapping all 242 constraints
/// to a predicable index.
///
/// This writeup:
/// https://web.archive.org/web/20230426084731/https://garethrees.org/2007/06/10/zendoku-generation/#section-4
/// is the inspiration for my method.
///
/// constraint_broad_value is the zero indexed row/column/sub_grid
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
fn fill_row_constraints(board: &Board, constraint_row: &mut [bool; 242]) {
  for row_idx in 0..9 {
    let board_row = board.get_row(row_idx);
    for cell_idx in 0..9 {
      let required_number = cell_idx as i32 + 1;
      if board_row.contains(&required_number) {
        let index =
          map_constraint_to_column_idx(ConstraintType::Row, row_idx, cell_idx);
        constraint_row[index] = true;
      }
    }
  }
}

fn fill_column_constraints(board: &Board, constraint_row: &mut [bool; 242]) {
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
        constraint_row[index] = true;
      }
    }
  }
}

/// The sub grid constraints take up index 161 to 242
/// The first nine index'es represent whether the top
/// left sub grid contains each number. The next
/// nine indexes represent whether the top middle sub
/// grid contain each number. Following this pattern
/// the fourth set of nine indexes would represent whether
/// the left middle sub grid has each number.
fn fill_sub_grid_constraints(board: &Board, constraint_row: &mut [bool; 242]) {
  for sub_grid_y_start in (0..9).step_by(3) {
    for sub_grid_x_start in (0..9).step_by(3) {
      let mut sub_grid_numbers: Vec<i32> = vec![];
      // Add row 1
      sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start));
      sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start + 1));
      sub_grid_numbers.push(board.get(sub_grid_y_start, sub_grid_x_start + 2));
      // Add row 2
      sub_grid_numbers.push(board.get(sub_grid_y_start + 1, sub_grid_x_start));
      sub_grid_numbers
        .push(board.get(sub_grid_y_start + 1, sub_grid_x_start + 1));
      sub_grid_numbers
        .push(board.get(sub_grid_y_start + 1, sub_grid_x_start + 2));
      // Add row 3
      sub_grid_numbers.push(board.get(sub_grid_y_start + 2, sub_grid_x_start));
      sub_grid_numbers
        .push(board.get(sub_grid_y_start + 2, sub_grid_x_start + 1));
      sub_grid_numbers
        .push(board.get(sub_grid_y_start + 2, sub_grid_x_start + 2));

      for cell_idx in 0..9 {
        let required_number = cell_idx as i32 + 1;
        if sub_grid_numbers.contains(&required_number) {
          let index = map_constraint_to_column_idx(
            ConstraintType::SubGrid,
            sub_grid_x_start + (sub_grid_y_start / 3),
            cell_idx,
          );
          constraint_row[index] = true;
        }
      }
    }
  }
}

pub fn fill_constraint_table_row(board: &Board, row: &mut [bool; 242]) {
  fill_row_constraints(board, row);
  fill_column_constraints(board, row);
  fill_sub_grid_constraints(board, row);
}

fn generate_constraint_table() -> ConstraintTable {
  let mut ct = ConstraintTable::default();

  let mut current_constraint_row = 0;

  for row_idx in 0..9 {
    for col_idx in 0..9 {
      for value in 1..10 {
        // Here we need to get the correct row from the constraint table as a slice
        // Then we need to create a board with the specified choice,
        //  - eg for the first index that would be a board with a 1 at 0, 0
        // Then we need to call `fill_constraint_table_row` with the constraint table slice and the board representing the choice
        let constraint_row = &mut ct.table[current_constraint_row];
        let mut board = Board::new();
        board.set(col_idx, row_idx, value);
        fill_constraint_table_row(&board, constraint_row);
        current_constraint_row += 1;
      }
    }
  }

  return ct;
}

pub fn launch_algorithm_x() -> Board {
  // Convert to exact cover problem

  // Constraints:
  //  - all rows must contain 1-9 (81)
  //  - all columns must contain 1-9 (81)
  //  - each subgrid must contain 1-9 (81)

  // 242 constraints

  // Choices:
  //  - each cell has a choice between 1-9

  // 728 choices

  let constraint_table = generate_constraint_table().table;

  // Manual algorithm x

  let mut solution_set: Vec<[bool; 242]> = vec![];

  // Step 1: Pick an unsatisifed constraint
  let unsatisfied_column_idx = 0;

  // Step 2: Pick a row satisfying that constraint
  let satisfying_row =
    constraint_table.iter().find(|e| e[unsatisfied_column_idx]);

  // Step 3: Remove any rows that satisfy any of the constraitns satisfied by the chosen row
  // Need to write this function...

  Board::new()
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

    let mut constraint_row = [false; 242];
    fill_row_constraints(&board, &mut constraint_row);

    let all_false = constraint_row
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

    let mut constraint_row = [false; 242];
    fill_row_constraints(&board, &mut constraint_row);

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
    let mut board = Board::new();
    board.set(0, 3, 5);
    board.set(4, 3, 6);
    board.set(7, 3, 7);
    board.set(8, 3, 8);

    board.set(0, 5, 1);
    board.set(1, 5, 4);
    board.set(2, 5, 5);
    board.set(3, 5, 8);

    let mut constraint_row = [false; 242];
    fill_row_constraints(&board, &mut constraint_row);

    // fourth row will be from index [3*9, 4*9)
    let fourth_row_section_of_constraints = &constraint_row[27..36];
    let expected_fourth_row =
      [false, false, false, false, true, true, true, true, false];
    assert_eq!(fourth_row_section_of_constraints, expected_fourth_row);

    let sixth_row_section_of_constraints = &constraint_row[45..54];
    let expected_sixth_row =
      [true, false, false, true, true, false, false, true, false];
    assert_eq!(sixth_row_section_of_constraints, expected_sixth_row);
  }

  // ==========
  // Test generate_constraint_column
  // ==========
  #[test]
  fn test_no_false_positives() {
    let board = Board::new();
    let mut constraint_column = [false; 242];
    fill_column_constraints(&board, &mut constraint_column);
    let all_false = constraint_column
      .iter()
      .all(|element| -> bool { element == &false });
    assert!(all_false);
  }

  #[test]
  fn test_detects_first_column_constraints() {
    let mut board = Board::new();
    board.set(0, 3, 9);
    board.set(0, 0, 8);
    board.set(0, 8, 1);

    let mut constraint_row = [false; 242];
    fill_column_constraints(&board, &mut constraint_row);

    let offset = ConstraintType::Column.get_offset();
    let first_col_slice = &constraint_row[offset..(offset + 9)];

    let expected_first_column_constraints =
      [true, false, false, false, false, false, false, true, true];
    assert_eq!(first_col_slice, expected_first_column_constraints);
  }

  #[test]
  fn test_detects_two_random_column_constraints() {
    let mut board = Board::new();
    board.set(2, 1, 1);
    board.set(2, 2, 2);
    board.set(2, 3, 5);

    board.set(8, 6, 4);
    board.set(8, 7, 6);
    board.set(8, 8, 7);

    let mut column_constraints = [false; 242];
    fill_column_constraints(&board, &mut column_constraints);

    let column_two_offset = ConstraintType::Column.get_offset() + (9 * 2);
    let column_two_constraints =
      &column_constraints[column_two_offset..(column_two_offset + 9)];

    let expected_column_two_constraints =
      [true, true, false, false, true, false, false, false, false];
    assert_eq!(column_two_constraints, expected_column_two_constraints);

    let column_eight_offset = ConstraintType::Column.get_offset() + (9 * 8);
    let column_eight_constraints =
      &column_constraints[column_eight_offset..(column_eight_offset + 9)];

    let expected_column_eight_constraints =
      [false, false, false, true, false, true, true, false, false];
    assert_eq!(column_eight_constraints, expected_column_eight_constraints);
  }

  #[test]
  fn test_detects_sub_grid_constraints() {
    let mut board = Board::new();
    // Set the top left sub grid
    board.set(0, 0, 1);
    board.set(1, 1, 2);
    board.set(2, 2, 5);
    board.set(1, 2, 4);
    board.set(2, 0, 9);
    board.set(1, 0, 1);
    // | 1 | 1 | 9 |
    // |   | 2 |   |
    // | 4 |   | 5 |

    // Set the middle subgrid
    board.set(3, 3, 9);
    board.set(4, 3, 5);
    board.set(5, 3, 1);
    board.set(4, 4, 1);
    board.set(5, 4, 3);
    board.set(3, 5, 4);
    // | 9 | 5 | 1 |
    // |   | 1 | 3 |
    // | 4 |   |   |

    // Set the bottom right subgrid
    board.set(6, 6, 2);
    board.set(7, 6, 5);
    board.set(8, 6, 6);
    board.set(6, 7, 8);
    board.set(7, 7, 7);
    board.set(8, 7, 4);
    board.set(6, 8, 3);
    board.set(7, 8, 9);
    board.set(8, 8, 9);
    // | 2 | 5 | 6 |
    // | 8 | 7 | 4 |
    // | 3 | 9 | 9 |

    let top_left_offset = ConstraintType::SubGrid.get_offset();
    let mut sub_grid_constraints = [false; 242];
    fill_sub_grid_constraints(&board, &mut sub_grid_constraints);

    let top_left_constraints =
      &sub_grid_constraints[top_left_offset..(top_left_offset + 9)];
    let expected_top_left_constraints =
      [true, true, false, true, true, false, false, false, true];
    assert_eq!(top_left_constraints, expected_top_left_constraints);

    let middle_sub_grid_offset = ConstraintType::SubGrid.get_offset() + (9 * 4);
    let middle_constraints = &sub_grid_constraints
      [middle_sub_grid_offset..(middle_sub_grid_offset + 9)];
    let expected_middle_constraints =
      [true, false, true, true, true, false, false, false, true];
    assert_eq!(middle_constraints, expected_middle_constraints);

    let bottom_left_offset = ConstraintType::SubGrid.get_offset() + (9 * 8);
    let bottom_left_constraints =
      &sub_grid_constraints[bottom_left_offset..(bottom_left_offset + 9)];
    let expected_bottom_left_constraints =
      [false, true, true, true, true, true, true, true, true];
    assert_eq!(bottom_left_constraints, expected_bottom_left_constraints);
  }

  #[test]
  fn test_detects_all_constraints_satisfied() {
    let mut board = Board::new();

    // Set row 1
    board.set(0, 0, 3);
    board.set(1, 0, 1);
    board.set(2, 0, 6);
    board.set(3, 0, 5);
    board.set(4, 0, 7);
    board.set(5, 0, 8);
    board.set(6, 0, 4);
    board.set(7, 0, 9);
    board.set(8, 0, 2);
    // Set row 2
    board.set(0, 1, 5);
    board.set(1, 1, 2);
    board.set(2, 1, 9);
    board.set(3, 1, 1);
    board.set(4, 1, 3);
    board.set(5, 1, 4);
    board.set(6, 1, 7);
    board.set(7, 1, 6);
    board.set(8, 1, 8);

    // Set row 3
    board.set(0, 2, 4);
    board.set(1, 2, 8);
    board.set(2, 2, 7);
    board.set(3, 2, 6);
    board.set(4, 2, 2);
    board.set(5, 2, 9);
    board.set(6, 2, 5);
    board.set(7, 2, 3);
    board.set(8, 2, 1);

    // Set row 4
    board.set(0, 3, 2);
    board.set(1, 3, 6);
    board.set(2, 3, 3);
    board.set(3, 3, 4);
    board.set(4, 3, 1);
    board.set(5, 3, 5);
    board.set(6, 3, 9);
    board.set(7, 3, 8);
    board.set(8, 3, 7);

    // Set row 5
    board.set(0, 4, 9);
    board.set(1, 4, 7);
    board.set(2, 4, 4);
    board.set(3, 4, 8);
    board.set(4, 4, 6);
    board.set(5, 4, 3);
    board.set(6, 4, 1);
    board.set(7, 4, 2);
    board.set(8, 4, 5);

    // Set row 6
    board.set(0, 5, 8);
    board.set(1, 5, 5);
    board.set(2, 5, 1);
    board.set(3, 5, 7);
    board.set(4, 5, 9);
    board.set(5, 5, 2);
    board.set(6, 5, 6);
    board.set(7, 5, 4);
    board.set(8, 5, 3);

    // Set row 7
    board.set(0, 6, 1);
    board.set(1, 6, 3);
    board.set(2, 6, 8);
    board.set(3, 6, 9);
    board.set(4, 6, 4);
    board.set(5, 6, 7);
    board.set(6, 6, 2);
    board.set(7, 6, 5);
    board.set(8, 6, 6);

    // Set row 8
    board.set(0, 7, 6);
    board.set(1, 7, 9);
    board.set(2, 7, 2);
    board.set(3, 7, 3);
    board.set(4, 7, 5);
    board.set(5, 7, 1);
    board.set(6, 7, 8);
    board.set(7, 7, 7);
    board.set(8, 7, 4);

    // Set row 9
    board.set(0, 8, 7);
    board.set(1, 8, 4);
    board.set(2, 8, 5);
    board.set(3, 8, 2);
    board.set(4, 8, 8);
    board.set(5, 8, 6);
    board.set(6, 8, 3);
    board.set(7, 8, 1);
    board.set(8, 8, 9);

    let mut constraints_row = [false; 242];
    fill_row_constraints(&board, &mut constraints_row);
    fill_column_constraints(&board, &mut constraints_row);
    fill_sub_grid_constraints(&board, &mut constraints_row);

    let all_true = constraints_row
      .iter()
      .all(|constraint| -> bool { constraint == &true });
    assert!(all_true);
  }

  #[test]
  fn test_generate_constraint_table() {
    // The constraint table's indexes represent this:
    //
    //                | Row 0 has a 1 | Row 0 has a 2 | Row 0 has a 9 | ... | Row 1 has a 1 | ... |
    //
    // Place 1 at 0,0 |     true      |    false      |     false     | ... |    false      | ... |
    // Place 2 at 0,0
    // ...
    // Place 9 at 0,0 |     false     |    false      |      true     | ... |    false      | ... |
    // Place 1 at 0,1 |     true      |    false      |      false    | ... |    false      | ... |
    // ...
    // So in general, what should the constraint table look like?
    //  - We should generate it and then check specific choices
    let ct = generate_constraint_table();

    // =====================================
    // For choice of placing 4 at 0,1
    // =====================================
    let first_choice_row = &ct.table[9 + 4 - 1];
    // Asserting that the "Row 0 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[4 - 1]);
    // Asserting that the "Column 1 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[ConstraintType::Column.get_offset() + 9 + 4 - 1]);
    // Asserting that the "Subgrid 0 has a 4" constraint is true for choice "Place 4 at 0,1"
    assert!(first_choice_row[ConstraintType::SubGrid.get_offset() + 4 - 1]);
    // Asserting that all other constraints are false
    assert!(first_choice_row.iter().filter(|var| **var).count() == 3);

    // =====================================
    // For choice of placing 7 at 8, 4
    // =====================================
    let second_choice_row = &ct.table[(9 * 9 * 8) + (9 * 4) + 7 - 1];
    // Asserting that the "Row 8 has a 7" constraint is true for choice "Place 7 at 8, 4"
    assert!(second_choice_row[(9 * 8) + 7 - 1]);
    // Asserting that the "Column 4 has a 7" constraint is true for choice "Place 7 at 8,4"
    assert!(
      second_choice_row[ConstraintType::Column.get_offset() + (9 * 4) + 7 - 1]
    );
    // Asserting that the "Subgrid 7 has a 7" constraint is true for choice "Place 7 at 8,4"
    assert!(
      second_choice_row[ConstraintType::SubGrid.get_offset() + (9 * 7) + 7 - 1]
    );
    // Asserting that all other constraints are false
    assert!(second_choice_row.iter().filter(|var| **var).count() == 3);

    // =====================================
    // For choice of placting 1 at 3,0
    // =====================================
    let third_choice_row = &ct.table[9 * 3 + 1 - 1];
    // Assert that the "Subgrid 1 has a 1" constraint is true for choice "Place 1 at 0,3"
    assert!(third_choice_row[ConstraintType::SubGrid.get_offset() + 9 + 1 - 1]);
    // Placing a 1 at 0,3 should only satisfy three constraints
    assert!(third_choice_row.iter().filter(|var| **var).count() == 3);
  }
}
