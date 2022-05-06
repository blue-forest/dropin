use edit::edit_file;

use std::fmt::{Display, Error, Formatter};

use crate::interactive::{Cli, Command};

pub struct Edit;

impl Display for Edit {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "edit".fmt(f)
  }
}

impl Command for Edit {
  fn run(&self, cli: &mut Cli) -> u32 {
    edit_file(&cli.cwd.parent().unwrap().join(".dropin")).unwrap();
    0
  }
}

