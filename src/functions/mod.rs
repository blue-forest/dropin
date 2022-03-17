use std::fmt::Debug;

use crate::refs::{Query, Ref};

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
