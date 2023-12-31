use crate::util;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Identifier,
  Number,
  String,
  Keyword,
  Operator,
  
  LParen,
  RParen,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: String,
  pub pos: util::Position,
}
