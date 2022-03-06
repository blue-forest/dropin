/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

mod types;
pub use types::read_type;

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

struct RecipeHeader {
  id:   String,
  name: Option<String>,
}

impl RecipeHeader {
  fn new(mut pairs: Pairs<Rule>) -> (RecipeHeader, Option<Pair<Rule>>) {
    let no_header_fn = || { 
      stderr().write_all("no header found in recipe\n".as_bytes()).unwrap();
      exit(1);
    };
    let header_pair = pairs.next().unwrap_or_else(no_header_fn);
    match header_pair.as_rule() {
      Rule::header => {}
      _ => { no_header_fn(); }
    }
    let mut header_pairs = header_pair.into_inner();
    let id = header_pairs.next().unwrap_or_else(|| { 
      stderr().write_all("no id found in recipe\n".as_bytes()).unwrap();
      exit(1);
    }).as_str().to_string();
    let mut header = RecipeHeader{
      id,
      name: None,
    };
    if let Some(name) = header_pairs.next() {
      header.name = Some(name.as_str().to_string());
      // recipe.set(Query::new("name"), text(name.as_str())).unwrap();
    }
    // TODO: description, terms, icon...
    (header, pairs.next())
  }
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
