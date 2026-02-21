use super::*;

#[test]
fn test_generate_column_headers() {
  let constraint_table = generate_constraint_table();
  let headers = generate_column_headers(constraint_table);
  let correct_header = ColumnHeader {
    cell_count: 9,
    up: None,
    down: None,
    left: None,
    right: None,
  };

  for header in headers {
    assert_eq!(header, correct_header);
  }
}
