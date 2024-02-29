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

use edit::edit_file;

use std::fmt::{Display, Error, Formatter};

use crate::interactive::{Cli, Command};

pub struct Edit;

impl Display for Edit {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"edit".fmt(f)
	}
}

impl Command for Edit {
	fn run(&self, cli: &mut Cli) -> u32 {
		edit_file(&cli.cwd.parent().unwrap().join(".dropin")).unwrap();
		0
	}
}
