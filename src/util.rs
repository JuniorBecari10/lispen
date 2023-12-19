use std::io::{self, Write};

#[derive(Debug)]
pub struct Position {
  pub line: usize,
  pub col: usize,
}

impl Default for Position {
  fn default() -> Self {
      Self {
        line: 0,
        col: 0,
      }
  }
}

pub fn input(prompt: &str, out: &mut String) {
  print!("{}", prompt);

  io::stdout().flush().unwrap();
  io::stdin().read_line(out).unwrap();
}

pub fn print_error(message: &str, pos: Position) -> Option<()> {
  eprintln!("Error at {}:{} | {}", pos.line, pos.col, message);
  None
}
