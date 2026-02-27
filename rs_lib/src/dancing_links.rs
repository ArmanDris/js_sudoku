use crate::algorithm_x::{generate_constraint_table, ConstraintTable};
use std::array;

#[cfg(test)]
#[path = "dancing_links_test.rs"]
mod dancing_links_test;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
  column_index: usize,
  row_index: usize,
  up: Option<usize>,
  down: Option<usize>,
  left: Option<usize>,
  right: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ColumnHeader {
  cell_count: i32,
  up: Option<usize>,
  down: Option<usize>,
  left: Option<usize>,
  right: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Link {
  ColumnHeader(ColumnHeader),
  Cell(Cell),
  EmptyLink,
}

struct LinkedTable {
  table: [[Link; 324]; 730],
}

impl Default for LinkedTable {
  fn default() -> Self {
    LinkedTable {
      table: [[Link::EmptyLink; 324]; 730],
    }
  }
}

/// Generates a row of column headers with the correct
/// cell counts. DOES NOT INITIALIZE UP, DOWN, LEFT
/// RIGHT POINTERS, THOSE ARE LEFT AS `None`
fn generate_column_headers(constraint_table: &ConstraintTable) -> [Link; 324] {
  let column_cell_counts = array::from_fn::<i32, 324, _>(|row_index| {
    let mut count = 0;
    for column_index in 0..729 {
      if constraint_table.table[column_index][row_index] {
        count += 1;
      }
    }

    count
  });

  array::from_fn::<Link, 324, _>(|row_index| {
    Link::ColumnHeader(ColumnHeader {
      cell_count: column_cell_counts[row_index],
      up: None,
      down: None,
      left: None,
      right: None,
    })
  })
}

fn generate_linked_rows(
  constraint_table: &ConstraintTable,
) -> Box<[[Link; 324]; 729]> {
  let mut linked_rows: Vec<[Link; 324]> = vec![];

  for (row_idx, row) in constraint_table.table.iter().enumerate() {
    let mut current_linked = [Link::EmptyLink; 324];
    for (col_idx, cell) in row.iter().enumerate() {
      if !cell {
        continue;
      }

      current_linked[col_idx] = Link::Cell(Cell {
        column_index: col_idx,
        row_index: row_idx,
        up: None,
        down: None,
        left: None,
        right: None,
      })
    }
    linked_rows.push(current_linked);
  }

  let linked_arm: Box<[[Link; 324]; 729]> = linked_rows.try_into().unwrap();

  linked_arm
}

/// HAHAHAH
fn link_linked_table(mut linked_table: LinkedTable) -> LinkedTable {
  for row_idx in 0..730 {
    // We know col[0] is the ColumHeader so it is the first link
    let mut first_link_index: Option<usize> = None;
    let mut last_link_index: Option<usize> = None;

    for col_idx in 0..324 {
      let curr_cell = &mut linked_table.table[row_idx][col_idx];

      match curr_cell {
        Link::EmptyLink => (),
        Link::ColumnHeader(col_head) => col_head.right = last_link_index,
        Link::Cell(cell) => cell.right = last_link_index,
      }

      if curr_cell == &Link::EmptyLink {
        continue;
      }

      last_link_index = Some(col_idx);
      if first_link_index.is_none() {
        first_link_index = Some(col_idx);
      }
    }
    // Both of these should be initialized, if not, something bigger wrong
    let first_link_index = first_link_index.unwrap();
    let last_link_index = last_link_index.unwrap();

    let first_cell = &mut linked_table.table[row_idx][first_link_index];

    match first_cell {
      Link::EmptyLink => (),
      Link::ColumnHeader(cell_head) => cell_head.left = Some(last_link_index),
      Link::Cell(cell) => cell.left = Some(last_link_index),
    }

    let last_cell = &mut linked_table.table[row_idx][last_link_index];
    match last_cell {
      Link::EmptyLink => (),
      Link::ColumnHeader(cell_head) => cell_head.right = Some(first_link_index),
      Link::Cell(cell) => cell.right = Some(first_link_index),
    }
  }
  todo!()
}

fn generate_linked_table() -> LinkedTable {
  let constraint_table = generate_constraint_table();
  let mut linked_table = LinkedTable::default();
  linked_table.table[0] = generate_column_headers(&constraint_table);
  linked_table.table[1..]
    .clone_from_slice(&*generate_linked_rows(&constraint_table));

  todo!();
}

pub fn launch_dancing_links() {
  let linked_tablre = generate_linked_table();
}
