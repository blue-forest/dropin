use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Format, FormatInner};

use crate::objects_getter::write_class_name;

use super::Sub;

pub fn gen_format<'a, S>(
  output: &mut String,
  state: &S,
  trace: &[&str],
  format: &Format,
) -> fmt::Result
where
  S: Sub<'a>,
{
  let format = format.format_inner.as_ref().unwrap();
  match format {
    FormatInner::Any(_) => write!(output, "dynamic")?,
    FormatInner::Boolean(_) => write!(output, "bool")?,
    FormatInner::Choices(_) => todo!(),
    FormatInner::Date(_) => todo!(),
    FormatInner::Index(sub) => {
      write!(output, "Map<String,")?;
      gen_format(
        output,
        state,
        &[trace, &["*"]].concat(),
        sub.format.as_ref().unwrap(),
      )?;
      write!(output, ">")?;
    }
    FormatInner::List(sub) => {
      write!(output, "List<")?;
      gen_format(
        output,
        state,
        &[trace, &["*"]].concat(),
        sub.format.as_ref().unwrap(),
      )?;
      write!(output, ">")?;
    }
    FormatInner::Object(_) => write_class_name(output, trace)?,
    FormatInner::Quantity(_) => write!(output, "num")?,
    FormatInner::Text(_) => write!(output, "String")?,
  }
  Ok(())
}
