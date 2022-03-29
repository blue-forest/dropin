/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation under version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use home::home_dir;
use path_clean::PathClean;

use std::env::{var, current_dir};
use std::path::{Path, PathBuf};

use super::ConfigError;

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

fn validate_path(path: &PathBuf) -> Result<(), ConfigError> {
  if !path.exists() {
    match path.parent() {
      Some(parent) => { check_permissions(parent, true) }
      None => {
        Err(ConfigError::from("Can't find the parent directory"))
      }
    }
  } else {
    check_permissions(path, true)
  }
}

fn check_permissions(path: &Path, is_dir: bool) -> Result<(), ConfigError> {
  match path.metadata() {
    Ok(metadata) => {
      if metadata.is_dir() != is_dir {
        return Err(ConfigError::new(format!(
          "\"{}\" is a {}",
          path.to_str().unwrap_or("[non-utf8]"),
          if is_dir { "file" } else { "directory" },
        )));
      }
      if metadata.permissions().readonly() {
        return Err(ConfigError::new(format!(
          "You cannot write into \"{}\"",
          path.to_str().unwrap_or("[non-utf8]"),
        )));
      }
      Ok(())
    }
    Err(err)     => {
      Err(ConfigError::new(format!("{}", err)))
    }
  }
}
