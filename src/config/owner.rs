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

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use regex::Regex;

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir;

use super::{Cli, Command, ConfigError};

pub struct OwnerCommand;

impl Display for OwnerCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "owner".fmt(f)
  }
}

impl Command for OwnerCommand {
  fn run(&self, cli: &mut Cli) -> bool {
    let mut commands: Vec<Box<dyn Command>> = Vec::new();
    for (i, owner) in cli.owners.iter().enumerate() {
      commands.push(Box::new(Select{
        name: owner.to_string(),
        index: i,
      }));
    }
    commands.push(Box::new(Add{}));
    cli.run_select("Owner", &commands);
    false
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
  fn run(&self, cli: &mut Cli) -> bool {
    cli.selected_owner = Some(self.index);
    true
  }
}

struct Add;

impl Display for Add {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "add".fmt(f)
  }
}

impl Command for Add {
  fn run(&self, cli: &mut Cli) -> bool {
    let owner_name = loop {
      let owner_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Owner name for your recipes ? (leave empty to cancel)")
        .allow_empty(true)
        .interact_text().unwrap();
      if owner_name.is_empty() { return false; }
      if let Err(err) = validate_owner_name(cli, &owner_name) {
        println!("{}", err);
        continue;
      }
      break owner_name;
    };
    create_dir(cli.root.join(&owner_name)).unwrap();
    println!("Owner {} created", owner_name);
    let index = cli.owners.len();
    cli.owners.push(owner_name);
    cli.selected_owner = Some(index);
    true
  }
}

fn validate_owner_name(
  cli:        &Cli,
  owner_name: &str,
) -> Result<(), ConfigError> {
  let re = Regex::new(
    r"^(\w|[.-_àâæçéèêëïîôœùûüÿÀÂÆÇÉÈÊËÏÎÔŒÙÛÜŸ])+$"
  ).unwrap();
  if !re.is_match(owner_name) {
    return Err(ConfigError::from(
      "Owner name may only be composed of alphanumerics, '.', '-' and '_'",
    ));
  }
  let owner_root = cli.root.join(owner_name);
  if owner_root.exists() {
    return Err(ConfigError::from(
      "Owner directory already exists",
    ))
  }
  Ok(())
}
