/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#[cfg(debug_assertions)]
use core::fmt::Write;

use alloc::string::String;
use dropin_compiler_common::ir::{Component, Zone};

use crate::Table;

use self::yaml::YamlReader;

mod snippet;
mod yaml;

pub fn parse(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  input: String,
  main_non_terminal: Option<String>,
  table: &Table,
) -> Component {
  let mut yaml = YamlReader::new(&input);
  Component(parse_zone(&mut yaml))
}

pub fn parse_zone(yaml: &mut YamlReader) -> Zone {
  let mut classes_static = vec![];
  let mut classes_dynamic = vec![];
  while let Some(key) = yaml.next_key() {
    match key {
      "classes" => {
        let mut list = yaml.next_list();
        while let Some(class) = list.next_text() {
          classes_static.push(class);
        }
      }
      _ => panic!(r#"unknown zone key "{key}""#),
    }
  }
  Zone {
    classes_static,
    classes_dynamic,
  }
}
