use std::fmt::{Display, Error, Formatter};

use crate::interactive::{Cli, Command};

pub struct Select{
  pub name:  String,
  pub index: usize,
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
    cli.cwd.push(&cli.owners[self.index]);
    cli.cwd.push("models");
    cli.config.set_owner(self.name.clone());
    1
  }
}

