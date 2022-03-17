use std::collections::HashMap;

use crate::functions::Handler;
use crate::types::Format;

#[derive(Debug)]
pub struct Method {
  #[allow(dead_code)]
  argument:  Format,
  #[allow(dead_code)]
  handlers:  Vec<Box<dyn Handler>>,
  #[allow(dead_code)]
  variables: HashMap<String, Format>,
}

impl Method {
  pub fn new(
    argument:  Format,
    variables: HashMap<String, Format>,
    handlers:  Vec<Box<dyn Handler>>,
  ) -> Self {
    Self{ argument, handlers, variables }
  }
}

pub type MethodBody = (HashMap<String, Format>, Vec<Box<dyn Handler>>);
