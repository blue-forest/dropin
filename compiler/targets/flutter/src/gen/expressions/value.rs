use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{
  ExpressionInner, Getter, RichTextInner, Value, ValueInner,
};

use crate::gen::Sub;

use super::gen_expressions;

pub fn gen_value<'a, S>(
  output: &mut String,
  state: &S,
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
            gen_expressions(output, state, expression)?;
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
    }
    ValueInner::List(_) => todo!(),
    ValueInner::Object(_) => todo!(),
  }
  Ok(())
}
