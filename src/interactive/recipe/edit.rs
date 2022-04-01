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

use crate::interactive::{Cli, Command};
use crate::interactive::path::get_version;
use super::select::Selection;

pub struct Edit(Arc<Selection>);

impl Edit {
  pub fn new(selection: Arc<Selection>) -> Self {
    Self(selection)
  }
}

impl Display for Edit {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "edit".fmt(f)
  }
}

impl Command for Edit {
  fn run(&self, cli: &mut Cli) -> u32 {
    let mut path = get_version(cli).unwrap();
    path.push(self.0.recipe().dir_name());
    for ns in self.0.namespaces().iter() {
      path.push(&ns);
    }
    path.push(format!("{}.dropin", self.0.id()));
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
    0
  }
}
