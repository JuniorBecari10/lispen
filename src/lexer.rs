use crate::util;
use crate::token;
use crate::util::Position;

pub struct Lexer {
  input: Vec<char>,
  tokens: Vec<token::Token>,

  start: usize,
  current: usize,

  start_pos: util::Position,
  current_pos: util::Position,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
      Self {
        input: input.chars().to_owned().collect(),
        tokens: Vec::new(),

        start: 0,
        current: 0,

        start_pos: util::Position::default(),
        current_pos: util::Position::default(),
      }
    }

    pub fn lex(&mut self) -> (Vec<token::Token>, bool) {
      let mut had_error = false;
      
      while !self.is_at_end() {
        self.start = self.current;
        self.start_pos = self.current_pos.clone();

        if self.token().is_none() {
          had_error = true;
        }
      }

      (self.tokens.clone(), had_error)
    }

    fn token(&mut self) -> Option<()> {
      match self.advance()? {
        c if c.is_numeric() => self.number()?,
        '"' => self.string()?,

        '(' => self.add_token(token::TokenKind::LParen),
        ')' => self.add_token(token::TokenKind::RParen),

        '\n' => {
          self.current_pos.line += 1;
          self.current_pos.col = 0;
        }

        c if c.is_whitespace() => {} // test
        _ => self.identifier()?
      };

      Some(())
    }

    // ---

    fn identifier(&mut self) -> Option<()> {
      while !self.is_at_end() && is_identifier(self.peek()?) { self.advance(); }

      let mut kind = token::TokenKind::Identifier;
      
      if is_keyword(self.slice_input()) {
        kind = token::TokenKind::Keyword;
      }

      self.add_token(kind);

      Some(())
    }

    fn number(&mut self) -> Option<()> {
      while !self.is_at_end() && self.peek()?.is_numeric() { self.advance(); }

      if self.peek()? == '.' && self.peek_next()?.is_numeric() {
        self.advance();

        while self.peek()?.is_numeric() { self.advance(); }
      }

      let slice = self.slice_input();

      if slice.parse::<f64>().is_err() {
        util::print_error(&format!("Invalid number literal: '{}'", slice), self.start_pos.clone());
        return None;
      }

      self.add_token(token::TokenKind::Number);
      Some(())
    }

    fn string(&mut self) -> Option<()> {
      while !self.is_at_end() && self.peek()? != '"' {
        if self.peek()? == '\n' {
          util::print_error("Unterminated string", self.current_pos.clone());
          return None;
        }

        self.advance();
      }

      self.advance();
      self.add_token_literal(token::TokenKind::String, self.slice_range(self.start + 1, self.current - 1), self.start_pos.clone());

      Some(())
    }

    // ---

    fn is_at_end(&self) -> bool {
      self.current >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
      self.input.get(self.current).cloned()
    }

    fn peek_next(&self) -> Option<char> {
      self.input.get(self.current + 1).cloned()
    }

    fn advance(&mut self) -> Option<char> {
      let c = self.peek();
      
      self.current += 1;
      self.current_pos.col += 1;

      c
    }

    fn slice_input(&self) -> String {
      self.input[self.start..self.current].iter().collect()
    }

    fn slice_range(&self, start: usize, end: usize) -> String {
      self.input[start..end].iter().collect()
    }

    // ---

    fn add_token(&mut self, kind: token::TokenKind) {
      self.tokens.push(token::Token {
        kind,
        lexeme: self.slice_input(),
        pos: self.start_pos.clone(),
      });
    }

    fn add_token_literal(&mut self, kind: token::TokenKind, lexeme: String, pos: Position) {
      self.tokens.push(token::Token {
        kind,
        lexeme,
        pos,
      });
    }
}

fn is_identifier(c: char) -> bool {
  match c {
    c if c.is_numeric() => false,
    
      '"'
    | '('
    | ')'
    | '\n' => false,
    
    c if c.is_whitespace() => false,
    _ => true
  }
}

fn is_keyword(s: String) -> bool {
  matches!(s.as_str(), "true" | "false" | "nil")
}
