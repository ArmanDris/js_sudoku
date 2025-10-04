use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub struct Board {
  cells: [i32; 81],
}

impl Default for Board {
  fn default() -> Self {
    Self { cells: [0; 81] }
  }
}

#[wasm_bindgen]
impl Board {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set(&mut self, x: usize, y: usize, value: i32) {
    self.cells[y * 9 + x] = value;
  }

  pub fn get(&self, x: usize, y: usize) -> i32 {
    self.cells[y * 9 + x]
  }

  pub fn print_board(&self) {
    for row in 0..9 {
      let mut row_string = String::from("Hello");
      for cell in 0..9 {
        row_string.push_str(&self.cells[row * 9 + cell].to_string());
      }
      println!("{}", row_string);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }

  #[test]
  fn it_greets() {
    let mut greeter = Board::new();
    greeter.set(0, 5, 1);
    assert_eq!(greeter.get(0, 5), 1);
  }
}
