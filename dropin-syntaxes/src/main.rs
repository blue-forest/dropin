use structopt::StructOpt;

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::CharIndices;

use dropin_utils::path::get_root;

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in syntaxes")]
struct Cli {
  #[structopt(long)]
  id: String,
}

fn get_path(id: String) -> PathBuf {
  let mut split = id.split(':');
  let owner = split.next().expect("expected owner");
  let model = split.next().expect("expected model");
  let version = split.next().expect("expected version");
  let mut recipe = split.next().expect("expected recipe").to_string();
  recipe.push_str(".dropin");
  let mut result = get_root();
  result.push(owner);
  // result.push("models");
  result.push(model);
  result.push(version);
  result.push("syntaxes");
  result.push(recipe);
  result
}

fn get_recipe(id: String) -> String {
  let path = get_path(id);
  let content = read_to_string(path).unwrap();
  let header_split = content.find("\n===").unwrap();
  let start = content.get(header_split+4..).unwrap().find("\n").unwrap() + header_split + 5;
  content.get(start..).unwrap().to_string()
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
      // pattern_start = None;
      break;
    }
  }
  result
}

fn parse<'a>(
  syntax: &'a str,
) -> HashMap<&'a str, Box<dyn Expression + 'a>> {
  let mut result = HashMap::new();
  let mut iter = syntax.char_indices();
  let key = get_key(&syntax, &mut iter);
  if key.is_none() {
    panic!("no pattern found");
  }
  result.insert(key.unwrap(), Concat::parse(syntax, &mut iter));
  result
}

trait Expression: Debug {}

#[derive(Debug)]
struct Concat<'a> {
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
            _ => { panic!("unknown token {}", c); }
          }
        );
      }
    }
    Box::new(Concat{ tokens })
  }
}

impl<'a> Expression for Concat<'a> {}

#[derive(Debug)]
struct Litteral<'a> {
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
    let value = value.expect("expected \"");
    Box::new(Litteral{ value })
  }
}

impl<'a> Expression for Litteral<'a> {}

fn main() {
  let cli = Cli::from_args();
  let syntax = get_recipe(cli.id);
  let parsed = parse(&syntax);
  println!("PARSED: {:?}", parsed);
}
