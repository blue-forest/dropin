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

use home::home_dir;
use path_clean::PathClean;

use std::env::{var, current_dir};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn get_root() -> PathBuf {
  if let Ok(root) = var("DROPIN_ROOT") {
    println!("Using $DROPIN_ROOT ({})", root);
    return PathBuf::from(root);
  }
  let mut path = match home_dir() {
    Some(path) => { path.join(".dropin.recipes") }
    None       => { current_dir().unwrap().join(".dropin.recipes") }
  };
  if path.is_relative() {
    path = current_dir().unwrap().join(path).clean();
  } else {
    path = path.clean();
  }
  path
}

pub fn get_build(root: &Path, owner: &str, model: &str) -> PathBuf {
  let mut result = root.to_path_buf();
  result.push(".builds");
  result.push(owner);
  if !result.exists() {
    create_dir_all(&result).unwrap();
  }
  result.push(&format!("{}_v1.wasm", model));
  result
}

