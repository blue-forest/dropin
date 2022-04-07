use structopt::StructOpt;

use std::collections::HashMap;
use std::fmt::Debug;
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

fn get_key<'a>(syntax: &'a str, iter: &mut CharIndices<'a>) -> Option<&'a str> {
  let mut pattern_start: Option<usize> = None;
  let mut result: Option<&str> = None;
  while let Some((i, c)) = iter.next() {
    if !c.is_whitespace() {
      pattern_start = Some(i);
      break;
    }
  }
  if pattern_start.is_none() {
    panic!("no pattern found");
  }
  while let Some((i, c)) = iter.next() {
    if c.is_whitespace() {
      result = Some(syntax.get(pattern_start.unwrap()..i).unwrap());
      break;
    }
  }
  result
}

fn parse<'a>(syntax: &'a str) -> HashMap<&'a str, Box<dyn Expression + 'a>> {
  let mut result = HashMap::new();
  let mut iter = syntax.char_indices();
  let key = get_key(&syntax, &mut iter);
  if key.is_none() {
    panic!("no pattern found");
  }
  result.insert(key.unwrap(), Concat::parse(syntax, &mut iter));
  result
}

fn main() {
  let cli = Cli::from_args();
  let syntax = get_recipe(cli.id);
  let parsed = parse(&syntax);
  println!("PARSED: {:?}", parsed);
}

