/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use std::fs::read_to_string;
use std::path::PathBuf;

use pest::iterators::Pair;

use super::{read_file, RecipeHeader, Rule, print_pairs};

pub fn read_type(path: PathBuf) {
  let content = read_to_string(path).unwrap();
  let pairs = read_file(content.as_str()).into_inner();
  let (header, content_pair_opt) = RecipeHeader::new(pairs);
  let mut content_pairs = content_pair_opt
      .expect("expected type content")
      .into_inner();
  let template_pair = content_pairs.next().expect("expected type templates");
  read_template(template_pair);
  println!("{}", header.id);
  print_pairs(content_pairs, 0);
}

fn read_template(pair: Pair<Rule>) {
  if !matches!(pair.as_rule(), Rule::type_templates) {
    panic!("expected type templates, got {:?}", pair.as_rule());
  }
  for template in pair.into_inner() {
    let mut key_value = template
      .into_inner()
      .next()
      .expect("expected key-value")
      .into_inner();
    let template_key = key_value.next().expect("expected key").as_str();
    let template_format = key_value.next().expect("expected format").as_str();
  }
}
