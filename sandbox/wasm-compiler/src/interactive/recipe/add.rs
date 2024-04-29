/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
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

use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use edit::edit_file;

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir_all;

use crate::interactive::{Cli, Command};

pub struct Add;

impl Add {
	pub fn new() -> Self {
		Self {}
	}
}

impl Display for Add {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"add".fmt(f)
	}
}

impl Command for Add {
	fn run(&self, cli: &mut Cli) -> u32 {
		let id: String = Input::with_theme(&ColorfulTheme::default())
			.with_prompt("{} ID ? (split namespaces with '/', leave empty to cancel)")
			.allow_empty(true)
			.interact_text()
			.unwrap();
		if id.is_empty() {
			return 0;
		}

		let mut id_split: Vec<&str> = id.split('/').collect();
		let n_splits = id_split.len();
		let id = id_split.split_off(id_split.len() - 1)[0];
		for ns in id_split {
			cli.cwd.push(ns);
		}
		if !cli.cwd.exists() {
			create_dir_all(&cli.cwd).unwrap();
		}
		cli.cwd.push(&format!("{}.dropin", id));
		edit_file(&cli.cwd).unwrap();
		cli.cwd = cli.cwd.ancestors().nth(n_splits).unwrap().to_path_buf();
		0
	}
}
