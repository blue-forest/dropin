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

use regex::Regex;

use std::path::Path;

use super::ConfigError;

pub fn validate_name(root: &Path, name: &str) -> Result<(), ConfigError> {
  let re = Regex::new(
    r"^(\w|[.\-_àâæçéèêëïîôœùûüÿÀÂÆÇÉÈÊËÏÎÔŒÙÛÜŸ])+$"
  ).unwrap();
  if !re.is_match(name) {
    return Err(ConfigError::from(
      "Name may only be composed of alphanumerics, '.', '-' and '_'",
    ));
  }
  let name_root = root.join(name);
  if name_root.exists() {
    return Err(ConfigError::new(format!(
      "{} directory already exists",
      name_root.to_str().unwrap(),
    )))
  }
  Ok(())
}

pub fn get_dirs(path: &Path) -> Vec<String> {
  let mut result = Vec::new();
  for owner_dir in path.read_dir().unwrap().flatten() {
    let path = owner_dir.path();
    if path.is_dir() {
      result.push(
        path.file_name().unwrap().to_str().unwrap().to_string(),
      );
    }
  }
  result
}
