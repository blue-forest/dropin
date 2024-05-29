use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Format, FormatInner};

pub enum GenFormat {
  Any,
  Boolean,
  Choices,
  Date,
  Index,
  List,
  Object,
  Quantity,
  Text,
}

impl From<Format> for GenFormat {
  fn from(value: Format) -> Self {
    match value.format_inner.unwrap() {
      FormatInner::Any(_) => Self::Any,
      FormatInner::Boolean(_) => Self::Boolean,
      FormatInner::Choices(_) => Self::Choices,
      FormatInner::Date(_) => Self::Date,
      FormatInner::Index(_) => Self::Index,
      FormatInner::List(_) => Self::List,
      FormatInner::Object(_) => Self::Object,
      FormatInner::Quantity(_) => Self::Quantity,
      FormatInner::Text(_) => Self::Text,
    }
  }
}

impl GenFormat {
  pub fn gen(self, output: &mut String) -> fmt::Result {
    match self {
      GenFormat::Any => write!(output, "dynamic")?,
      GenFormat::Boolean => write!(output, "bool")?,
      GenFormat::Choices => todo!(),
      GenFormat::Date => todo!(),
      GenFormat::Index => write!(output, "Map<String, dynamic>")?,
      GenFormat::List => todo!(),
      GenFormat::Object => todo!(),
      GenFormat::Quantity => write!(output, "num")?,
      GenFormat::Text => write!(output, "String")?,
    }
    Ok(())
  }
}
