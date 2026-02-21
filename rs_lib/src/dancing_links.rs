use crate::algorithm_x::{generate_constraint_table, ConstraintTable};
use std::array;

#[cfg(test)]
#[path = "dancing_links_test.rs"]
mod dancing_links_test;

struct Cell {
  column_index: i32,
  row_index: i32,
  up: Option<usize>,
  down: Option<usize>,
  left: Option<usize>,
  right: Option<usize>,
}

#[derive(Debug, PartialEq)]
struct ColumnHeader {
  cell_count: i32,
  up: Option<usize>,
  down: Option<usize>,
  left: Option<usize>,
  right: Option<usize>,
}

enum Link {
  ColumnHeader,
  Cell,
}

struct LinkedTable {
  table: [[Link; 324]; 730],
}

/// Generates a row of column headers with the correct
/// cell counts. DOES NOT INITIALIZE UP, DOWN, LEFT
/// RIGHT POINTERS, THOSE ARE LEFT AS `None`
fn generate_column_headers(
  constraint_table: ConstraintTable,
) -> [ColumnHeader; 324] {
  let column_cell_counts = array::from_fn::<i32, 324, _>(|row_index| {
    let mut count = 0;
    for column_index in 0..729 {
      if constraint_table.table[column_index][row_index] {
        count += 1;
      }
    }

    count
  });

  array::from_fn::<ColumnHeader, 324, _>(|row_index| ColumnHeader {
    cell_count: column_cell_counts[row_index],
    up: None,
    down: None,
    left: None,
    right: None,
  })
}

fn generate_linked_table() -> LinkedTable {
  let constraint_table = generate_constraint_table();
  let column_headers = generate_column_headers(constraint_table);

  todo!();
}

pub fn launch_dancing_links() {
  let linked_tablre = generate_linked_table();
}
