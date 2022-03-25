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
use pest::iterators::{Pair, Pairs};

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::collections::Method;
use crate::types::{self, Format, Methods, Type};

use super::{read_file, read_handlers, RecipeHeader, Rule};

pub fn read_type(path: PathBuf) -> Type {
  let content = read_to_string(path).unwrap();
  let pairs = read_file(content.as_str()).into_inner();
  let (header, content_pair_opt) = RecipeHeader::new(pairs);
  let mut content_pairs = content_pair_opt
      .expect("expected type content")
      .into_inner();
  let templates_pair = content_pairs.next().expect("expected type templates");
  let templates = read_templates(templates_pair);
  let methods_pair = content_pairs.next().expect("expected methods");
  let methods = read_methods(methods_pair);
  Type::new(header.id, templates, methods)
}

fn read_templates(pair: Pair<Rule>) -> HashMap<String, Format> {
  if !matches!(pair.as_rule(), Rule::templates) {
    panic!("expected type templates, got {:?}", pair.as_rule());
  }
  let mut result = HashMap::new();
  for template in pair.into_inner() {
    let (key, format) = read_key_format(template.into_inner());
    result.insert(key, format);
  }
  result
}

pub fn read_key_format(mut pairs: Pairs<Rule>) -> (String, Format) {
  let key = pairs.next().expect("expected key").as_str();
  let format_pair = pairs.next().expect("expected format");
  let format = read_format(format_pair);
  (key.to_string(), format)
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

pub fn read_methods(pair: Pair<Rule>) -> Methods {
  let mut encode: Option<Method> = None;
  for method in pair.into_inner() {
    let mut pairs = method.into_inner();
    let key = pairs.next().expect("expected method key").as_str();
    let mut next_pair = pairs.next().expect("expected method handlers");
    let mut variables = HashMap::new();
    if matches!(next_pair.as_rule(), Rule::variables) {
      for variable in next_pair.into_inner() {
        let (key, format) = read_key_format(variable.into_inner());
        variables.insert(key, format);
      }
      next_pair = pairs.next().expect("expected method handlers");
    }
    let handlers = read_handlers(next_pair.into_inner());
    match key {
      "encode" => { encode = Some(Method::new(variables, handlers)) }
      _ => { panic!("unknown method: {}", key) }
    }
  }
  let encode_body = encode.expect("expected encode method");
  Methods::new(encode_body)
}
