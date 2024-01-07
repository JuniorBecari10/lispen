use std::fmt::{Display, Formatter};

use crate::{env, expr, util};

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
        env: env::Environment,
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
            }
            Value::Nil => write!(f, "nil"),
            Value::Function {
                name,
                params: _,
                body: _,
                env: _,
            } => write!(f, "<fn {}>", name),
        }
    }
}

pub struct Interpreter {
    exprs: Vec<expr::Expr>,
    env: env::Environment,
}

impl Interpreter {
    pub fn new(exprs: Vec<expr::Expr>, env: env::Environment) -> Self {
        Self { exprs, env }
    }

    pub fn interpret(&mut self) -> env::Environment {
        if self.exprs.len() == 1 {
            match self.execute(self.exprs[0].clone()) {
                Some(res) => println!("< {}", res),
                None => return self.env.clone(),
            }

            return self.env.clone();
        }

        for expr in self.exprs.clone() {
            if self.execute(expr).is_none() {
                break;
            }
        }

        self.env.clone()
    }

    // ---

    fn execute(&mut self, expr: expr::Expr) -> Option<Value> {
        match expr.data {
            expr::ExprData::Number(n) => Some(Value::Number(n)),
            expr::ExprData::String(s) => Some(Value::String(s)),
            expr::ExprData::Identifier(i) => match self.env.get_variable(&i) {
                Some(v) => Some(v),
                None => {
                    util::print_error(
                        &format!("Variable '{}' doesn't exist in this scope", &i),
                        expr.pos,
                    )
                }
            },
            expr::ExprData::Bool(b) => Some(Value::Bool(b)),

            expr::ExprData::Operator(_) => {
                util::print_error("Operators cannot be used as values, only as instructions, by placing them as the first argument in lists", expr.pos)
            }

            expr::ExprData::Keyword(_) => {
                util::print_error("Keywords cannot be used as values, only as instructions, by placing them as the first argument in lists", expr.pos)
            }

            expr::ExprData::List(l, is_quote) => {
                if l.is_empty() {
                    return Some(Value::List(vec![]));
                }

                if is_quote {
                    let mut vec = vec![];
                    for expr in l {
                        vec.push(self.execute(expr)?);
                    }

                    Some(Value::List(vec))
                } else {
                    match l[0].data.clone() {
                        expr::ExprData::Operator(o) => {
                            if l.len() != 3 {
                                // it's 2, but the operator also counts as an argument
                                return util::print_error(
                                    &format!(
                                        "Expected 2 arguments for the operator, got {}",
                                        l.len() - 1
                                    ),
                                    expr.pos,
                                );
                            }

                            let a = self.execute(l[1].clone())?;
                            let b = self.execute(l[2].clone())?;

                            match o.as_str() {
                                // for now, all operators will only support 2 arguments
                                "+" => match (a, b) {
                                    (Value::Number(a), Value::Number(b)) => {
                                        Some(Value::Number(a + b))
                                    }

                                    (Value::String(a), Value::String(b)) => {
                                        Some(Value::String(format!("{}{}", a, b)))
                                    }

                                    _ => util::print_error("Operator '+' can only be used with numbers and strings, both being of the same type", expr.pos)
                                },

                                "-" => match (a, b) {
                                    (Value::Number(a), Value::Number(b)) => {
                                        Some(Value::Number(a - b))
                                    }

                                    _ => util::print_error("Operator '-' can only be used with numbers", expr.pos)
                                },

                                "*" => match (a, b) {
                                    (Value::Number(a), Value::Number(b)) => {
                                        Some(Value::Number(a * b))
                                    }

                                    _ => util::print_error("Operator '*' can only be used with numbers", expr.pos)
                                },

                                "/" => match (a, b) {
                                    (Value::Number(a), Value::Number(b)) => {
                                        if b == 0.0 {
                                            return util::print_error("Cannot divide by zero", expr.pos);
                                        }

                                        Some(Value::Number(a / b))
                                    }

                                    _ => util::print_error("Operator '-' can only be used with numbers", expr.pos)
                                },

                                op => util::print_error(&format!("Unknown operator : '{}'", op), expr.pos)
                            }
                        }

                        expr::ExprData::Keyword(k) => match k.as_str() {
                            "let" => {
                                if l.len() != 3 {
                                    return util::print_error(&format!("Invalid number of arguments in 'let' expression; expected 3, got {}", l.len()), expr.pos);
                                }

                                if let expr::ExprData::Identifier(name) = l[1].data.clone() {
                                    let value = self.execute(l[2].clone())?;
                                    self.env.define_variable(name, value.clone());

                                    return Some(value);
                                }

                                None
                            }

                            // (fn name (args) (body))
                            "fn" => {
                                if l.len() != 4 {
                                    return util::print_error(&format!("Invalid number of arguments in 'fn' expression; expected 4, got {}", l.len()), expr.pos);
                                }

                                if let expr::ExprData::Identifier(name) = l[1].data.clone() {
                                    let param_list = l[2].clone();
                                    let mut string_list: Vec<String> = Vec::new();

                                    if let expr::ExprData::List(list, _) = param_list.data.clone() {
                                        for param in list {
                                            if let expr::ExprData::Identifier(name) = param.data.clone() {
                                                string_list.push(name);
                                            } else {
                                                return util::print_error(&format!("Invalid argument in function '{}'; expected an identifier", name), expr.pos);
                                            }
                                        }

                                        let f = Value::Function {
                                            name: name.clone(),
                                            params: string_list,
                                            body: l[3].clone(),
                                            env: self.env.clone(),
                                        };

                                        self.env.define_variable(name, f.clone());
                                        Some(f)
                                    } else {
                                        util::print_error(&format!("Invalid argument for the declaration of the function '{}'; expected a list of identifiers for the parameter list", name), expr.pos)
                                    }
                                }
                                else {
                                    util::print_error("Function name must be an identifier", expr.pos)
                                }
                            }

                            kw => util::print_error(&format!("Keyword '{}' cannot be used as instruction", kw), expr.pos)
                        },

                        expr::ExprData::Identifier(name) => {
                            let function = match self.env.get_variable(&name) {
                                Some(f) => f,
                                None => { return util::print_error(&format!("Variable '{}' doesn't exist in this scope", &name), expr.pos); }
                            };

                            if let Value::Function { name: _, params, body, env } = function {
                                let old_env = self.env.clone();
                                self.env = env::Environment::from_enclosing(env);

                                if l.len() - 1 != params.len() {
                                    return util::print_error(&format!("Invalid number of arguments; expected {}, got {}", params.len(), l.len()), expr.pos);
                                }

                                for (i, param) in params.iter().enumerate() {
                                    let arg = self.execute(l[i].clone())?;
                                    self.env.define_variable(param.into(), arg);
                                }

                                let res = self.execute(body)?;
                                self.env = old_env;

                                return Some(res);
                            }

                            util::print_error(&format!("Value '{}' isn't a function", &name), expr.pos)
                        }

                        expr::ExprData::Nil => Some(Value::Nil),

                        _ => {
                            let mut vec = vec![];
                            for expr in l {
                                vec.push(self.execute(expr)?);
                            }

                            Some(Value::List(vec))
                        }
                    }
                }
            }

            expr::ExprData::Nil => Some(Value::Nil),
        }
    }
}
