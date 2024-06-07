use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{ComponentChildInner, ComponentZone};

use crate::gen::expressions::gen_rich_text;

use super::Sub;

pub fn gen_zone<'a, S>(
  output: &mut String,
  state: &S,
  _trace: &[usize],
  zone: &ComponentZone,
) -> fmt::Result
where
  S: Sub<'a>,
{
  for (_i, child) in zone.blocks.iter().enumerate() {
    // let trace = &[trace, &[i]].concat();
    match child.component_child_inner.as_ref().unwrap() {
      ComponentChildInner::Text(text) => {
        write!(output, "Text(")?;
        gen_rich_text(output, state, &[], text.content.as_ref().unwrap())?;
        write!(output, ")")?;
      }
    }
  }
  Ok(())
}
