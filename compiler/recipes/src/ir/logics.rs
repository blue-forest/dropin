use super::{Expression, ExpressionInner, Logic, LogicInner, Operands};

impl Expression {
  pub fn and(operands: Vec<Self>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Logic(Box::new(Logic {
        logic_inner: Some(LogicInner::And(Operands { operands })),
      }))),
    }
  }

  pub fn or(operands: Vec<Self>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Logic(Box::new(Logic {
        logic_inner: Some(LogicInner::Or(Operands { operands })),
      }))),
    }
  }

  pub fn not(value: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Logic(Box::new(Logic {
        logic_inner: Some(LogicInner::Not(Box::new(value))),
      }))),
    }
  }

  pub fn exists(value: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Logic(Box::new(Logic {
        logic_inner: Some(LogicInner::Exists(Box::new(value))),
      }))),
    }
  }
}
