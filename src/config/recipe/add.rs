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

use dialoguer::{Editor, Input};
use dialoguer::theme::ColorfulTheme;

use std::fs::{create_dir_all, File};
use std::fmt::{Display, Error, Formatter};
use std::io::Write;
use std::sync::Arc;

use crate::config::{Cli, Command};
use crate::config::path::get_version;
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
  fn run(&self, cli: &mut Cli) -> bool {
    let id: String = Input::with_theme(&ColorfulTheme::default())
      .with_prompt("{} ID ? (split namespaces with '/', leave empty to cancel)")
      .allow_empty(true)
      .interact_text().unwrap();
    if id.is_empty() { return false; }

    let owner       = &cli.owners[cli.owner_selected.unwrap()];
    let model       = &cli.models[cli.model_selected.unwrap()];
    let recipe_name = self.recipe.dir_name();

    let mut full_id = String::new();
    full_id.push_str(&self.namespaces.join("/"));
    if !full_id.is_empty() { full_id.push('/'); }
    full_id.push_str(&id);
    let editor = Editor::new()
      .edit(&format!(
        "{} {}:{}:{}:{}\n{:=>width$}\n",
        recipe_name, owner, model, cli.version, full_id, 
        "", width=recipe_name.len()
          + owner.len()
          + model.len()
          + cli.version.len()
          + full_id.len()
          + 4,
      ));
    if let Some(recipe_content) = editor.unwrap() {
      let mut path = get_version(cli).unwrap();
      path.push(&recipe_name);
      for namespace in self.namespaces.iter() {
        path.push(namespace);
      }
      let namespaces: Vec<&str> = id.split('/').collect();
      for namespace in namespaces.get(..namespaces.len()-1).unwrap() {
        path.push(namespace);
      }

      if !path.exists() {
        create_dir_all(&path).unwrap();
      }
      path.push(&format!("{}.dropin", namespaces[namespaces.len()-1]));
      let mut file = File::create(&path).unwrap();
      file.write_all(recipe_content.as_bytes()).unwrap();
      println!("Recipe updated at {}", path.to_str().unwrap());
      if self.namespaces.is_empty() {
        RecipeCommand::new(self.recipe.clone()).run(cli);
      } else {
        let id = &self.namespaces[0];
        let mut namespaces = (*self.namespaces).clone();
        namespaces.pop();
        Namespace::new(
          self.recipe.clone(),
          id,
          Arc::new(namespaces),
        ).run(cli);
      };
      true
    } else {
      println!("Edition Canceled");
      false
    }
  }
}
