use humantime::format_duration;

use std::fmt::{Display, Error, Formatter};
use std::time::{Duration, SystemTime};

use crate::interactive::{Cli, Command};

pub struct Compile;

impl Display for Compile {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "compile".fmt(f)
  }
}

impl Command for Compile {
  fn run(&self, cli: &mut Cli) -> u32 {
    let owner = &cli.owners[cli.owner_selected.unwrap()];
    let model = &cli.models[cli.model_selected.unwrap()];
    let start = SystemTime::now();
    cli.embedder.compile(&cli.root, owner, model);
    let elapsed = SystemTime::now().duration_since(start).unwrap();
    println!("Compiled in {}", format_duration(
      Duration::new(elapsed.as_secs(), elapsed.as_millis() as u32 * 1_000_000),
    ));
    0
  }
}
