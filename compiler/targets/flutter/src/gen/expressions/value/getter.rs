use alloc::{
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::{
  ExpressionInner, Getter, RichText, RichTextInner, RichTextPart, Value,
  ValueInner,
};

use crate::{
  gen::{expressions::gen_expressions, Sub},
  objects_getter::ObjectGetterState,
  Stated,
};

pub fn gen_getter<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  value: &Getter,
) -> fmt::Result
where
  S: Sub<'a>,
{
  write!(output, "{}", value.ident)?;
  if !value.indexes.is_empty() {
    let objects = &<S as Stated<ObjectGetterState>>::state(state);
    let mut trace_current = Vec::new();
    trace_current.push(value.ident.as_str());
    for key in &value.indexes {
      let mut trace_key = "*";
      if let ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Text(RichText { parts })),
      }) = key.expression_inner.as_ref().unwrap()
      {
        if parts.len() == 1 {
          if let RichTextPart {
            rich_text_inner: Some(RichTextInner::Static(part)),
          } = &parts[0]
          {
            trace_key = part;
          }
        }
      }
      if objects
        .get(component)
        .map(|objects| objects.contains_key(&trace_current))
        .unwrap_or(false)
      {
        if trace_key == "*" {
          panic!("Objects cannot be indexed dynamically")
        }
        write!(output, ".{trace_key}")?;
      } else {
        write!(output, "[")?;
        gen_expressions(output, component, state, &trace_current, false, key)?;
        write!(output, "]")?;
      }
      trace_current.push(trace_key);
    }
  }
  Ok(())
}
