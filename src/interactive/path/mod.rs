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
use std::path::PathBuf;

use super::ConfigError;

mod validation;
pub use validation::*;

mod resolve;
pub use resolve::*;

pub fn get_root() -> PathBuf {
  if let Ok(root) = var("DROPIN_ROOT") {
    println!("Using $DROPIN_ROOT ({})", root);
    return PathBuf::from(root);
  }
  let mut path = match home_dir() {
    Some(path) => { path.join("dropin") }
    None       => { current_dir().unwrap().join("dropin") }
  };
  if path.is_relative() {
    path = current_dir().unwrap().join(path).clean();
  } else {
    path = path.clean();
  }
  validate_path(&path).unwrap();
  path
}
