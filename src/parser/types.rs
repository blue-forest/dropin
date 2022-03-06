/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use std::fs::read_to_string;
use std::path::PathBuf;

use super::{read_file, RecipeHeader, print_pairs};

pub fn read_type(path: PathBuf) {
  let content = read_to_string(path).unwrap();
  let pairs = read_file(content.as_str()).into_inner();
  let (header, content_pair) = RecipeHeader::new(pairs);
  println!("{}", header.id);
  print_pairs(content_pair.unwrap().into_inner(), 0);
}
