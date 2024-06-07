use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{
  ExpressionInner, Getter, RichText, RichTextInner, Value, ValueInner,
};

use crate::gen::Sub;

use super::gen_expressions;

pub fn gen_rich_text<'a, S>(
  output: &mut String,
  state: &S,
  trace: &[&str],
  value: &RichText,
) -> fmt::Result
where
  S: Sub<'a>,
{
  write!(output, "'")?;
  for part in &value.parts {
    match part.rich_text_inner.as_ref().unwrap() {
      RichTextInner::Static(part) => write!(output, "{part}")?,
      RichTextInner::Dynamic(expression) => {
        let is_braced = if let ExpressionInner::Value(Value {
          value_inner: Some(ValueInner::Getter(Getter { indexes, .. })),
        }) = expression.expression_inner.as_ref().unwrap()
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
        gen_expressions(output, state, trace, false, expression)?;
        if is_braced {
          write!(output, "}}")?;
        }
      }
    }
  }
  write!(output, "'")?;
  Ok(())
}
