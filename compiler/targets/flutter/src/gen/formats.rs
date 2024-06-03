use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Format, FormatInner};

pub fn gen_format(output: &mut String, format: &Format) -> fmt::Result {
  let format = format.format_inner.as_ref().unwrap();
  match format {
    FormatInner::Any(_) => write!(output, "dynamic")?,
    FormatInner::Boolean(_) => write!(output, "bool")?,
    FormatInner::Choices(_) => todo!(),
    FormatInner::Date(_) => todo!(),
    FormatInner::Index(sub) => {
      write!(output, "Map<String,")?;
      gen_format(output, sub.format.as_ref().unwrap())?;
      write!(output, ">")?;
    }
    FormatInner::List(sub) => {
      write!(output, "List<")?;
      gen_format(output, sub.format.as_ref().unwrap())?;
      write!(output, ">")?;
    }
    FormatInner::Object(_keys) => todo!(),
    FormatInner::Quantity(_) => write!(output, "num")?,
    FormatInner::Text(_) => write!(output, "String")?,
  }
  Ok(())
}
