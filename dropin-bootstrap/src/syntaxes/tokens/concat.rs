use std::iter::Peekable;
use std::str::CharIndices;

use crate::syntaxes::{Expression, Patterns, ParseError};
use super::{Getter, Litteral, Token};

#[derive(Debug)]
pub struct Concat<'a> {
  tokens: Vec<Box<dyn Token<'a> + 'a>>,
}

impl<'a> Concat<'a> {
  pub fn parse(
    syntax: &'a str,
    iter: &mut Peekable<CharIndices<'a>>,
  ) -> Box<dyn Token<'a> + 'a> {
    let mut tokens = Vec::new();
    while let Some((_, c)) = iter.next() {
      if !c.is_whitespace() {
        tokens.push(
          match c {
            '"' => Litteral::parse(syntax, iter),
            '$' => Getter::parse(syntax, iter),
            _   => { panic!("unknown token {}", c); }
          }
        );
        if let Some((_, peeked)) = iter.peek() {
          if !peeked.is_whitespace() {
            panic!("unexpected '{}'", c);
          }
        }
      } else if c == '\n' {
        if let Some((_, peeked)) = iter.peek() {
          if !peeked.is_whitespace() || *peeked == '\n' {
            break;
          }
        }
      }
    }
    Box::new(Concat{ tokens })
  }
}

impl<'a> Token<'a> for Concat<'a> {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b, 'c>,
  ) -> Result<(), ParseError> {
    for token in self.tokens.iter() {
      token.parse(patterns, module, iter, expr)?;
    }
    Ok(())
  }
}
