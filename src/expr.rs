use crate::util;

#[derive(Debug, Clone)]
pub struct Expr {
  pub pos: util::Position,
  pub data: ExprData,
}

impl Expr {
  pub fn new(pos: util::Position, data: ExprData) -> Self {
    Self {
      pos,
      data,
    }
  }
}

#[derive(Debug, Clone)]
pub enum ExprData {
  Number(f64),
  String(String),
  Identifier(String),
  Bool(bool),
  List(Vec<Expr>, bool),
  Nil,
}
