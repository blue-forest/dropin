use alloc::{fmt, string::String};
use dropin_compiler_recipes::ir::{Expression, ExpressionInner};

use self::{comparison::gen_comparison, logic::gen_logic, value::gen_value};

use super::Sub;

mod comparison;
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
  match expression.expression_inner.as_ref().unwrap() {
    ExpressionInner::Value(value) => gen_value(output, state, trace, value)?,
    ExpressionInner::Comparison(comparison) => {
      gen_comparison(output, state, comparison)?
    }
    ExpressionInner::Logic(logic) => gen_logic(output, state, logic)?,
    ExpressionInner::Control(value) => todo!("{value:?}"),
    ExpressionInner::Arithmetic(_) => todo!(),
  }
  Ok(())
}
