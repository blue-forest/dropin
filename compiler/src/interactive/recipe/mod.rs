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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt::{Display, Error, Formatter};
use std::fs::read_dir;
use std::path::Path;
use std::sync::Arc;

use super::{Cli, Command};

mod add;
use add::Add;
mod concepts;
pub use concepts::*;
mod edit;
mod namespace;
use namespace::Namespace;
mod remove;
mod select;
use select::Select;

pub struct RecipeCommand {
	recipe: Arc<dyn Recipe>,
}

impl RecipeCommand {
	pub fn new(recipe: Arc<dyn Recipe>) -> Self {
		Self { recipe }
	}
}

impl Display for RecipeCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		self.recipe.fmt(f)
	}
}

impl Command for RecipeCommand {
	fn run(&self, cli: &mut Cli) -> u32 {
		cli.cwd.push(self.recipe.dir_name());
		let result = cli.run_select(&self.recipe.title(), |cli| {
			get_entries(&cli.cwd, self.recipe.clone(), Arc::new(Vec::new()))
		});
		cli.cwd.pop();
		result
	}

	fn is_enabled(&self, cli: &Cli) -> bool {
		cli.model_selected.is_some()
	}
}

fn get_entries(
	path: &Path,
	recipe: Arc<dyn Recipe>,
	namespaces: Arc<Vec<String>>,
) -> Vec<Box<dyn Command>> {
	let mut commands: Vec<Box<dyn Command>> = Vec::new();
	if let Ok(dir) = read_dir(path) {
		for entry in dir.flatten() {
			if entry.path().is_dir() {
				commands.push(Box::new(Namespace::new(
					recipe.clone(),
					entry.path().file_name().unwrap().to_str().unwrap(),
					namespaces.clone(),
				)));
			} else {
				commands.push(Box::new(Select::new(
					recipe.clone(),
					entry.path().file_stem().unwrap().to_str().unwrap(),
					namespaces.clone(),
				)));
			}
		}
	}
	commands.push(Box::new(Add::new()));
	commands
}

pub trait Recipe: Display {
	fn title(&self) -> String;
	fn dir_name(&self) -> String;
	fn commands(&self, _path: &Path) -> Vec<Box<dyn Command>> {
		vec![]
	}
}
