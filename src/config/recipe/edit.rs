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

use dialoguer::Editor;

use std::fmt::{Display, Error, Formatter};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::sync::Arc;

use crate::config::{Cli, Command};
use crate::config::path::get_version;
use super::Recipe;

pub struct Edit {
  recipe:     Arc<dyn Recipe>,
  id:         String,
  namespaces: Arc<Vec<String>>,
}

impl Edit {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self{ recipe, id: id.to_string(), namespaces }
  }
}

impl Display for Edit {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    format!("edit {}", self.id).fmt(f)
  }
}

impl Command for Edit {
  fn run(&self, cli: &mut Cli) -> bool {
    let mut path = get_version(cli).unwrap();
    path.push(self.recipe.dir_name());
    for ns in self.namespaces.iter() {
      path.push(&ns);
    }
    path.push(format!("{}.dropin", self.id));
    let content = read_to_string(&path).unwrap();
    let editor = Editor::new()
      .edit(&content);
    if let Some(updated_content) = editor.unwrap() {
      let mut file = File::create(&path).unwrap();
      file.write_all(updated_content.as_bytes()).unwrap();
      println!("Recipe updated at {}", path.to_str().unwrap());
    } else {
      println!("Edition Canceled");
    }
    false
  }
}
