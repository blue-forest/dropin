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
use std::sync::Arc;

use crate::interactive::{Cli, Command};
use crate::interactive::path::get_version;
use super::{get_entries, Recipe};

pub struct Namespace {
  recipe:     Arc<dyn Recipe>,
  id:         String,
  namespaces: Arc<Vec<String>>
}

impl Namespace {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self{ recipe, id: id.to_string(), namespaces }
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    format!("namespace {}", self.id).fmt(f)
  }
}

impl Command for Namespace {
  fn run(&self, cli: &mut Cli) -> bool {
    let namespaces = {
      let mut namespaces = (*self.namespaces).clone();
      namespaces.push(self.id.clone());
      Arc::new(namespaces)
    };
    let mut path = get_version(cli).unwrap();
    path.push(self.recipe.dir_name());
    for ns in namespaces.iter() {
      path.push(&ns);
    }
    let commands = get_entries(
      &path, self.recipe.clone(), namespaces.clone(),
    );
    cli.run_select(
      &format!("{} Namespace {}", self.recipe, namespaces.join("/")),
      &commands,
    );
    false
  }
}
