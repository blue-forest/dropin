use super::{Binary, Comparison, ComparisonInner, Expression, ExpressionInner};

impl Expression {
  pub fn equals_to(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::EqualsTo(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn different_from(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::DifferentFrom(Box::new(
            Binary {
              left: Some(Box::new(left)),
              right: Some(Box::new(right)),
            },
          ))),
        },
      ))),
    }
  }

  pub fn r#in(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::In(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn less_than(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::LessThan(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn more_than(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::MoreThan(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn at_least(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::AtLeast(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn at_most(left: Self, right: Self) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Comparison(Box::new(
        Comparison {
          comparison_inner: Some(ComparisonInner::AtMost(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }
}
