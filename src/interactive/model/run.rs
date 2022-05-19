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

use dialoguer::Input;

use std::fmt::{Display, Error, Formatter};
use std::fs::read;
use std::path::PathBuf;

use dropin_helpers::fs::{header, wasm};
use dropin_helpers::header::{Header, HeaderFunction, HeaderType};

use crate::embedder::Param;
use crate::interactive::{Cli, Command};

pub struct Run {
	pub(super) model: String,
}

impl Display for Run {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"run".fmt(f)
	}
}

impl Command for Run {
	fn run(&self, cli: &mut Cli) -> u32 {
		let root = &cli.root;
		let owner = &cli.owners[cli.owner_selected.unwrap()];
		let header_path = header(root, owner, &self.model, "v1");
		let wasm_path = wasm(root, owner, &self.model, "v1");
		let bytes = read(&header_path).unwrap();
		let header = Header::from_bytes(&bytes).unwrap();
		cli.run_select("run", |_| {
			let mut commands: Vec<Box<dyn Command>> = vec![];
			for function in header.functions() {
				commands.push(Box::new(RunFunction {
					function,
					path: wasm_path.clone(),
				}));
			}
			commands
		});
		0
	}
}

struct RunFunction<'a> {
	function: &'a HeaderFunction<'a>,
	path: PathBuf,
}

impl<'a> Display for RunFunction<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		if self.function.name() == "_start" {
			"main".fmt(f)
		} else {
			self.function.name().fmt(f)
		}
	}
}

impl<'a> Command for RunFunction<'a> {
	fn run(&self, cli: &mut Cli) -> u32 {
		let mut params = vec![];
		for param in self.function.params() {
			let input: String = Input::new()
				.with_prompt(format!("{} ({})", param.key(), param.type_()))
				.interact_text()
				.unwrap();
			params.push(match param.type_() {
				HeaderType::Bytes => Param::Bytes(input.into_bytes()),
			});
		}
		cli
			.embedder
			.run(Some(&cli.root), &self.path, self.function.name(), params);
		1
	}
}
