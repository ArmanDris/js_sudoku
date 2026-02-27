use super::*;

#[test]
fn test_generate_column_headers() {
  let constraint_table = generate_constraint_table();
  let headers = generate_column_headers(&constraint_table);
  let correct_header = Link::ColumnHeader(ColumnHeader {
    cell_count: 9,
    up: None,
    down: None,
    left: None,
    right: None,
  });

  for header in headers {
    assert_eq!(header, correct_header);
  }
}

#[test]
fn test_generate_linked_rows() {
  let constraint_table = generate_constraint_table();
  let rows = generate_linked_rows(&constraint_table);

  let num_cells_first_row =
    rows[0].iter().filter(|x| **x != Link::EmptyLink).count();

  assert_eq!(num_cells_first_row, 4);

  let mut num_cells_last_column = 0;
  for index in 0..729 {
    if rows[index][323] != Link::EmptyLink {
      num_cells_last_column += 1;
    }
  }

  assert_eq!(num_cells_last_column, 9);
}

use std::mem::{align_of, size_of, size_of_val};

#[test]
fn test_mem() {
  let linked_row = [Link::EmptyLink; 324];

  println!("size_of::<Link>() = {}", size_of::<Link>());
  println!("align_of::<Link>() = {}", align_of::<Link>());

  println!("size_of(linked_row) = {}", size_of_val(&linked_row));

  let boxed = Box::new(linked_row);
  println!("size_of(boxed_linked_row) = {}", size_of_val(&boxed));
  println!("size_of(*boxed_linked_row) = {}", size_of_val(&*boxed));

  todo!()
}
