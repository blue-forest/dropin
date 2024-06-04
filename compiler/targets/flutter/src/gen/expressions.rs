use alloc::{fmt, string::String};
use dropin_compiler_recipes::ir::{Expression, ExpressionInner};

use self::value::gen_value;

use super::Sub;

mod value;

pub fn gen_expressions<'a, S>(
  output: &mut String,
  state: &S,
  expression: &Expression,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match expression.expression_inner.as_ref().unwrap() {
    ExpressionInner::Value(value) => gen_value(output, state, value)?,
    ExpressionInner::Comparison(_) => todo!(),
    ExpressionInner::Logic(_) => todo!(),
    ExpressionInner::Control(_) => todo!(),
    ExpressionInner::Arithmetic(_) => todo!(),
  }
  Ok(())
}
