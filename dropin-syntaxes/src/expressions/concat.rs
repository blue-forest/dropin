use std::str::CharIndices;

use super::{Expression, Getter, Litteral};

#[derive(Debug)]
pub struct Concat<'a> {
  #[allow(dead_code)]
  tokens: Vec<Box<dyn Expression + 'a>>,
}

impl<'a> Concat<'a> {
  pub fn parse(
    syntax: &'a str,
    iter: &mut CharIndices<'a>,
  ) -> Box<dyn Expression + 'a> {
    let mut tokens = Vec::new();
    while let Some((_, c)) = iter.next() {
      if !c.is_whitespace() {
        tokens.push(
          match c {
            '"' => Litteral::parse(syntax, iter),
            '$' => Getter::parse(syntax, iter),
            _ => { panic!("unknown token {}", c); }
          }
        );
      }
    }
    Box::new(Concat{ tokens })
  }
}

impl<'a> Expression for Concat<'a> {}
