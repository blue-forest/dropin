use std::fmt::{Display, Error, Formatter};

use super::Recipe;

pub struct Collections;

impl Display for Collections {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "collections".fmt(f)
  }
}

impl Recipe for Collections {
  fn title(&self) -> String { "Collections".to_string() }
  fn dir_name(&self) -> String { "collections".to_string() }
}

pub struct Functions;

impl Display for Functions {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "functions".fmt(f)
  }
}

impl Recipe for Functions {
  fn title(&self) -> String { "Functions".to_string() }
  fn dir_name(&self) -> String { "types".to_string() }
}

pub struct Pipelines;

impl Display for Pipelines {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "pipelines".fmt(f)
  }
}

impl Recipe for Pipelines {
  fn title(&self) -> String { "Pipelines".to_string() }
  fn dir_name(&self) -> String { "pipelines".to_string() }
}

pub struct Types;

impl Display for Types {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "types".fmt(f)
  }
}

impl Recipe for Types {
  fn title(&self) -> String { "Types".to_string() }
  fn dir_name(&self) -> String { "types".to_string() }
}

