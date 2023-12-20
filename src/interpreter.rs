use std::fmt::{Display, Formatter};

use crate::expr;

pub enum Value {
  Number(f64),
  String(String),
  Bool(bool),
  Nil,
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    match *self {
        Value::Number(n) => write!(f, "{}", n),
        Value::String(s) => write!(f, "{}", s),
        Value::Bool(b) => write!(f, "{}", b),
        Value::Nil => write!(f, "nil"),
    }
  }
}

pub struct Interpreter {
  exprs: Vec<expr::Expr>,
  current: usize,
}

impl Interpreter {
  pub fn new(exprs: Vec<expr::Expr>) -> Self {
    Self {
      exprs,
      current: 0,
    }
  }

  pub fn interpret(&mut self) {
    if self.exprs.len() == 1 {
      println!("< {}", self.execute(self.exprs[0]));
      return;
    }

    for expr in self.exprs {
      self.execute(expr);
    }
  }

  // ---

  fn execute(&mut self, expr: expr::Expr) -> Value {
    
  }

  // ---

  fn is_at_end(&self) -> bool {
    self.current >= self.exprs.len()
  }
}
