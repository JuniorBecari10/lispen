use std::collections::HashMap;

use crate::interpreter;

#[derive(Clone)]
pub struct Environment {
  values: HashMap<String, interpreter::Value>,
  enclosing: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
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

  pub fn define_variable(&mut self, name: String, value: interpreter::Value) {
    self.values.insert(name, value);
  }

  pub fn get_variable(&self, name: &str) -> Option<interpreter::Value> {
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
