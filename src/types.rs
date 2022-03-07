use std::collections::HashMap;
use std::sync::Arc;

use crate::refs::{Object, Ref};

pub struct Type {
  templates: HashMap<String, Format>,
}

impl Type {
  pub fn new() -> Self {
    Self{
      templates: HashMap::new(),
    }
  }

  pub fn add_template(&mut self, key: String, format: Format) {
    self.templates.insert(key, format);
  }
}

pub struct Format {
  type_:   Arc<Type>,
  format:  HashMap<String, Format>,
  options: Object,
}

impl Format {
  pub fn new(type_: Arc<Type>) -> Self {
    Self{
      type_,
      format:  HashMap::new(),
      options: Object::new(),
    }
  }

  pub fn set_format(&mut self, format: Format) {
    if !self.format.is_empty() {
      panic!("trying to set an existing format");
    }
    self.format.insert("".to_string(), format);
  }

  pub fn add_format(&mut self, key: String, format: Format) {
    if let Some(_) = self.format.insert(key, format) {
      panic!("trying to set an existing key format");
    }
  }
}
