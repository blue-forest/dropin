use std::iter::Peekable;
use std::str::CharIndices;

use super::Expression;

#[derive(Debug)]
pub enum Quantifier<'a> {
  AtLeastOne(Box<dyn Expression + 'a>),
}

impl<'a> Quantifier<'a> {
  pub fn detect(c: char) -> bool {
    c == '+'
  }

  pub fn new(
    iter:   &mut Peekable<CharIndices<'a>>,
    expr:   Box<dyn Expression + 'a>
  ) -> Self {
    let (_, c) = iter.next().unwrap();
    match c {
      '+' => Self::AtLeastOne(expr),
      _   => { unreachable!() }
    }
  }
}

impl<'a> Expression for Quantifier<'a> {}
