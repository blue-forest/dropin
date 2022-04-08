use std::iter::Peekable;
use std::str::CharIndices;

use crate::syntaxes::{Expression, Patterns, ParseError};

use super::{Quantifier, Token};

#[derive(Debug)]
pub struct Getter<'a> {
  query: &'a str,
}

impl<'a> Getter<'a> {
  pub fn parse(
    syntax: &'a str,
    iter:   &mut Peekable<CharIndices<'a>>,
  ) -> Box<dyn Token<'a> + 'a> {
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

impl<'a> Token<'a> for Getter<'a> {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b, 'c>,
  ) -> Result<(), ParseError> {
    if self.query.starts_with("patterns.") {
      let key = self.query.get(9..).unwrap();
      let pattern = patterns.get(key).unwrap();
      expr.add_inner(pattern.parse(patterns, module, iter)?);
      Ok(())
    } else if self.query.starts_with("std.") {
      if let Some((_, c)) = iter.peek() {
        if c.is_alphanumeric() {
          iter.next();
          Ok(())
        } else {
          Err(ParseError::new(
            format!("unexpected token {}, expected alphanum", c),
          ))
        }
      } else {
        Err(ParseError::new(
          "unexpected end of file, expected alphanum".to_string(),
        ))
      }
    } else {
      return Err(ParseError::new(format!("unknown ref: {}", self.query)));
    }
  }
}

