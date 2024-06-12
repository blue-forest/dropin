use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{
  Expression, ExpressionInner, KeyFormat, Value, ValueInner,
};

use super::{expressions::gen_expressions, formats::gen_format, Sub};

pub fn gen_keys<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  trace: &[&str],
  write_default: bool,
  required: &BTreeMap<String, Expression>,
  keys: &[KeyFormat],
) -> fmt::Result
where
  S: Sub<'a>,
{
  for key_format in keys {
    let trace_current = &[trace, &[key_format.key.as_str()]].concat();
    let default = required.get(&key_format.key);
    gen_format(
      output,
      state,
      trace_current,
      key_format.format.as_ref().unwrap(),
    )?;
    if default.is_none() {
      write!(output, "?")?;
    }
    write!(output, " {}", key_format.key)?;
    if write_default {
      if let Some(default) = default {
        if !is_undefined(default) {
          write!(output, "=")?;
          gen_expressions(
            output,
            component,
            state,
            trace_current,
            false,
            default,
          )?;
        }
      }
    }
    write!(output, ";")?;
  }
  Ok(())
}

pub fn is_undefined(expression: &Expression) -> bool {
  if let ExpressionInner::Value(Value {
    value_inner: Some(ValueInner::Undefined(_)),
  }) = expression.expression_inner.as_ref().unwrap()
  {
    true
  } else {
    false
  }
}
