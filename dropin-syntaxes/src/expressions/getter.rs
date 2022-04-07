use std::iter::Peekable;
use std::str::CharIndices;

use super::{Expression, Quantifier};

#[derive(Debug)]
pub struct Getter<'a> {
  #[allow(dead_code)]
  query: &'a str,
}

impl<'a> Getter<'a> {
  pub fn parse(
    syntax: &'a str,
    iter:   &mut Peekable<CharIndices<'a>>,
  ) -> Box<dyn Expression + 'a> {
    let mut start: Option<usize> = None;
    loop {
      let next = iter.next();
      if next.is_none() {
        break Box::new(Getter{
          query: syntax.get(start.expect("expected query")..).unwrap(),
        });
      }
      let (i, c) = next.unwrap();
      if start.is_none() {
        if !c.is_alphanumeric() { // $ alone ?
          panic!("unexpected query {}", c);
        }
        start = Some(i);
        continue;
      }
      if let Some((pi, pc)) = iter.peek() {
        if pc.is_whitespace() {
          break Box::new(Getter{
            query: syntax.get(start.expect("expected query")..*pi).unwrap(),
          });
        }
        if Quantifier::detect(*pc) {
          let end = *pi;
          break {
            Box::new(Quantifier::new(iter, Box::new(Getter{
              query: syntax.get(start.expect("expected query")..end).unwrap(),
            })))
          };
        }
      }
    }
  }
}

impl<'a> Expression for Getter<'a> {}

