use pest::iterators::Pairs;

use super::{read_value, Rule};

pub fn read_handlers(pairs: Pairs<Rule>) {
  // print_pairs(pairs, 0);
  for pair in pairs {
    match pair.as_rule() {
      Rule::set => set(pair.into_inner()),
      _ => { panic!("unknown handler: {}", pair.as_str()) }
    }
  }
}

pub fn set(mut pairs: Pairs<Rule>) {
  let query = pairs.next().expect("expected query").as_str();
  let value = read_value(
    pairs.next().expect("expected value").into_inner()
  );
  todo!()
}
