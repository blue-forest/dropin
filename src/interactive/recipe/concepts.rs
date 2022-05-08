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

use std::fmt::{Display, Error, Formatter};
use std::path::Path;

use super::{Command, Recipe};

pub struct Modules;

impl Display for Modules {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "modules".fmt(f)
  }
}

impl Recipe for Modules {
  fn title(&self) -> String { "Modules".to_string() }
  fn dir_name(&self) -> String { "modules".to_string() }
  fn commands(&self, _path: &Path) -> Vec<Box<dyn Command>> { vec![] }
}

pub struct Syntaxes;

impl Display for Syntaxes {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "syntaxes".fmt(f)
  }
}

impl Recipe for Syntaxes {
  fn title(&self) -> String { "Syntaxes".to_string() }
  fn dir_name(&self) -> String { "syntaxes".to_string() }
}

