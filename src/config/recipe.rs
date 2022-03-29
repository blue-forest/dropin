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

use super::{Cli, Command};

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
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(Add{ recipe: self.recipe.clone() })
    ];
    cli.run_select(&format!("{}", self.recipe), &commands);
    false
  }

  fn is_enabled(&self, cli: &Cli) -> bool { cli.model_selected.is_some() }
}

struct Add {
  recipe: Arc<dyn Recipe>,
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
    let mut path = cli.root.clone();
    let owner = &cli.owners[cli.owner_selected.unwrap()];
    path.push(owner);
    let model = &cli.models[cli.model_selected.unwrap()];
    path.push(model);
    path.push(&cli.version);
    let recipe_name = self.recipe.dir_name();
    path.push(&recipe_name);
    let namespaces: Vec<&str> = id.split('/').collect();
    for namespace in namespaces.get(..namespaces.len()-1).unwrap() {
      path.push(namespace);
    }
    if !path.exists() {
      create_dir_all(&path).unwrap();
    }
    path.push(&format!("{}.dropin", namespaces[namespaces.len()-1]));
    if let Some(recipe_content) = Editor::new()
      .edit(&format!(
        "{} {}:{}:{}:{}\n{:=>width$}\n",
        recipe_name, owner, model, cli.version, id,
        "", width=recipe_name.len()
          + owner.len()
          + model.len()
          + cli.version.len()
          + id.len()
          + 4,
      ))
      .unwrap() {
      let mut file = File::create(&path).unwrap();
      file.write_all(recipe_content.as_bytes()).unwrap();
      println!("Recipe updated at {}", path.to_str().unwrap());
    } else {
      println!("Edition Canceled");
    }
    false
  }
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
