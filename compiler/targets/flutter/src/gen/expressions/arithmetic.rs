use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Arithmetic, ArithmeticInner};

use crate::gen::Sub;

use super::gen_expressions;

pub fn gen_arithmetic<'a, S>(
  output: &mut String,
  state: &S,
  arithmetic: &Arithmetic,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match arithmetic.arithmetic_inner.as_ref().unwrap() {
    ArithmeticInner::Opposite(operand) => {
      write!(output, "-")?;
      gen_expressions(output, state, &[], true, operand)?;
    }
    ArithmeticInner::Add(operands) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        operands.left.as_ref().unwrap(),
      )?;
      write!(output, "+")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        operands.right.as_ref().unwrap(),
      )?;
    }
    ArithmeticInner::Sub(operands) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        operands.left.as_ref().unwrap(),
      )?;
      write!(output, "-")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        operands.right.as_ref().unwrap(),
      )?;
    }
  }
  Ok(())
}
