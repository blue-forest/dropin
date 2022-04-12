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

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir;

use super::{Cli, Command};
use super::utils::validate_name;

pub struct OwnerCommand;

impl Display for OwnerCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "owners".fmt(f)
  }
}

impl Command for OwnerCommand {
  fn run(&self, cli: &mut Cli) -> u32 {
    let mut commands: Vec<Box<dyn Command>> = Vec::new();
    for (i, owner) in cli.owners.iter().enumerate() {
      commands.push(Box::new(Select{
        name:  owner.to_string(),
        index: i,
      }));
    }
    commands.push(Box::new(Add{}));
    cli.run_select("Owner", &commands)
  }
}

struct Select{
  name:  String,
  index: usize,
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    self.name.fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    cli.owner_selected = Some(self.index);
    cli.model_selected = None;
    cli.config.set_owner(self.name.clone());
    1
  }
}

struct Add;

impl Display for Add {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "add".fmt(f)
  }
}

impl Command for Add {
  fn run(&self, cli: &mut Cli) -> u32 {
    let (owner_name, owner_path) = loop {
      let owner_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Owner name for your recipes ? (leave empty to cancel)")
        .allow_empty(true)
        .interact_text().unwrap();
      if owner_name.is_empty() { return 0; }
      let owner_path = cli.root.join(&owner_name);
      if let Err(err) = validate_name(&owner_path, &owner_name) {
        println!("{}", err);
        continue;
      }
      break (owner_name, owner_path);
    };
    create_dir(&owner_path).unwrap();
    println!("Owner {} created", owner_name);
    let index = cli.owners.len();
    cli.owners.push(owner_name);
    cli.owner_selected = Some(index);
    cli.model_selected = None;
    cli.config.set_owner(cli.owners[index].clone());
    1
  }
}

