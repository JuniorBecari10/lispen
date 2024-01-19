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

#[must_use="Use this to return from a function, either by using '?' or a return statement"]
pub fn print_error<T>(message: &str, pos: Position) -> Option<T> {
    eprintln!("Error at {}:{} | {}", pos.line + 1, pos.col + 1, message);
    None
}
