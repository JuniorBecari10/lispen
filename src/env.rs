use std::collections::HashMap;

use crate::interpreter;

pub struct Environment<'a> {
  values: HashMap<String, interpreter::Value>,
  enclosing: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn from_enclosing(enclosing: &'a mut Environment<'a>) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(enclosing),
    }
  }

  // ---

  pub fn define_variable(&mut self, name: String, value: interpreter::Value) {
    self.values.insert(name, value);
    
    for (k, v) in self.values.iter() {
      println!("{}: {}", k, v);
    }
  }

  pub fn get_variable(&self, name: &str) -> Option<interpreter::Value> {
    let res = self.values.get(name).cloned();

    if res.is_none() {
      match self.enclosing {
        Some(e) => e.get_variable(name),
        None => None,
      }
    }
    else {
      res
    }
  }
}
