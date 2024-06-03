use alloc::{
  fmt::{self, Write},
  string::String,
};

use crate::objects_getter::write_class_name;

use super::{keys::gen_keys, Sub};

pub fn gen_classes<'a, S>(output: &mut String, state: &S) -> fmt::Result
where
  S: Sub<'a>,
{
  for (trace, format) in &state.state().objects {
    write!(output, "class ")?;
    write_class_name(output, trace)?;
    write!(output, "{{")?;
    gen_keys(output, state, trace, false, &format.required, &format.keys)?;

    // constructor
    write_class_name(output, trace)?;
    write!(output, "({{")?;
    let mut is_first = true;
    for key_format in &format.keys {
      if !is_first {
        write!(output, ",")?;
      }
      is_first = false;
      write!(output, "this.{}", key_format.key)?;
      if let Some(_default) = format.required.get(&key_format.key) {
        write!(output, " =")?;
        todo!("gen expression");
      }
    }

    write!(output, "}});}}")?;
  }
  Ok(())
}
