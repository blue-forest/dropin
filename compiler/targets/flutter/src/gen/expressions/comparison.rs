use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::{Comparison, ComparisonInner};

use crate::gen::{expressions::gen_expressions, Sub};

pub fn gen_comparison<'a, S>(
  output: &mut String,
  state: &S,
  comparison: &Comparison,
) -> fmt::Result
where
  S: Sub<'a>,
{
  match comparison.comparison_inner.as_ref().unwrap() {
    ComparisonInner::EqualsTo(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, "==")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
    ComparisonInner::DifferentFrom(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, "!=")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
    ComparisonInner::In(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, ".contains(")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
      write!(output, ")")?;
    }
    ComparisonInner::LessThan(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, "<")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
    ComparisonInner::MoreThan(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, ">")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
    ComparisonInner::AtLeast(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, ">=")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
    ComparisonInner::AtMost(comparison) => {
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.left.as_ref().unwrap(),
      )?;
      write!(output, "<=")?;
      gen_expressions(
        output,
        state,
        &[],
        true,
        comparison.right.as_ref().unwrap(),
      )?;
    }
  }
  Ok(())
}
