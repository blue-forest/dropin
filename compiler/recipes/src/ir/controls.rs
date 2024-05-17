use super::{
  AnonymousFunction, Control, ControlInner, Expression, ExpressionInner,
  FunctionCall, If, NamedFunction,
};

impl Expression {
  // ------------------------------------------------------------------- CONTROL
  pub fn r#if(condition: Self, then: Self, else_: Option<Self>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Control(Box::new(Control {
        control_inner: Some(ControlInner::If(Box::new(If {
          condition: Some(Box::new(condition)),
          then: Some(Box::new(then)),
          r#else: else_.map(|e| Box::new(e)),
        }))),
      }))),
    }
  }

  pub fn anonymous_function(args: Vec<String>, body: Expression) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Control(Box::new(Control {
        control_inner: Some(ControlInner::AnonymousFunction(Box::new(
          AnonymousFunction {
            args,
            body: Some(Box::new(body)),
          },
        ))),
      }))),
    }
  }

  pub fn named_function(
    name: String,
    args: Vec<String>,
    body: Expression,
  ) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Control(Box::new(Control {
        control_inner: Some(ControlInner::NamedFunction(Box::new(
          NamedFunction {
            name,
            args,
            body: Some(Box::new(body)),
          },
        ))),
      }))),
    }
  }

  pub fn function_call(function: Expression, args: Vec<Expression>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Control(Box::new(Control {
        control_inner: Some(ControlInner::FunctionCall(Box::new(
          FunctionCall {
            function: Some(Box::new(function)),
            args,
          },
        ))),
      }))),
    }
  }
}
