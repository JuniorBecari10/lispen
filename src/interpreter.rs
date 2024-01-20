use std::fmt::{Display, Formatter};

use crate::{env, expr, util};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Function {
        params: Vec<String>,
        body: expr::Expr,
        env: env::Environment,
    },
    Nil,
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self.clone() {
            Value::Number(n) => n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => b,
            Value::List(l) => !l.is_empty(),
            Value::Function { params: _, body: _, env: _ } => true,
            Value::Nil => false
        }
    }
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
                params: _,
                body: _,
                env: _,
            } => write!(f, "<fn>"),
        }
    }
}

pub fn interpret(exprs: Vec<expr::Expr>, env: &mut env::Environment) {
    if exprs.len() == 1 {
        match execute(exprs[0].clone(), env) {
            Some(res) => println!("< {}", res),
            None => return,
        }
        
        return;
    }
    
    for expr in exprs {
        if execute(expr, env).is_none() {
            break;
        }
    }
}

// ---

fn execute(expr: expr::Expr, env: &mut env::Environment) -> Option<Value> {
    match expr.data {
        expr::ExprData::Number(n) => Some(Value::Number(n)),
        expr::ExprData::String(s) => Some(Value::String(s)),
        expr::ExprData::Identifier(i) => match env.get_variable(&i) {
            Some(v) => Some(v),
            None => {
                util::print_error(
                    &format!("Variable '{}' doesn't exist in this scope", &i),
                    expr.pos,
                )
            }
        },
        
        expr::ExprData::Bool(b) => Some(Value::Bool(b)),
        
        // this is caught in parser, should not run
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
                    vec.push(execute(expr, env)?);
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
                        
                        let a = execute(l[1].clone(), env)?;
                        let b = execute(l[2].clone(), env)?;
                        
                        match o.as_str() {
                            // for now, all operators will only support 2 arguments
                            "+" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
                                (Value::String(a), Value::String(b)) => Some(Value::String(format!("{}{}", a, b))),
                                
                                _ => util::print_error("Operator '+' can only be used with numbers and strings, both being of the same type", expr.pos)
                            },
                            
                            "-" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
                                _ => util::print_error("Operator '-' can only be used with numbers", expr.pos)
                            },
                            
                            "*" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
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
                            
                            ">" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a > b)),
                                _ => util::print_error("Operator '>' can only be used with numbers", expr.pos)
                            },
                            
                            ">=" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a >= b)),
                                _ => util::print_error("Operator '>=' can only be used with numbers", expr.pos)
                            },
                            
                            "<" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a < b)),
                                _ => util::print_error("Operator '<' can only be used with numbers", expr.pos)
                            },
                            
                            "<=" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a <= b)),
                                _ => util::print_error("Operator '<=' can only be used with numbers", expr.pos)
                            },
                            
                            "=" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a == b)),
                                (Value::String(a), Value::String(b)) => Some(Value::Bool(a == b)),
                                _ => util::print_error("Operator '=' can only be used with numbers and strings, both being of the same type", expr.pos)
                            },
                            
                            "!=" => match (a, b) {
                                (Value::Number(a), Value::Number(b)) => Some(Value::Bool(a != b)),
                                (Value::String(a), Value::String(b)) => Some(Value::Bool(a != b)),
                                _ => util::print_error("Operator '!=' can only be used with numbers and strings, both being of the same type", expr.pos)
                            },
                            
                            op => util::print_error(&format!("Unknown operator : '{}'", op), expr.pos)
                        }
                    }
                    
                    expr::ExprData::Keyword(k) => match k.as_str() {
                        "set" => {
                            if l.len() != 3 {
                                return util::print_error(&format!("Invalid number of arguments in 'set' expression; expected 3, got {}", l.len()), expr.pos);
                            }
                            
                            if let expr::ExprData::Identifier(name) = l[1].data.clone() {
                                let value = execute(l[2].clone(), env)?;
                                env.define_variable(name, value.clone());
                                
                                return Some(value);
                            }
                            
                            None
                        }
                        
                        // (defn name (args) (body))
                        "defn" => {
                            if l.len() != 4 {
                                return util::print_error(&format!("Invalid number of arguments in 'defn' expression; expected 4, got {}", l.len()), expr.pos);
                            }
                            
                            if let expr::ExprData::Identifier(name) = l[1].data.clone() {
                                let param_list = l[2].clone();
                                let mut string_list: Vec<String> = Vec::new();
                                
                                if let expr::ExprData::List(list, _) = param_list.data {
                                    for param in list {
                                        if let expr::ExprData::Identifier(name) = param.data.clone() {
                                            string_list.push(name);
                                        } else {
                                            return util::print_error(&format!("Invalid argument in function '{}'; expected an identifier", name), expr.pos);
                                        }
                                    }
                                    
                                    let f = Value::Function {
                                        params: string_list,
                                        body: l[3].clone(),
                                        env: env.clone(),
                                    };
                                    
                                    env.define_variable(name, f.clone());
                                    Some(f)
                                } else {
                                    util::print_error(&format!("Invalid argument for the declaration of the function '{}'; expected a list of identifiers for the parameter list", name), expr.pos)
                                }
                            }
                            else {
                                util::print_error("Function name must be an identifier", expr.pos)
                            }
                        }
                        
                        // (fn (args) (body))
                        "fn" => {
                            if l.len() != 3 {
                                return util::print_error(&format!("Invalid number of arguments in 'fn' expression; expected 3, got {}", l.len()), expr.pos);
                            }
                            
                            let param_list = l[1].clone();
                            let mut string_list: Vec<String> = Vec::new();
                            
                            if let expr::ExprData::List(list, _) = param_list.data {
                                for param in list {
                                    if let expr::ExprData::Identifier(name) = param.data.clone() {
                                        string_list.push(name);
                                    } else {
                                        return util::print_error("Invalid argument in function expression; expected an identifier", expr.pos);
                                    }
                                }
                                
                                Some(Value::Function {
                                    params: string_list,
                                    body: l[2].clone(),
                                    env: env.clone(),
                                })
                            } else {
                                util::print_error("Invalid argument for the definition of a function; expected a list of identifiers for the parameter list", expr.pos)
                            }
                        }
                        
                        // (if (condition) (then) (else?))
                        "if" => {
                            if l.len() < 3 || l.len() > 4 {
                                return util::print_error(&format!("Invalid number of arguments in 'if' expression; expected 3 or 4, got {}", l.len()), expr.pos);
                            }
                            
                            let condition = l[1].clone();
                            let then_block = l[2].clone();
                            let else_block = l.get(3).cloned();
                            
                            if execute(condition, env)?.is_truthy() {
                                execute(then_block, env)
                            }
                            else if let Some(expr) = else_block {
                                execute(expr, env)
                            }
                            else {
                                Some(Value::Nil)
                            }
                        }
                        
                        // (while (condition) (body))
                        "while" => {
                            if l.len() != 3 {
                                return util::print_error(&format!("Invalid number of arguments in 'while' expression; expected 3, got {}", l.len()), expr.pos);
                            }
                            
                            let condition = l[1].clone();
                            let body = l[2].clone();
                            
                            while execute(condition.clone(), env)?.is_truthy() {
                                execute(body.clone(), env);
                            }
                            
                            Some(Value::Nil)
                        }
                        
                        kw => util::print_error(&format!("Keyword '{}' cannot be used as instruction", kw), expr.pos)
                    },
                    
                    expr::ExprData::Identifier(name) => {
                        let function = match env.get_variable(&name) {
                            Some(f) => f,
                            None => { return util::print_error(&format!("Variable '{}' doesn't exist in this scope", &name), expr.pos); }
                        };
                        
                        if let Value::Function { params, body, env: fn_env } = function {
                            let mut new_env = env::Environment::from_enclosing(fn_env);
                            
                            if l.len() - 1 != params.len() {
                                return util::print_error(&format!("Invalid number of arguments; expected {}, got {}", params.len(), l.len()), expr.pos);
                            }
                            
                            for (i, param) in params.iter().enumerate() {
                                let arg = execute(l[i + 1].clone(), env)?;
                                new_env.define_variable(param.into(), arg.clone());
                            }
                            
                            let res = execute(body, &mut new_env)?;
                            return Some(res);
                        }
                        
                        util::print_error(&format!("Value '{}' isn't a function", &name), expr.pos)
                    }
                    
                    expr::ExprData::Nil => Some(Value::Nil),
                    
                    _ => {
                        let mut vec = vec![];
                        for expr in l {
                            vec.push(execute(expr, env)?);
                        }
                        
                        Some(Value::List(vec))
                    }
                }
            }
        }
        
        expr::ExprData::Nil => Some(Value::Nil),
    }
}
