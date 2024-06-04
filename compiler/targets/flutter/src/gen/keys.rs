use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Expression, KeyFormat};

use super::{expressions::gen_expressions, formats::gen_format, Sub};

pub fn gen_keys<'a, S>(
  output: &mut String,
  state: &S,
  trace: &[&str],
  write_default: bool,
  required: &BTreeMap<String, Expression>,
  keys: &[KeyFormat],
) -> fmt::Result
where
  S: Sub<'a>,
{
  for key_format in keys {
    let trace_current = &[trace, &[key_format.key.as_str()]].concat();
    let default = required.get(&key_format.key);
    gen_format(
      output,
      state,
      trace_current,
      key_format.format.as_ref().unwrap(),
    )?;
    if default.is_none() {
      write!(output, "?")?;
    }
    write!(output, " {}", key_format.key)?;
    if write_default {
      if let Some(default) = default {
        write!(output, "=")?;
        gen_expressions(output, state, trace_current, default)?;
      }
    }
    write!(output, ";")?;
  }
  Ok(())
}
