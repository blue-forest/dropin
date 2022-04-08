use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
  pub fn new(message: String) -> Self { Self(message) }
}

impl From<&str> for ParseError {
  fn from(message: &str) -> Self { Self(message.to_string()) }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    Display::fmt(&self.0, f)
  }
}

impl Error for ParseError {}

