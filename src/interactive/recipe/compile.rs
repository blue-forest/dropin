use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use crate::interactive::path::get_version;
use super::{Cli, Command, Recipe};
use super::select::Selection;

pub struct Compile(Arc<Selection>);

impl Compile {
  pub fn new(selection: Arc<Selection>) -> Self {
    Self(selection)
  }
}

impl Display for Compile {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "compile".fmt(f)
  }
}

impl Command for Compile {
  fn run(&self, cli: &mut Cli) -> u32 {
    let mut path = get_version(cli).unwrap();
    path.push(self.0.recipe().dir_name());
    for ns in self.0.namespaces().iter() {
      path.push(&ns);
    }
    path.push(format!("{}.dropin", self.0.id()));
    todo!()
  }
}

