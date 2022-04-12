/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
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

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use edit::edit_file;

use std::fs::{create_dir_all, File};
use std::fmt::{Display, Error, Formatter};
use std::io::Write;
use std::sync::Arc;

use crate::interactive::{Cli, Command};
use crate::interactive::path::get_namespace;
use super::{Recipe, RecipeCommand};
use super::namespace::Namespace;

pub struct Add {
  namespaces: Arc<Vec<String>>,
  recipe:     Arc<dyn Recipe>,
}

impl Add {
  pub fn new(recipe: Arc<dyn Recipe>, namespaces: Arc<Vec<String>>) -> Self {
    Self{ namespaces, recipe }
  }
}

impl Display for Add {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "add".fmt(f)
  }
}

impl Command for Add {
  fn run(&self, cli: &mut Cli) -> u32 {
    let id: String = Input::with_theme(&ColorfulTheme::default())
      .with_prompt("{} ID ? (split namespaces with '/', leave empty to cancel)")
      .allow_empty(true)
      .interact_text().unwrap();
    if id.is_empty() { return 0; }
    let recipe_name = self.recipe.dir_name();

    let mut id_split: Vec<&str> = id.split('/').collect();
    let id = id_split.split_off(id_split.len()-1)[0];
    let namespaces = [
      (*self.namespaces).iter().map(|s| s.as_str()).collect(),
      id_split,
    ].concat();
    let mut path = get_namespace(cli, &recipe_name, namespaces);
    if !path.exists() {
      create_dir_all(&path).unwrap();
    }
    path.push(&format!("{}.dropin", id));
    let mut file = File::create(&path).unwrap();
    file.write_all(
      format!(
        "{} {}\n{:=>width$}\n",
        recipe_name, id, 
        "", width=recipe_name.len() + id.len() + 1,
      ).as_bytes(),
    ).unwrap();
    edit_file(path).unwrap();
    if self.namespaces.is_empty() {
      RecipeCommand::new(self.recipe.clone()).run(cli);
    } else {
      let id = &self.namespaces[self.namespaces.len()-1];
      let mut namespaces = (*self.namespaces).clone();
      namespaces.pop();
      Namespace::new(
        self.recipe.clone(),
        id,
        Arc::new(namespaces),
      ).run(cli);
    };
    1
  }
}
