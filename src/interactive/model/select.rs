use std::fmt::{Display, Error, Formatter};

use crate::interactive::{Cli, Command};

pub struct Select {
  pub name:  String,
  pub index: usize,
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "select".fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    cli.model_selected = Some(self.index);
    cli.config.set_model(self.name.clone());
    2
  }
}

