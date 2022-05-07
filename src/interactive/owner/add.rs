use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir;

use crate::interactive::{Cli, Command};
use crate::utils::validate_name;

pub struct Add;

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
