use pest::Parser;
use pest::iterators::{Pair, Pairs};
use termion::color;

use std::io::{stderr, Write};
use std::process::exit;

#[derive(Parser)]
#[grammar = "parser/recipes.pest"]
struct RecipesParser;

pub fn read_file<'a>(content: &'a str) -> Pair<'a, Rule> {
  let mut pairs = RecipesParser::parse(Rule::main, &content)
    .unwrap_or_else(|e| { 
      stderr().write_all((e.to_string() + "\n").as_bytes()).unwrap();
      exit(1);
    });
  pairs.next().unwrap()
}

pub fn print_pairs(pairs: Pairs<Rule>, depth: usize) {
  for pair in pairs {
    let mut pair_str = pair.as_str().to_owned();
    if !pair_str.is_empty() {
      if let Some(i) = pair_str[1..].find('\n') {
        pair_str = pair_str[..=i].to_owned() + "...";
      }
    }
    println!("{:indent$}{:?}: {}{}{}",
      "", pair.as_rule(),
      color::Fg(color::Green), pair_str, color::Fg(color::Reset),
      indent=depth,
    );
    print_pairs(pair.into_inner(), depth+1);
  }
}
