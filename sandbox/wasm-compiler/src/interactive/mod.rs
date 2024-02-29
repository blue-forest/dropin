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

use std::fmt::Display;
use std::fs::create_dir;
use std::path::PathBuf;

use dropin_utils::path::get_root;

use crate::utils::get_dirs;
use crate::Embedder;

mod cli_run;
mod config;
use self::config::Config;
mod error;
use error::ConfigError;
mod model;
mod owner;
mod path;
mod recipe;
use path::validate_path;

pub struct Cli {
	config: Config,
	cwd: PathBuf,
	embedder: Embedder,
	model_selected: Option<usize>,
	models: Vec<String>,
	owner_selected: Option<usize>,
	owners: Vec<String>,
	root: PathBuf,
	version: String,
}

impl Cli {
	pub fn new() -> Self {
		let root = get_root();
		validate_path(&root).unwrap();
		let owners = if !root.exists() {
			println!("Created drop'in root");
			create_dir(&root).unwrap();
			vec![]
		} else {
			get_dirs(&root)
		};
		let mut cwd = root.clone();
		let config = Config::new(&root);
		let mut owner_selected = None;
		let mut model_selected = None;
		let mut models = vec![];
		if let Some(owner) = config.owner() {
			owner_selected = Some(owners.iter().position(|o| o == owner).unwrap());
			cwd.push(owner);
			cwd.push("models");
			models = get_dirs(&cwd);
			if let Some(model) = config.model() {
				cwd.push(model);
				cwd.push("v1"); // TODO: deal with versions
				model_selected = Some(models.iter().position(|m| m == model).unwrap());
			}
		}
		Self {
			config,
			cwd,
			embedder: Embedder::new(&root),
			model_selected,
			models,
			owner_selected,
			owners,
			root,
			version: "v1".to_string(), // TODO: deal with versions
		}
	}
}

impl Default for Cli {
	fn default() -> Self {
		Self::new()
	}
}

pub trait Command: Display {
	fn run(&self, cli: &mut Cli) -> u32;
	fn is_enabled(&self, _cli: &Cli) -> bool {
		true
	}
}
