use alloc::{
  fmt::{self, Write},
  string::{String, ToString},
  vec::Vec,
};
use anyhow::Result;
use dropin_compiler_recipes::ir::Component;

use crate::keys::GenKeys;

pub struct GenComponent {
  imports: Vec<String>,
  name: String,
  variables: GenKeys,
}

impl From<Component> for GenComponent {
  fn from(value: Component) -> Self {
    let imports = Vec::from(["package:flutter/widgets.dart".to_string()]);
    let name = value.name;
    Self {
      imports,
      name,
      variables: value.variables.unwrap().into(),
    }
  }
}

impl GenComponent {
  pub fn gen(self) -> Result<String, fmt::Error> {
    let mut output = String::new();
    {
      let output = &mut output;
      for import in self.imports {
        write!(output, "import '{import}';")?;
      }
      write!(
        output,
        "class {} extends StatelessWidget {{ final Core _core;",
        self.name
      )?;
      self.variables.gen(output)?;
      write!(output, "}}")?;
    }
    Ok(output)
  }
}
