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
  gen::Sub, objects_getter::write_class_name,
  objects_getter::ObjectGetterState, Stated,
};

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
    ValueInner::Text(value) => gen_rich_text(output, state, trace, value)?,
    ValueInner::Quantity(value) => write!(output, "{value}")?,
    ValueInner::Boolean(value) => {
      if *value {
        write!(output, "true")?;
      } else {
        write!(output, "false")?;
      }
    }
    ValueInner::Getter(value) => {
      write!(output, "{}", value.ident)?;
      if !value.indexes.is_empty() {
        let objects = &<S as Stated<ObjectGetterState>>::state(state).objects;
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
          if objects.contains_key(&trace_current) {
            if trace_key == "*" {
              panic!("Objects cannot be indexed dynamically")
            }
            write!(output, ".{trace_key}")?;
          } else {
            write!(output, "[")?;
            gen_expressions(output, state, &trace_current, false, key)?;
            write!(output, "]")?;
          }
          trace_current.push(trace_key);
        }
      }
    }
    ValueInner::List(values) => {
      write!(output, "[")?;
      let mut is_first = true;
      let trace_current = &[trace, &["*"]].concat();
      for value in &values.values {
        if !is_first {
          write!(output, ",")?;
        }
        is_first = false;
        gen_expressions(output, state, trace_current, false, value)?;
      }
      write!(output, "]")?;
    }
    ValueInner::Object(value) => {
      if let Some(_) = <S as Stated<ObjectGetterState>>::state(state)
        .objects
        .get(trace)
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
