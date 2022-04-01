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
use std::fs::remove_file;
use std::sync::Arc;

use crate::interactive::{Cli, Command};
use crate::interactive::path::get_recipe;
use super::RecipeCommand;
use super::namespace::Namespace;
use super::select::Selection;

pub struct Remove(Arc<Selection>);

impl Remove {
  pub fn new(selection: Arc<Selection>) -> Self {
    Self(selection)
  }
}

impl Display for Remove {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "remove".fmt(f)
  }
}

impl Command for Remove {
  fn run(&self, cli: &mut Cli) -> bool {
    let self_namespaces = self.0.namespaces();
    let namespaces = self_namespaces.iter().map(|s| s.as_str()).collect();
    let path = get_recipe(
      cli, &self.0.recipe().dir_name(), namespaces, self.0.id(),
    );
    remove_file(path).unwrap();
    if self_namespaces.is_empty() {
      RecipeCommand::new(self.0.recipe()).run(cli);
    } else {
      let id = self_namespaces[self_namespaces.len()-1].clone();
      let mut namespaces = (*self_namespaces).clone();
      namespaces.pop();
      Namespace::new(self.0.recipe(), &id, Arc::new(namespaces)).run(cli);
    };
    true
  }
}
