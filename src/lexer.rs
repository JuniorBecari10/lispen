use crate::util;
use crate::token;

pub struct Lexer {
  input: Vec<char>,

  start: usize,
  current: usize,

  start_pos: util::Position,
  current_pos: util::Position,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
      Self {
        input: input.chars().to_owned().collect(),

        start: 0,
        current: 0,

        start_pos: util::Position::default(),
        current_pos: util::Position::default(),
      }
    }

    pub fn lex(&mut self) -> (Vec<token::Token>, bool) {
      let mut tokens: Vec<token::Token> = Vec::new();
      let mut had_error = false;
      
      while !self.is_at_end() {
        match self.token() {
          Some(t) => tokens.push(t),
          None => { had_error = true; }
        }
      }

      (tokens, had_error)
    }

    fn token(&mut self) -> Option<token::Token> {

    }

    // ---

    fn is_at_end(&self) -> bool {
      self.current >= self.input.len()
    }

    fn peek(&self) -> char {
      self.input.get(self.current)
        .cloned()
        .unwrap_or('\0')
    }
}