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

use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use crate::interactive::{Command, Cli};
use super::Recipe;
use super::compile::Compile;
use super::edit::Edit;
use super::remove::Remove;
use super::run::Run;

pub struct Select(Arc<Selection>);

impl Select {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self(Arc::new(Selection::new(recipe, id, namespaces)))
  }
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    format!("recipe {}", self.0.id).fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(Run::new(self.0.clone())),
      Box::new(Compile::new(self.0.clone())),
      Box::new(Edit::new(self.0.clone())),
      Box::new(Remove::new(self.0.clone())),
    ];
    let mut id = String::new();
    for namespace in self.0.namespaces.iter() {
      id.push_str(namespace);
      id.push('/');
    }
    id.push_str(&self.0.id);
    cli.run_select(&id, &commands)
  }
}

pub struct Selection {
  namespaces: Arc<Vec<String>>,
  id:         String,
  recipe:     Arc<dyn Recipe>,
}

impl Selection {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self{ recipe, id: id.to_string(), namespaces }
  }

  pub fn namespaces(&self) -> Arc<Vec<String>> { self.namespaces.clone() }
  pub fn id(&self)         -> &str             { &self.id }
  pub fn recipe(&self)     -> Arc<dyn Recipe>  { self.recipe.clone() }
}
