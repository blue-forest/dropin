use std::fmt::{Display, Error, Formatter};

use dropin_utils::path::get_build;

use crate::interactive::{Cli, Command};

pub struct Run;

impl Display for Run {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "run".fmt(f)
  }
}

impl Command for Run {
  fn run(&self, cli: &mut Cli) -> u32 {
    let root = &cli.root;
    let owner = &cli.owners[cli.owner_selected.unwrap()];
    let model = &cli.models[cli.model_selected.unwrap()];
    cli.embedder.run(Some(root), &get_build(root, owner, model));
    0
  }
}
