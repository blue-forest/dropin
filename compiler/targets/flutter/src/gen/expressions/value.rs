use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Value, ValueInner};

use crate::{
  gen::Sub, objects_getter::write_class_name,
  objects_getter::ObjectGetterState, Stated,
};

use super::gen_expressions;

pub use self::getter::gen_getter;
pub use self::text::gen_rich_text;

mod getter;
mod text;

pub fn gen_value<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  trace: &[&str],
  value: &Value,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match value.value_inner.as_ref().unwrap() {
    ValueInner::Text(value) => {
      gen_rich_text(output, component, state, trace, value)?
    }
    ValueInner::Quantity(value) => write!(output, "{value}")?,
    ValueInner::Boolean(value) => {
      if *value {
        write!(output, "true")?;
      } else {
        write!(output, "false")?;
      }
    }
    ValueInner::Getter(value) => gen_getter(output, component, state, value)?,
    ValueInner::List(values) => {
      write!(output, "[")?;
      let mut is_first = true;
      let trace_current = &[trace, &["*"]].concat();
      for value in &values.values {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        gen_expressions(output, component, state, trace_current, false, value)?;
      }
      write!(output, "]")?;
    }
    ValueInner::Object(value) => {
      if let Some(_) = <S as Stated<ObjectGetterState>>::state(state)
        .get(component)
        .map(|component_objects| component_objects.get(trace))
        .flatten()
      {
        write_class_name(output, trace)?;
        write!(output, "(")?;
        let mut is_first = true;
        for (key, value) in &value.values {
          if !is_first {
            write!(output, ",")?;
          }
          is_first = false;
          write!(output, "{key}: ")?;
          gen_expressions(
            output,
            component,
            state,
            &[trace, &[key]].concat(),
            false,
            value,
          )?;
        }
        write!(output, ")")?;
      } else {
        write!(output, "{{")?;
        let mut is_first = true;
        for (key, value) in &value.values {
          if !is_first {
            write!(output, ",")?;
          }
          is_first = false;
          write!(output, "{key}:")?;
          gen_expressions(
            output,
            component,
            state,
            &[trace, &[key]].concat(),
            false,
            value,
          )?;
        }
        write!(output, "}}")?;
      }
    }
    ValueInner::Undefined(_) => unreachable!(),
  }
  Ok(())
}
