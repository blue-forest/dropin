use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{
  ExpressionInner, Getter, RichTextInner, Value, ValueInner,
};

use crate::{gen::Sub, objects_getter::write_class_name};

use super::gen_expressions;

pub fn gen_value<'a, S>(
  output: &mut String,
  state: &S,
  trace: &[&str],
  value: &Value,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match value.value_inner.as_ref().unwrap() {
    ValueInner::Text(value) => {
      write!(output, "'")?;
      for part in &value.parts {
        match part.rich_text_inner.as_ref().unwrap() {
          RichTextInner::Static(part) => write!(output, "{part}")?,
          RichTextInner::Dynamic(expression) => {
            let is_braced = if let ExpressionInner::Value(Value {
              value_inner: Some(ValueInner::Getter(Getter { indexes, .. })),
            }) =
              expression.expression_inner.as_ref().unwrap()
            {
              if indexes.is_empty() {
                false
              } else {
                true
              }
            } else {
              true
            };
            write!(output, "$")?;
            if is_braced {
              write!(output, "{{")?;
            }
            gen_expressions(output, state, trace, expression)?;
            if is_braced {
              write!(output, "}}")?;
            }
          }
        }
      }
      write!(output, "'")?;
    }
    ValueInner::Quantity(value) => write!(output, "{value}")?,
    ValueInner::Boolean(_) => todo!(),
    ValueInner::Getter(value) => {
      write!(output, "{}", value.ident)?;
      if !value.indexes.is_empty() {
        todo!("getter indexes")
      }
    }
    ValueInner::List(_) => todo!(),
    ValueInner::Object(value) => {
      if let Some(_) = state.state().objects.get(trace) {
        write_class_name(output, trace)?;
        write!(output, "(")?;
        let mut is_first = true;
        for (key, value) in &value.values {
          if !is_first {
            write!(output, ",")?;
          }
          is_first = false;
          write!(output, "{key}: ")?;
          gen_expressions(output, state, &[trace, &[key]].concat(), value)?;
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
          gen_expressions(output, state, &[trace, &[key]].concat(), value)?;
        }
        write!(output, "}}")?;
      }
    }
  }
  Ok(())
}
