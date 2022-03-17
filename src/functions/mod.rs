use std::collections::HashMap;
use std::fmt::Debug;

use crate::refs::{Query, Ref};
use crate::types::Format;

#[derive(Debug)]
pub struct Function {
  #[allow(dead_code)]
  argument:  Format,
  #[allow(dead_code)]
  handlers:  Vec<Box<dyn Handler>>,
  #[allow(dead_code)]
  variables: HashMap<String, Format>,
}

impl Function {
  pub fn new(
    argument:  Format,
    handlers:  Vec<Box<dyn Handler>>,
    variables: HashMap<String, Format>,
  ) -> Self {
    Self{ argument, handlers, variables }
  }
}

pub trait Handler: Debug + Send + Sync {}

#[derive(Debug)]
pub struct Set {
  #[allow(dead_code)]
  query: Query,
  #[allow(dead_code)]
  value: Box<dyn Ref>,
}

impl Set {
  pub fn new(query: Query, value: Box<dyn Ref>) -> Self {
    Self{ query, value }
  }
}

impl Handler for Set {}
