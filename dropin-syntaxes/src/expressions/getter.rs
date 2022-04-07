use std::str::CharIndices;

use super::Expression;

#[derive(Debug)]
pub struct Getter<'a> {
  #[allow(dead_code)]
  query: &'a str,
}

impl<'a> Getter<'a> {
  pub fn parse(
    syntax: &'a str,
    iter:   &mut CharIndices<'a>,
  ) -> Box<dyn Expression + 'a> {
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    while let Some((i, c)) = iter.next() {
      if start.is_none() {
        start = Some(i);
      }
      if c.is_whitespace() {
        end = Some(i);
        break;
      }
    }
    let query = syntax.get(
      start.expect("expected query")..end.unwrap_or(syntax.len())
    ).unwrap();
    Box::new(Getter{ query })
  }
}

impl<'a> Expression for Getter<'a> {}

