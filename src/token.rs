use crate::util;

#[derive(Debug)]
pub enum TokenKind {
  Identifier,
  Number,
  String,
  
  LParen,
  RParen,
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub pos: util::Position,
}
