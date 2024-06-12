use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Expression, ExpressionInner};

pub use self::{
  arithmetic::gen_arithmetic,
  comparison::gen_comparison,
  control::gen_control,
  logic::gen_logic,
  value::{gen_getter, gen_rich_text, gen_value},
};

use super::Sub;

mod arithmetic;
mod comparison;
mod control;
mod logic;
mod value;

pub fn gen_expressions<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  trace: &[&str],
  is_nested: bool,
  expression: &Expression,
) -> fmt::Result
where
  S: Sub<'a>,
{
  let start = output.len();
  let is_parenthesized = match expression.expression_inner.as_ref().unwrap() {
    ExpressionInner::Value(value) => {
      gen_value(output, component, state, trace, value)?;
      false
    }
    ExpressionInner::Comparison(comparison) => {
      gen_comparison(output, component, state, comparison)?;
      true
    }
    ExpressionInner::Logic(logic) => {
      gen_logic(output, component, state, logic)?;
      true
    }
    ExpressionInner::Control(control) => {
      gen_control(output, component, state, control)?;
      false
    }
    ExpressionInner::Arithmetic(arithmetic) => {
      gen_arithmetic(output, component, state, arithmetic)?;
      true
    }
  };
  if is_nested && is_parenthesized {
    output.insert(start, '(');
    write!(output, ")")?;
  }
  Ok(())
}
