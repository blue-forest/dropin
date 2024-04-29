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

use std::fmt::{Display, Error, Formatter};

use super::path::get_owner;
use super::{get_dirs, Cli, Command};
use dropin_utils::path::get_build;

mod add;
use add::Add;
mod compile;
use compile::Compile;
mod edit;
use self::edit::Edit;
mod run;
use run::Run;
mod select;
use select::Select;

pub struct Models;

impl Display for Models {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"models".fmt(f)
	}
}

impl Command for Models {
	fn run(&self, cli: &mut Cli) -> u32 {
		let mut path = get_owner(cli).unwrap();
		path.push("models");
		cli.run_select("Models", |cli| {
			cli.models = get_dirs(&path);
			let mut commands: Vec<Box<dyn Command>> = Vec::new();
			for (i, model) in cli.models.iter().enumerate() {
				commands.push(Box::new(Model {
					name: model.to_string(),
					index: i,
				}));
			}
			commands.push(Box::new(Add {}));
			commands
		})
	}

	fn is_enabled(&self, cli: &Cli) -> bool {
		cli.owner_selected.is_some()
	}
}

struct Model {
	name: String,
	index: usize,
}

impl Display for Model {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		self.name.fmt(f)
	}
}

impl Command for Model {
	fn run(&self, cli: &mut Cli) -> u32 {
		cli.run_select(&format!("Model {}", self.name), |cli| {
			let mut result: Vec<Box<dyn Command>> = vec![
				Box::new(Select {
					name: self.name.clone(),
					index: self.index,
				}),
				Box::new(Edit {}),
				Box::new(Compile {}),
			];
			let owner = &cli.owners[cli.owner_selected.unwrap()];
			let model = &cli.models[cli.model_selected.unwrap()];
			let build_path = get_build(&cli.root, owner, model);
			if build_path.exists() {
				result.push(Box::new(Run {}));
			}
			result
		})
	}
}
