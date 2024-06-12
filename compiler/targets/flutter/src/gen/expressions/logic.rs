use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Logic, LogicInner};

use crate::gen::Sub;

use super::gen_expressions;

pub fn gen_logic<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  logic: &Logic,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match logic.logic_inner.as_ref().unwrap() {
    LogicInner::And(logic) => {
      let mut is_first = true;
      for operand in &logic.operands {
        if !is_first {
          write!(output, "&&")?;
        }
        is_first = false;
        gen_expressions(output, component, state, &[], true, &operand)?;
      }
    }
    LogicInner::Or(logic) => {
      let mut is_first = true;
      for operand in &logic.operands {
        if !is_first {
          write!(output, "||")?;
        }
        is_first = false;
        gen_expressions(output, component, state, &[], true, &operand)?;
      }
    }
    LogicInner::Not(logic) => {
      write!(output, "!")?;
      gen_expressions(output, component, state, &[], true, &logic)?;
    }
    LogicInner::Exists(logic) => {
      gen_expressions(output, component, state, &[], true, &logic)?;
      write!(output, "!= null")?;
    }
  }
  Ok(())
}
