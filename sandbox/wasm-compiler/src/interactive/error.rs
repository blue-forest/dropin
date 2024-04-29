/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct ConfigError(String);

impl ConfigError {
	pub fn new(message: String) -> Self {
		Self(message)
	}
}

impl From<&str> for ConfigError {
	fn from(message: &str) -> Self {
		Self(message.to_string())
	}
}

impl Display for ConfigError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}

impl Error for ConfigError {}
