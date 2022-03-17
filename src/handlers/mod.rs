use crate::refs::Ref;
use crate::query::Query;

pub trait Handler {

}

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
