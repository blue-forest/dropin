use std::iter::Peekable;
use std::str::CharIndices;

use crate::syntaxes::{Expression, ParseError, Patterns};
use super::Token;

#[derive(Debug)]
pub enum Quantifier<'a> {
  AtLeastOne(Box<dyn Token<'a> + 'a>),
}

impl<'a> Quantifier<'a> {
  pub fn detect(c: char) -> bool {
    c == '+'
  }

  pub fn new(
    iter:   &mut Peekable<CharIndices<'a>>,
    expr:   Box<dyn Token<'a> + 'a>
  ) -> Self {
    let (_, c) = iter.next().unwrap();
    match c {
      '+' => Self::AtLeastOne(expr),
      _   => { unreachable!() }
    }
  }
}

impl<'a> Token<'a> for Quantifier<'a> {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b>,
  ) -> Result<(), ParseError> {
    match self {
      Self::AtLeastOne(token) => {
        let mut ok = false;
        let err = loop {
          if let Err(err) = token.parse(patterns, module, iter, expr) {
            break Err(err);
          }
          ok = true;
          if let None = iter.peek() {
            break Ok(())
          }
        };
        if !ok { err } else { Ok(()) }
      }
    }
  }
}
