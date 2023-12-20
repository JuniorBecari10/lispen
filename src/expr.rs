use crate::util;

#[derive(Debug)]
pub struct Expr {
  pos: util::Position,
  data: ExprData,
}

impl Expr {
  pub fn new(pos: util::Position, data: ExprData) -> Self {
    Self {
      pos,
      data,
    }
  }
}

#[derive(Debug)]
pub enum ExprData {
  Number(f64),
  String(String),
  Identifier(String),
  Composite(CompositeExpr),
}

#[derive(Debug)]
pub struct CompositeExpr {
  pub name: String,
  pub args: Vec<Expr>,
}
