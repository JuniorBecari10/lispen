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

    pub fn lex(&mut self) -> Result<Vec<token::Token>, ()> {
      let mut had_error = false;
      
      while !self.is_at_end() {
        self.start = self.current;
        self.start_pos = self.current_pos.clone();

        if self.token().is_none() {
          had_error = true;
        }
      }

      if had_error {
        Err(())
      }
      else {
        Ok(self.tokens.clone())
      }
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

      let kind = match &self.slice_input() {
        s if is_keyword(s) => token::TokenKind::Keyword,
        s if is_operator(s) => token::TokenKind::Operator,

        _ => token::TokenKind::Identifier
      };

      self.add_token(kind);
      Some(())
    }

    fn number(&mut self) -> Option<()> {
      while !self.is_at_end() && self.peek()?.is_numeric() { self.advance(); }

      if !self.is_at_end() && self.peek()? == '.' && self.peek_next()?.is_numeric() {
        self.advance();

        while !self.is_at_end() && self.peek()?.is_numeric() { self.advance(); }
      }

      let slice = self.slice_input();

      if slice.parse::<f64>().is_err() {
        return util::print_error(&format!("Invalid number literal: '{}'", slice), self.start_pos.clone());
      }

      self.add_token(token::TokenKind::Number);
      Some(())
    }

    fn string(&mut self) -> Option<()> {
      while !self.is_at_end() && self.peek()? != '"' {
        if self.peek()? == '\n' {
          return util::print_error("Unterminated string", self.current_pos.clone());
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

fn is_keyword(s: &str) -> bool {
  matches!(s, "set" | "fn" | "defn" | "if" | "while" | "true" | "false" | "nil")
}

fn is_operator(s: &str) -> bool {
  matches!(s, "+" | "-" | "*" | "/" | ">" | ">=" | "<" | "<=" | "=" | "!=")
}
