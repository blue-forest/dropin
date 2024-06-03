use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::Keys;

use super::formats::gen_format;

pub fn gen_keys(output: &mut String, keys: &Keys) -> fmt::Result {
  for key_format in &keys.keys {
    let default = keys.required.get(&key_format.key);
    gen_format(output, key_format.format.as_ref().unwrap())?;
    if default.is_none() {
      write!(output, "?")?;
    }
    write!(output, " {}", key_format.key)?;
    if let Some(_default) = default {
      write!(output, " =")?;
      todo!("gen expression");
    }
    write!(output, ";")?;
  }
  Ok(())
}
