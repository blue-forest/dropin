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
use super::path::get_version;

mod add;
use add::Add;
mod edit;
use edit::Edit;
mod namespace;
use namespace::Namespace;

pub struct RecipeCommand {
  recipe: Arc<dyn Recipe>,
}

impl RecipeCommand {
  pub fn new(recipe: Arc<dyn Recipe>) -> Self {
    Self{ recipe }
  }
}

impl Display for RecipeCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    self.recipe.fmt(f)
  }
}

impl Command for RecipeCommand {
  fn run(&self, cli: &mut Cli) -> bool {
    let mut path = get_version(cli).unwrap();
    path.push(self.recipe.dir_name());
    let commands = get_entries(
      &path,
      self.recipe.clone(),
      Arc::new(Vec::new()),
    );
    cli.run_select(&format!("{}", self.recipe), &commands);
    false
  }

  fn is_enabled(&self, cli: &Cli) -> bool { cli.model_selected.is_some() }
}

fn get_entries(
  path:       &Path,
  recipe:     Arc<dyn Recipe>,
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
        commands.push(Box::new(Edit::new(
          recipe.clone(),
          entry.path().file_stem().unwrap().to_str().unwrap(),
          namespaces.clone(),
        )));
      }
    }
  }
  commands.push(Box::new(Add::new(recipe.clone(), namespaces)));
  commands
}

pub trait Recipe: Display {
  fn dir_name(&self) -> String;
}

pub struct Type;

impl Display for Type {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "Type".fmt(f)
  }
}

impl Recipe for Type {
  fn dir_name(&self) -> String { "types".to_string() }
}
