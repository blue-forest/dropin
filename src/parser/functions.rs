use pest::iterators::Pairs;

use crate::functions::{Handler, Set};
use crate::refs::Query;

use super::{read_value, Rule};

pub fn read_handlers(pairs: Pairs<Rule>) -> Vec<Box<dyn Handler>> {
  // print_pairs(pairs, 0);
  let mut handlers: Vec<Box<dyn Handler>> = Vec::new();
  for pair in pairs {
    let handler = match pair.as_rule() {
      Rule::set => read_set(pair.into_inner()),
      _ => { panic!("unknown handler: {}", pair.as_str()) }
    };
    handlers.push(Box::new(handler));
  }
  handlers
}

pub fn read_set(mut pairs: Pairs<Rule>) -> Set {
  let query = pairs.next().expect("expected query").as_str();
  let value = read_value(
    pairs.next().expect("expected value").into_inner()
  );
  Set::new(Query::new(query.to_string()), value)
}
