use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}, process::Command, io::Write};

use crate::interpreter::{Value, Function};

#[derive(Clone)]
pub struct Environment {
  values: HashMap<String, Value>,
  enclosing: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      values: hashmap_with_native_fns(),
      enclosing: None,
    }
  }

  pub fn from_enclosing(enclosing: Environment) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(Box::new(enclosing)),
    }
  }

  // ---

  /*
  // For debugging. Uncomment when necessary.
  pub fn print_variables(&self) {
    for (k, v) in self.values.iter() {
      println!("{}: {}", k, v);
    }
  }
  */

  pub fn define_variable(&mut self, name: String, value: Value) {
    self.values.insert(name, value);
  }

  pub fn get_variable(&self, name: &str) -> Option<Value> {
    let res = self.values.get(name).cloned();

    if res.is_none() {
      match self.enclosing.clone() {
        Some(e) => e.get_variable(name),
        None => None,
      }
    }
    else {
      res
    }
  }
}

fn hashmap_with_native_fns() -> HashMap<String, Value> {
  HashMap::from([
    // -- Prelude --

    ("println".into(), Value::Function(Function::NativeFn { arity: 1, call: |args| {
      println!("{}", args[0]);
      Some(Value::Nil)
    } })),

    ("print".into(), Value::Function(Function::NativeFn { arity: 1, call: |args| {
      print!("{}", args[0]);
      Some(Value::Nil)
    } })),

    ("println_blank".into(), Value::Function(Function::NativeFn { arity: 0, call: |_| {
      println!();
      Some(Value::Nil)
    } })),

    ("time_ms".into(), Value::Function(Function::NativeFn { arity: 0, call: |_| {
      Some(Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_millis() as f64))
    } })),

    ("time_sec".into(), Value::Function(Function::NativeFn { arity: 0, call: |_| {
      Some(Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as f64))
    } })),

    ("execute".into(), Value::Function(Function::NativeFn { arity: 1, call: |args| {
      if let Value::String(s) = args[0].clone() {
        let output = if cfg!(target_os = "windows") {
          Command::new("cmd")
                  .arg("/c")
                  .arg(s)
                  .output()
                  .ok()?
        }
        else {
          Command::new("sh")
                  .arg("-c")
                  .arg(s)
                  .output()
                  .ok()?
        };
  
        Some(Value::String(String::from_utf8_lossy(output.stdout.as_slice()).into_owned()))
      }
      else {
        None
      }
    } })),

    ("execute_exit_code".into(), Value::Function(Function::NativeFn { arity: 1, call: |args| {
      if let Value::String(s) = args[0].clone() {
        let output = if cfg!(target_os = "windows") {
          Command::new("cmd")
                  .arg("/c")
                  .arg(s)
                  .output()
                  .ok()?
        }
        else {
          Command::new("sh")
                  .arg("-c")
                  .arg(s)
                  .output()
                  .ok()?
        };
  
        Some(Value::Number(output.status.code()? as f64))
      }
      else {
        None
      }
    } })),

    ("input".into(), Value::Function(Function::NativeFn { arity: 1, call: |args| {
      print!("{}", args[0]);

      let mut input = String::new();

      std::io::stdout().flush().unwrap();
      std::io::stdin().read_line(&mut input).unwrap();

      Some(Value::String(input))
    } })),
  ])
}
