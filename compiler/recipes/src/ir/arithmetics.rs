use super::{Arithmetic, ArithmeticInner, Binary, Expression, ExpressionInner};

impl Expression {
  pub fn opposite(value: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Arithmetic(Box::new(
        Arithmetic {
          arithmetic_inner: Some(ArithmeticInner::Opposite(Box::new(value))),
        },
      ))),
    }
  }

  pub fn add(left: Expression, right: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Arithmetic(Box::new(
        Arithmetic {
          arithmetic_inner: Some(ArithmeticInner::Add(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }

  pub fn sub(left: Expression, right: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Arithmetic(Box::new(
        Arithmetic {
          arithmetic_inner: Some(ArithmeticInner::Sub(Box::new(Binary {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
          }))),
        },
      ))),
    }
  }
}
