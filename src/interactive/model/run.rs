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

use parity_wasm::elements::Internal;
use parity_wasm::deserialize_file;

use std::fmt::{Display, Error, Formatter};
use std::path::PathBuf;

use dropin_helpers::fs::header;
use dropin_helpers::Header;

use crate::interactive::{Cli, Command};

pub struct Run;

impl Display for Run {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"run".fmt(f)
	}
}

impl Command for Run {
	fn run(&self, cli: &mut Cli) -> u32 {
		cli.run_select("run", |cli| {
			let root = &cli.root;
			let owner = &cli.owners[cli.owner_selected.unwrap()];
			let model = &cli.models[cli.model_selected.unwrap()];
			let path = header(root, owner, model, "v1");
			let header = Header::from_file(&path);
			println!("{:?}", header);
			std::process::exit(1);
			let module = deserialize_file(&path).unwrap();
			let mut commands: Vec<Box<dyn Command>> = vec![];
			if let Some(export) = module.export_section() {
				for entry in export.entries() {
					if let Internal::Function(id) = entry.internal() {
						commands.push(Box::new(RunFunction{
							name: entry.field().to_string(), path: path.clone(), id: *id,
						}));
					}
				}
			}
			commands
		});
		0
	}
}

struct RunFunction {
	name: String,
	path: PathBuf,
	id: u32,
}

impl Display for RunFunction {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		if self.name == "_start" {
			"main".fmt(f)
		} else {
			self.name.fmt(f)
		}
	}
}

impl Command for RunFunction {
	fn run(&self, cli: &mut Cli) -> u32 {
		let module = deserialize_file(&self.path).unwrap();
		cli.embedder.run(Some(&cli.root), &self.path, &self.name);
		0
	}
}
