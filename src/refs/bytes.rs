use super::Ref;

#[derive(Debug)]
pub struct Byte {
  #[allow(dead_code)]
  data: u8,
}

impl Byte {
  pub fn new(data: u8) -> Self {
    Self{ data }
  }
}

impl Ref for Byte {}
