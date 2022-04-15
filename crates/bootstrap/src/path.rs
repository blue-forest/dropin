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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs::read_to_string;
use std::path::PathBuf;
use dropin_utils::path::get_root;

fn get_path(collection: &str, id: &str) -> PathBuf {
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
  result.push(collection);
  result.push(recipe);
  result
}

pub fn get_recipe(collection: &str, id: &str) -> String {
  let path = get_path(collection, id);
  let content = read_to_string(path).unwrap();
  let header_split = content.find("\n===").unwrap();
  let start = content.get(header_split+4..).unwrap().find("\n").unwrap() + header_split + 5;
  content.get(start..).unwrap().to_string()
}

