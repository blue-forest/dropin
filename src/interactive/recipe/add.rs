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
use super::Recipe;

pub struct Add {
  recipe:     Arc<dyn Recipe>,
}

impl Add {
  pub fn new(recipe: Arc<dyn Recipe>) -> Self {
    Self{ recipe }
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
    let n_splits = id_split.len();
    let id = id_split.split_off(id_split.len()-1)[0];
    for ns in id_split {
      cli.cwd.push(ns);
    }
    if !cli.cwd.exists() {
      create_dir_all(&cli.cwd).unwrap();
    }
    cli.cwd.push(&format!("{}.dropin", id));
    let mut file = File::create(&cli.cwd).unwrap();
    file.write_all(
      format!(
        "{} {}\n{:=>width$}\n",
        recipe_name, id, 
        "", width=recipe_name.len() + id.len() + 1,
      ).as_bytes(),
    ).unwrap();
    edit_file(&cli.cwd).unwrap();
    cli.cwd = cli.cwd.ancestors().nth(n_splits).unwrap().to_path_buf();
    0
  }
}
