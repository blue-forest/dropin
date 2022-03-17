use super::Ref;

#[derive(Debug)]
pub struct Text {
  data: &'static [u8],
}

impl Text {
  pub fn new(data: &'static [u8]) -> Self {
    Self{ data }
  }
}

impl Ref for Text {}

