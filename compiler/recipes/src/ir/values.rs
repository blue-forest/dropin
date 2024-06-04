use std::collections::BTreeMap;

use super::{
  Expression, ExpressionInner, Getter, List, Object, RichText, RichTextPart,
  Undefined, Value, ValueInner,
};

impl Expression {
  pub fn text(parts: Vec<RichTextPart>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Text(RichText { parts })),
      })),
    }
  }

  pub fn quantity(value: f64) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Quantity(value)),
      })),
    }
  }

  pub fn boolean(value: bool) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Boolean(value)),
      })),
    }
  }

  pub fn getter(ident: String, indexes: Vec<Self>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Getter(Getter { ident, indexes })),
      })),
    }
  }

  pub fn list(values: Vec<Expression>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::List(List { values })),
      })),
    }
  }

  pub fn object(values: BTreeMap<String, Expression>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Object(Object { values })),
      })),
    }
  }

  pub fn undefined() -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Undefined(Undefined {})),
      })),
    }
  }
}
