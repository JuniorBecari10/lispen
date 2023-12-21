use std::collections::HashMap;

use crate::interpreter;

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

  pub fn get_variable(&self, name: &str) -> Option<interpreter::Value> {
    self.values.get(name).cloned()
  }
}
