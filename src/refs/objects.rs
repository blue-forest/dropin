use std::collections::HashMap;
use std::sync::Arc;

use super::Ref;

#[derive(Debug)]
pub struct Object{
  data: HashMap<String, Arc<dyn Ref>>,
}

impl Object {
  pub fn new() -> Self {
    Self{
      data: HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: String, value: Arc<dyn Ref>) {
    if let Some(old_value) = self.data.insert(key, value) {
      panic!("overriding object value {:?}", old_value);
    }
  }
}

impl Ref for Object {
}
