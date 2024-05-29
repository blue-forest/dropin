use alloc::{
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::Keys;

use crate::{expression::GenExpression, format::GenFormat};

pub struct GenKeys(Vec<GenKey>);

impl From<Keys> for GenKeys {
  fn from(mut value: Keys) -> Self {
    let mut inner = Vec::with_capacity(value.keys.len());
    for key_format in value.keys {
      let default = value.required.remove(&key_format.key);
      inner.push(GenKey {
        key: key_format.key,
        format: key_format.format.unwrap().into(),
        is_required: default.is_some(),
        default: default.map(|d| d.into()),
      })
    }
    Self(inner)
  }
}

impl GenKeys {
  pub fn gen(self, output: &mut String) -> fmt::Result {
    for key in self.0 {
      key.gen(output)?;
    }
    Ok(())
  }
}

pub struct GenKey {
  key: String,
  format: GenFormat,
  is_required: bool,
  default: Option<GenExpression>,
}

impl GenKey {
  pub fn gen(self, output: &mut String) -> fmt::Result {
    if self.is_required {
      write!(output, "final ")?;
    }
    self.format.gen(output)?;
    write!(output, " {}", self.key)?;
    if let Some(default) = self.default {
      write!(output, " =")?;
      default.gen(output)?;
      write!(output, ";")?;
    }
    Ok(())
  }
}
