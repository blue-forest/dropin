use structopt::StructOpt;

use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::CharIndices;

mod expressions;
use expressions::{Concat, Expression};

mod path;
use path::get_recipe;

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in syntaxes")]
struct Cli {
  #[structopt(long)]
  id: String,
}

fn get_key<'a>(
  syntax: &'a str,
  iter: &mut Peekable<CharIndices<'a>>,
) -> Option<&'a str> {
  let mut pattern_start: Option<usize> = None;
  let mut result: Option<&str> = None;
  while let Some((i, c)) = iter.next() {
    if !c.is_whitespace() {
      pattern_start = Some(i);
      break;
    }
  }
  if pattern_start.is_none() {
    return None;
  }
  while let Some((i, c)) = iter.next() {
    if c.is_whitespace() {
      result = Some(syntax.get(pattern_start.unwrap()..i).unwrap());
      break;
    }
  }
  result
}

fn parse<'a>(syntax: &'a str) -> (
  HashMap<&'a str, Box<dyn Expression + 'a>>,
  &'a str
) {
  let mut result = HashMap::new();
  let mut iter = syntax.char_indices().peekable();
  let mut entry_key: Option<&str> = None;
  loop {
    let key = get_key(&syntax, &mut iter);
    if entry_key.is_none() {
      entry_key = key;
    }
    if key.is_none() {
      break
    }
    let c = Concat::parse(syntax, &mut iter);
    result.insert(key.unwrap(), c);
  }
  (result, entry_key.expect("no pattern found"))
}

fn main() {
  let cli = Cli::from_args();
  let syntax = get_recipe(cli.id);
  let parsed = parse(&syntax);
  println!("PARSED: {:?}", parsed);
}

