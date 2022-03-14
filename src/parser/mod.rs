/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use termion::color;

mod types;
pub use types::read_type;

mod values;
pub use values::read_values;

#[derive(Parser)]
#[grammar = "parser/recipes.pest"]
struct RecipesParser;

pub fn read_file<'a>(content: &'a str) -> Pair<'a, Rule> {
  let mut pairs = RecipesParser::parse(Rule::main, &content)
    .unwrap_or_else(|e| { 
      panic!("{}", e);
    });
  pairs.next().unwrap()
}

struct RecipeHeader {
  id:   String,
  name: Option<String>,
}

impl RecipeHeader {
  fn new(mut pairs: Pairs<Rule>) -> (RecipeHeader, Option<Pair<Rule>>) {
    let no_header = "no header found in recipe";
    let header_pair = pairs.next().expect(no_header);
    if !matches!(header_pair.as_rule(), Rule::header) {
      panic!("{}", no_header);
    }
    let mut header_pairs = header_pair.into_inner();
    let id = header_pairs
      .next()
      .expect("no id found in recipe")
      .as_str()
      .to_string();
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
