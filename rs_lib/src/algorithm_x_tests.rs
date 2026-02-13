use super::*;
use std::collections::HashSet;

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
  let middle_constraints =
    &sub_grid_constraints[middle_sub_grid_offset..(middle_sub_grid_offset + 9)];
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

#[test]
fn test_find_satisfying_row_scenarios() {
  // 1) No row satisfies the column → None
  let mut ct = [[false; 242]; 729];
  let col = 137;
  let hidden = HashSet::<usize>::new();
  assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![]);

  // 2) A single row satisfies → that row index
  ct[10][col] = true;
  assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![10]);

  // 3) An earlier row also satisfies, but it’s hidden → return next visible (10)
  ct[3][col] = true;
  let mut hidden = HashSet::new();
  hidden.insert(3);
  assert_eq!(find_satisfying_rows(&ct, &hidden, col), vec![10]);

  // 4) If earlier row isn’t hidden → earliest match (3) wins
  let hidden = HashSet::<usize>::new();
  assert_eq!(find_satisfying_rows(&ct, &hidden, col).first(), Some(&3));

  // 5) Hide both earlier matches; ensure it finds a later satisfying row
  let mut hidden = HashSet::new();
  hidden.insert(3);
  hidden.insert(10);
  ct[400][col] = true;
  assert_eq!(find_satisfying_rows(&ct, &hidden, col).first(), Some(&400));

  // 6) Boundary columns: 0 and 241
  let mut ct2 = [[false; 242]; 729];

  ct2[5][0] = true;
  assert_eq!(
    find_satisfying_rows(&ct2, &HashSet::new(), 0).first(),
    Some(&5)
  );

  ct2[5][0] = false;
  ct2[25][241] = true;
  assert_eq!(
    find_satisfying_rows(&ct2, &HashSet::new(), 241).first(),
    Some(&25)
  );
}

#[test]
fn test_get_conflicting_rows() {
  const N_COLS: usize = 242;
  const N_ROWS: usize = 729;

  fn row_with(cols_true: &[usize]) -> [bool; N_COLS] {
    let mut row = [false; N_COLS];
    for &c in cols_true {
      row[c] = true;
    }
    row
  }

  // Build a full constraint table with a few meaningful rows, rest all-false.
  let mut ct = [[false; N_COLS]; N_ROWS];

  // Selected row: true at columns 0, 5, 241
  ct[10] = row_with(&[0, 5, 241]);

  // Conflicting rows (share at least one true col with row 10)
  ct[3] = row_with(&[5]); // shares 5
  ct[7] = row_with(&[241]); // shares 241
  ct[15] = row_with(&[0, 100]); // shares 0 (and some other)
  ct[20] = row_with(&[0, 5, 241]); // shares many

  // Non-conflicting rows
  ct[8] = row_with(&[1, 2, 3]); // no overlap
  ct[9] = row_with(&[]); // all false
  ct[11] = row_with(&[6, 7]); // no overlap with 0,5,241

  // Hidden rows: one conflicting row is hidden, one non-conflicting row hidden too
  let mut hidden = HashSet::new();
  hidden.insert(7); // would conflict, but hidden -> should be excluded
  hidden.insert(8); // doesn't conflict anyway

  // Act
  let mut got = get_conflicting_rows(&ct, &hidden, 10);
  got.sort_unstable();

  // Expect: conflicting rows except selected (10) and hidden (7)
  let mut expected = vec![3, 15, 20];
  expected.sort_unstable();
  assert_eq!(got, expected);

  // Additional check: if we hide nothing, we should get 7 back too
  let empty_hidden = HashSet::new();
  let mut got2 = get_conflicting_rows(&ct, &empty_hidden, 10);
  got2.sort_unstable();

  let mut expected2 = vec![3, 7, 15, 20];
  expected2.sort_unstable();
  assert_eq!(got2, expected2);

  // Additional check: selecting a row with no trues should yield no conflicts
  // (since condition requires *val && target_row[column_index])
  let got3 = get_conflicting_rows(&ct, &empty_hidden, 9);
  assert!(got3.is_empty());
}

#[test]
fn test_pick_row() {
  // TODO: Assert that this panicks for an empty input
  // Empty input -> Panic (both strategies)

  // Single element -> always that element, remainder empty (both strategies)
  let r = pick_row(vec![42], DecisionStrategy::First);
  assert_eq!(r.0, 42);
  assert!(r.1.is_empty());

  let r = pick_row(vec![42], DecisionStrategy::Random);
  assert_eq!(r.0, 42);
  assert!(r.1.is_empty());

  // Multiple elements, First strategy: selects index 0, remainder is original without that element
  let input = vec![10, 20, 30, 40];
  let (selected, remaining) = pick_row(input.clone(), DecisionStrategy::First);
  assert_eq!(selected, 10);
  assert_eq!(remaining.len(), input.len() - 1);

  // Remaining should contain all originals except selected
  let mut remaining_set: HashSet<usize> = remaining.into_iter().collect();
  assert_eq!(remaining_set.len(), 3); // no duplicates introduced
  assert!(remaining_set.remove(&20));
  assert!(remaining_set.remove(&30));
  assert!(remaining_set.remove(&40));
  assert!(remaining_set.is_empty());

  // Multiple elements, Random strategy:
  // We can't assert which element was picked, but we can assert invariants.
  let input = vec![1, 2, 3, 4, 5];
  let (selected, remaining) = pick_row(input.clone(), DecisionStrategy::Random);
  assert_eq!(remaining.len(), input.len() - 1);
  assert!(input.contains(&selected));

  // Selected + remaining should equal the original multiset (no loss/duplication)
  let mut all: HashSet<usize> = remaining.into_iter().collect();
  assert!(all.insert(selected)); // selected shouldn't already be in remaining
  let input_set: HashSet<usize> = input.into_iter().collect();
  assert_eq!(all, input_set);
}

#[test]
fn test_algorithm_x() {
  let solution = launch_algorithm_x();
  // There are 729 constraint table rows where,
  // the first 9 are for 0,0 with the first representing
  // 0,0 = 1. The second representing 0,0 = 2, and the
  // third representing 0,0 = 3.

  // To map back to a choice we do a floor division by 9, that gives us the index.
  // Then we do a modulo division by 9 then add 1, that gives us the value
  for row_index in solution {
    let index = row_index / 9;
    let val = row_index % 9 + 1;

    println!("index: {:}, has value: {:}", index, val);
  }
  assert!(false);
}
