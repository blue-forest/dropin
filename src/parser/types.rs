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

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::Arc;

use pest::iterators::Pair;

use crate::functions::Function;
use crate::types::{self, Type, Format};

use super::{read_file, read_handlers, RecipeHeader, Rule};

pub fn read_type(path: PathBuf) -> Type {
  let content = read_to_string(path).unwrap();
  let pairs = read_file(content.as_str()).into_inner();
  let (header, content_pair_opt) = RecipeHeader::new(pairs);
  let mut content_pairs = content_pair_opt
      .expect("expected type content")
      .into_inner();
  let template_pair = content_pairs.next().expect("expected type templates");
  let mut type_ = Type::new(header.id);
  read_template(&mut type_, template_pair);
  if let Some(functions_pair) = content_pairs.next() {
    read_functions(&mut type_, functions_pair);
  }
  type_
}

fn read_template(type_: &mut Type, pair: Pair<Rule>) {
  if !matches!(pair.as_rule(), Rule::templates) {
    panic!("expected type templates, got {:?}", pair.as_rule());
  }
  for template in pair.into_inner() {
    let mut key_value = template.into_inner();
    let key = key_value.next().expect("expected key").as_str();
    let format_pair = key_value.next().expect("expected format");
    let format = read_format(format_pair);
    type_.add_template(key.to_string(), format);
  }
}

pub fn read_format(pair: Pair<Rule>) -> Format {
  let mut pairs = pair.into_inner();
  let type_id = pairs.next().expect("expected format type").as_str();
  let type_ = match type_id.find(':') {
    Some(_index) => { todo!() }
    None => {
      match type_id {
        "bytes" => types::BYTES.clone(),
        "byte"  => types::BYTE.clone(),
        _ => { panic!("unknown type: {}", type_id); }
      }
    }
  };
  Format::new(type_)
}

pub fn read_functions(type_: &mut Type, pair: Pair<Rule>) {
  for function in pair.into_inner() {
    let mut pairs = function.into_inner();
    let key = pairs.next().expect("expected function key").as_str();
    let next_pair = pairs.next().expect("expected function handlers");
    if matches!(next_pair.as_rule(), Rule::variables) {
      todo!("function variables");
      // next_pair = pairs.next().expect("expected function handlers");
    }
    let handlers = read_handlers(next_pair.into_inner());
    type_.add_function(key.to_string(), Function::new(
      Format::new(types::BYTES.clone()),
      handlers,
      HashMap::new(),
    ));
  }
}
