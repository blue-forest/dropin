use core::ops::Deref;

use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::FormatObject;

use crate::{visit::FormatTrace, Visit};

#[derive(Debug)]
pub struct ObjectGetterState<'a>(
  BTreeMap<&'a str, BTreeMap<Vec<&'a str>, &'a FormatObject>>,
);

impl<'a> Deref for ObjectGetterState<'a> {
  type Target = BTreeMap<&'a str, BTreeMap<Vec<&'a str>, &'a FormatObject>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Default)]
pub struct ObjectGetter<'a> {
  objects: BTreeMap<&'a str, BTreeMap<Vec<&'a str>, &'a FormatObject>>,
}

impl<'a, 'b> Visit<'a, ObjectGetterState<'a>> for ObjectGetter<'a> {
  fn build(self) -> ObjectGetterState<'a> {
    ObjectGetterState(self.objects)
  }

  fn visit_format_object(
    &mut self,
    format: &'a FormatObject,
    trace: &FormatTrace<'a>,
  ) {
    self
      .objects
      .entry(trace.component)
      .or_insert(BTreeMap::new())
      .insert(trace.keys.clone(), format);
  }
}

pub fn write_class_name(output: &mut String, trace: &[&str]) -> fmt::Result {
  for key in trace {
    match *key {
      "*" => {
        write!(output, "_")?;
      }
      "_" => write!(output, "__")?,
      _ => {
        let mut is_capital = true;
        for c in key.chars() {
          if c == '_' {
            is_capital = true;
            continue;
          }
          if is_capital {
            output.push(c.to_ascii_uppercase());
          } else {
            output.push(c);
          }
          is_capital = false;
        }
      }
    }
  }
  write!(output, "Object")?;
  Ok(())
}
