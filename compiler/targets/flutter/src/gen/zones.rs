use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_common::to_upper_camelcase;
use dropin_compiler_recipes::ir::{ComponentChildInner, ComponentZone};

use crate::gen::expressions::gen_rich_text;

use super::{expressions::gen_getter, Sub};

pub fn gen_zone<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  _trace: &[usize],
  zone: &ComponentZone,
) -> fmt::Result
where
  S: Sub<'a>,
{
  write!(output, "Row(children: [")?;
  for (i, child) in zone.blocks.iter().enumerate() {
    // let trace = &[trace, &[i]].concat();
    if i != 0 {
      write!(output, ",")?;
    }
    match child.component_child_inner.as_ref().unwrap() {
      ComponentChildInner::Text(text) => {
        write!(output, "Text(")?;
        gen_rich_text(
          output,
          component,
          state,
          &[],
          text.content.as_ref().unwrap(),
        )?;
        write!(output, ")")?;
      }
      ComponentChildInner::Input(input) => {
        write!(
          output,
          "SizedBox(width: 250, child: TextFormField(initialValue:"
        )?;
        gen_getter(
          output,
          component,
          state,
          input.on_change.as_ref().unwrap(),
        )?;
        write!(output, ", onChanged: (newText_) => {{")?;
        gen_getter(
          output,
          component,
          state,
          input.on_change.as_ref().unwrap(),
        )?;
        write!(output, "= newText_}}))")?;
      }
      ComponentChildInner::Extern(r#extern) => {
        write!(output, "{}()", to_upper_camelcase(&r#extern.path))?;
      }
    }
  }
  write!(output, "])")?;
  Ok(())
}
