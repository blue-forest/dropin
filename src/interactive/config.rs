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

use serde_derive::{Deserialize, Serialize};

use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

pub struct Config {
	path: PathBuf,
	content: Content,
}

#[derive(Deserialize, Serialize)]
struct Content {
	owner: Option<String>,
	model: Option<String>,
}

impl Config {
	pub fn new(root: &Path) -> Self {
		let mut path = root.to_path_buf();
		path.push("config.toml");
		let content = if !path.exists() {
			Content {
				owner: None,
				model: None,
			}
		} else {
			let file_content = read_to_string(&path).unwrap();
			toml::from_str(&file_content).unwrap()
		};
		Self { path, content }
	}

	pub fn set_owner(&mut self, owner: String) {
		self.content.owner = Some(owner);
		self.save();
	}

	pub fn owner(&self) -> &Option<String> {
		&self.content.owner
	}

	pub fn set_model(&mut self, model: String) {
		self.content.model = Some(model);
		self.save();
	}

	pub fn model(&self) -> &Option<String> {
		&self.content.model
	}

	fn save(&self) {
		write(&self.path, toml::to_string(&self.content).unwrap()).unwrap();
	}
}
