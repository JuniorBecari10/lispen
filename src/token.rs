use crate::util;

#[derive(Debug, Clone)]
pub enum TokenKind {
  Identifier,
  Number,
  String,
  Keyword,
  
  LParen,
  RParen,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub pos: util::Position,
}
