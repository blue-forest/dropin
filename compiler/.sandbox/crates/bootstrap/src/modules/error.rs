/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2023 Blue Forest
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

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CompileError(String);

impl CompileError {
	pub fn new(message: String) -> Self {
		Self(message)
	}
}

impl From<&str> for CompileError {
	fn from(message: &str) -> Self {
		Self::new(message.to_string())
	}
}

impl Display for CompileError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		Display::fmt(&self.0, f)
	}
}

impl Error for CompileError {}
