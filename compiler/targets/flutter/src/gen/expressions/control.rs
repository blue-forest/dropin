use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Control, ControlInner};

use crate::gen::Sub;

use super::gen_expressions;

pub fn gen_control<'a, S>(
  output: &mut String,
  state: &S,
  control: &Control,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match control.control_inner.as_ref().unwrap() {
    ControlInner::If(control) => {
      write!(output, "if(")?;
      gen_expressions(output, state, &[], control.condition.as_ref().unwrap())?;
      write!(output, ") {{")?;
      gen_expressions(output, state, &[], control.then.as_ref().unwrap())?;
      if let Some(r#else) = &control.r#else {
        write!(output, "}} else {{")?;
        gen_expressions(output, state, &[], r#else)?;
      }
      write!(output, "}}")?;
    }
    ControlInner::AnonymousFunction(control) => {
      write!(output, "(")?;
      let mut is_first = true;
      for arg in &control.args {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        write!(output, "{arg}")?;
      }
      write!(output, "{{ return ")?;
      gen_expressions(output, state, &[], control.body.as_ref().unwrap())?;
      write!(output, "; }}")?;
    }
    ControlInner::NamedFunction(control) => {
      write!(output, "(")?;
      let mut is_first = true;
      for arg in &control.args {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        write!(output, "{arg}")?;
      }
      write!(output, ") {{{}(", control.name)?;
      let mut is_first = true;
      for arg in &control.args {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        write!(output, "{arg}")?;
      }
      write!(output, "{{ return ")?;
      gen_expressions(output, state, &[], control.body.as_ref().unwrap())?;
      write!(output, "; }} return {}(", control.name)?;
      is_first = true;
      for arg in &control.args {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        write!(output, "{arg}")?;
      }
      write!(output, "); }}")?;
    }
    ControlInner::FunctionCall(control) => {
      gen_expressions(output, state, &[], control.function.as_ref().unwrap())?;
      write!(output, "(")?;
      let mut is_first = true;
      for arg in &control.args {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        gen_expressions(output, state, &[], arg)?;
      }
      write!(output, ")")?;
    }
  }
  Ok(())
}
