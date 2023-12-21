use std::io::{self, Write};

#[derive(Debug, Clone, Default)]
pub struct Position {
  pub line: usize,
  pub col: usize,
}

pub fn input(prompt: &str, out: &mut String) {
  print!("{}", prompt);

  io::stdout().flush().unwrap();
  io::stdin().read_line(out).unwrap();

  *out = out.trim().to_owned();
}

pub fn print_error(message: &str, pos: Position) -> Option<()> {
  eprintln!("Error at {}:{} | {}", pos.line + 1, pos.col + 1, message);
  None
}
