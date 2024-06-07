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
  value::{gen_rich_text, gen_value},
};

use super::Sub;

mod arithmetic;
mod comparison;
mod control;
mod logic;
mod value;

pub fn gen_expressions<'a, S>(
  output: &mut String,
  state: &S,
  trace: &[&str],
  expression: &Expression,
) -> fmt::Result
where
  S: Sub<'a>,
{
  write!(output, "(")?;
  match expression.expression_inner.as_ref().unwrap() {
    ExpressionInner::Value(value) => gen_value(output, state, trace, value)?,
    ExpressionInner::Comparison(comparison) => {
      gen_comparison(output, state, comparison)?
    }
    ExpressionInner::Logic(logic) => gen_logic(output, state, logic)?,
    ExpressionInner::Control(control) => gen_control(output, state, control)?,
    ExpressionInner::Arithmetic(arithmetic) => {
      gen_arithmetic(output, state, arithmetic)?;
    }
  }
  write!(output, ")")?;
  Ok(())
}
