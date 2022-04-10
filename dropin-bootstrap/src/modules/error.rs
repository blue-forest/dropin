use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CompileError(String);

impl CompileError {
  pub fn new(message: String) -> Self { Self(message) }
}

impl From<&str> for CompileError {
  fn from(message: &str) -> Self { Self::new(message.to_string()) }
}

impl Display for CompileError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    Display::fmt(&self.0, f)
  }
}

impl Error for CompileError {}

