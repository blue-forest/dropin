use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
};

use crate::{
  objects_getter::{write_class_name, ObjectGetterState},
  Stated,
};

use super::{
  expressions::gen_expressions,
  keys::{gen_keys, is_undefined},
  Sub,
};

pub fn gen_classes<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
) -> fmt::Result
where
  S: Sub<'a>,
{
  for (trace, format) in <S as Stated<ObjectGetterState>>::state(state)
    .objects
    .get(component)
    .unwrap_or(&BTreeMap::new())
  {
    write!(output, "class ")?;
    write_class_name(output, trace)?;
    write!(output, "{{")?;
    gen_keys(
      output,
      component,
      state,
      trace,
      false,
      &format.required,
      &format.keys,
    )?;

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
      if let Some(default) = format.required.get(&key_format.key) {
        if !is_undefined(default) {
          write!(output, "=")?;
          gen_expressions(
            output,
            component,
            state,
            &[trace.as_slice(), &[&key_format.key]].concat(),
            false,
            default,
          )?;
        }
      }
    }

    write!(output, "}});}}")?;
  }
  Ok(())
}
