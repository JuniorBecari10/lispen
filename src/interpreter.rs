use std::fmt::{Display, Formatter};

use crate::{expr, env, util};

#[derive(Clone)]
pub enum Value {
  Number(f64),
  String(String),
  Bool(bool),
  List(Vec<Value>),
  Function {
    name: String,
    params: Vec<String>,
    body: expr::Expr,
  },
  Nil,
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
        Value::Number(n) => write!(f, "{}", *n),
        Value::String(s) => write!(f, "{}", *s),
        Value::Bool(b) => write!(f, "{}", *b),
        Value::List(l) => {
          write!(f, "(").unwrap();

          for (count, i) in l.iter().enumerate() {
            write!(f, "{}", i).unwrap();

            if count < l.len() - 1 {
              write!(f, " ").unwrap();
            }
          }

          write!(f, ")").unwrap();
          Ok(())
        },
        Value::Nil => write!(f, "nil"),
        Value::Function { name: _, params: _, body: _ } => write!(f, "<fn>"),
    }
  }
}

pub struct Interpreter<'a> {
  exprs: Vec<expr::Expr>,
  env: &'a mut env::Environment,
}

impl<'a> Interpreter<'a> {
  pub fn new(exprs: Vec<expr::Expr>, env: &'a mut env::Environment) -> Self {
    Self {
      exprs,
      env,
    }
  }

  pub fn interpret(&mut self) {
    if self.exprs.len() == 1 {
      match self.execute(self.exprs[0].clone()) {
        Some(res) => println!("< {}", res),
        None => return
      }

      return;
    }

    for expr in self.exprs.clone() {
      if self.execute(expr).is_none() {
        break;
      }
    }
  }

  // ---

  fn execute(&mut self, expr: expr::Expr) -> Option<Value> {
    match expr.data {
        expr::ExprData::Number(n) => Some(Value::Number(n)),
        expr::ExprData::String(s) => Some(Value::String(s)),
        expr::ExprData::Identifier(i) => {
          match self.env.get_variable(&i) {
            Some(v) => Some(v),
            None => {
              util::print_error(&format!("Variable '{}' doesn't exist in this scope", &i), expr.pos);
              None
            }
          }
        },
        expr::ExprData::Bool(b) => Some(Value::Bool(b)),

        expr::ExprData::Keyword(_) => {
          util::print_error("Keywords cannot be used as values, only as instructions, by placing them as the first argument in lists", expr.pos);
          None
        },

        expr::ExprData::List(l, is_quote) => {
          if l.is_empty() {
            return Some(Value::List(vec![]));
          }

          match l[0].data.clone() {
            expr::ExprData::Keyword(k) => {
              match k.as_str() {
                "let" => {
                  if l.len() != 3 {
                    util::print_error(&format!("Invalid number of arguments in 'let' expression; expected 3, got {}", l.len()), expr.pos);
                    return None;
                  }
  
                  if let expr::ExprData::Identifier(name) = l[1].data.clone() {
                    let value = self.execute(l[2].clone())?;
                    self.env.define_variable(name, value.clone());

                    return Some(value);
                  }

                  None
                },
  
                "fn" => {
                  todo!()
                },
  
                kw => {
                  util::print_error(&format!("Keyword '{}' cannot be used as instruction", kw), expr.pos);
                  None
                }
              }
            }

            expr::ExprData::Identifier(name) => {
              if !is_quote {
                let function = match self.env.get_variable(&name) {
                  Some(f) => f,
                  None => {
                    util::print_error(&format!("Variable '{}' doesn't exist in this scope", &name), expr.pos);
                    return None;
                  }
                };
  
                if let Value::Function { name, params, body } = function {
                  
                }
  
                util::print_error(&format!("Value '{}' isn't a function", &name), expr.pos);
                return None;
              }

              None
            },

            _ => {
              let mut vec = vec![];
              for expr in l {
                vec.push(self.execute(expr)?);
              }

              Some(Value::List(vec))
            }
          }
        },
        expr::ExprData::Nil => Some(Value::Nil),
    }
  }
}
