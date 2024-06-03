use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Expression, KeyFormat};

use super::{formats::gen_format, Sub};

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
    let default = required.get(&key_format.key);
    gen_format(
      output,
      state,
      &[trace, &[key_format.key.as_str()]].concat(),
      key_format.format.as_ref().unwrap(),
    )?;
    if default.is_none() {
      write!(output, "?")?;
    }
    write!(output, " {}", key_format.key)?;
    if write_default {
      if let Some(_default) = default {
        write!(output, " =")?;
        todo!("gen expression");
      }
    }
    write!(output, ";")?;
  }
  Ok(())
}
