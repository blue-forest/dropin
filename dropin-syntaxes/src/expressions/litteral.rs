use std::str::CharIndices;

use super::Expression;

#[derive(Debug)]
pub struct Litteral<'a> {
  #[allow(dead_code)]
  value: &'a str,
}

impl<'a> Litteral<'a> {
  pub fn parse(
    syntax: &'a str,
    iter: &mut CharIndices<'a>,
  ) -> Box<dyn Expression + 'a> {
    let mut start: Option<usize> = None;
    let mut value: Option<&str> = None;
    let mut is_escaped = false;
    while let Some((i, c)) = iter.next() {
      if start.is_none() {
        start = Some(i);
      }
      if !is_escaped {
        match c {
          '"' => {
            value = Some(syntax.get(start.unwrap()..i).unwrap());
            break;
          }
          '\\' => {
            is_escaped = true;
            continue;
          }
          _ => {}
        }
      }
      is_escaped = false;
    }
    let value = value.expect("expected '\"'");
    Box::new(Litteral{ value })
  }
}

impl<'a> Expression for Litteral<'a> {}

