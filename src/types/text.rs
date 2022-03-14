use super::Type;

#[derive(Debug)]
pub struct Text {
}

impl Text {
  pub fn new() -> Self {
    Self{}
  }
}

impl Type for Text {}
