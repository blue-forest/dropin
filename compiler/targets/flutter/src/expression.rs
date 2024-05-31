use alloc::{
  fmt::{self, Write},
  string::{String, ToString},
  vec::Vec,
};
use dropin_compiler_recipes::ir::{
  ArithmeticInner, ComparisonInner, ControlInner, Expression, ExpressionInner,
  LogicInner, RichTextInner, ValueInner,
};

pub struct GenExpression(Vec<GenExpressionPart>);

impl From<Expression> for GenExpression {
  fn from(value: Expression) -> Self {
    Self(new_parts(value))
  }
}

impl GenExpression {
  pub fn gen(self, output: &mut String) -> fmt::Result {
    let mut iters = Vec::from([self.0.into_iter()]);
    while let Some(iter) = iters.last_mut() {
      let Some(part) = iter.next() else {
        iters.pop();
        if !iters.is_empty() {
          write!(output, ")")?;
        }
        continue;
      };
      match part {
        GenExpressionPart::Nested(nested) => {
          write!(output, "(")?;
          iters.push(nested.into_iter());
        }
        GenExpressionPart::Static(part) => {
          write!(output, "{part}")?;
        }
        GenExpressionPart::Dynamic(part) => {
          write!(output, "{part}")?;
        }
      }
    }
    Ok(())
  }
}

fn new_parts(value: Expression) -> Vec<GenExpressionPart> {
  match value.expression_inner.unwrap() {
    ExpressionInner::Value(value) => match value.value_inner.unwrap() {
      ValueInner::Text(value) => {
        let mut parts = value
          .parts
          .into_iter()
          .map(|part| {
            let part = part.rich_text_inner.unwrap();
            match part {
              RichTextInner::Static(part) => GenExpressionPart::Dynamic(part),
              RichTextInner::Dynamic(part) => {
                GenExpressionPart::Nested(new_parts(part))
              }
            }
          })
          .collect::<Vec<_>>();
        parts.insert(0, GenExpressionPart::Static("'"));
        parts.push(GenExpressionPart::Static("'"));
        parts
      }
      ValueInner::Quantity(value) => {
        Vec::from([GenExpressionPart::Dynamic(value.to_string())])
      }
      ValueInner::Boolean(value) => {
        Vec::from([GenExpressionPart::Static(if value {
          "true"
        } else {
          "false"
        })])
      }
      ValueInner::Getter(value) => {
        let mut parts = Vec::from([GenExpressionPart::Dynamic(value.ident)]);
        for index in value.indexes {
          parts.push(GenExpressionPart::Static("["));
          parts.push(GenExpressionPart::Nested(new_parts(index)));
          parts.push(GenExpressionPart::Static("]"));
        }
        parts
      }
      ValueInner::List(list) => {
        let mut parts = Vec::new();
        parts.push(GenExpressionPart::Static("["));
        for value in list.values {
          if parts.len() != 1 {
            parts.push(GenExpressionPart::Static(","));
          }
          parts.extend(new_parts(value));
        }
        parts.push(GenExpressionPart::Static("]"));
        parts
      }
      ValueInner::Object(object) => {
        let mut parts = Vec::new();
        parts.push(GenExpressionPart::Static("{"));
        for (key, value) in object.values {
          if parts.len() != 1 {
            parts.push(GenExpressionPart::Static(","));
          }
          parts.push(GenExpressionPart::Dynamic(key));
          parts.push(GenExpressionPart::Static(":"));
          parts.extend(new_parts(value));
        }
        parts.push(GenExpressionPart::Static("}"));
        parts
      }
    },
    ExpressionInner::Comparison(comparison) => {
      match comparison.comparison_inner.unwrap() {
        ComparisonInner::EqualsTo(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("=="),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ComparisonInner::DifferentFrom(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("!="),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ComparisonInner::In(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static(".contains("),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
          GenExpressionPart::Static(")"),
        ]),
        ComparisonInner::LessThan(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("<"),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ComparisonInner::MoreThan(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static(">"),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ComparisonInner::AtLeast(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static(">="),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ComparisonInner::AtMost(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("<="),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
      }
    }
    ExpressionInner::Logic(logic) => match logic.logic_inner.unwrap() {
      LogicInner::And(operands) => {
        let operands = operands.operands;
        let mut parts = Vec::with_capacity(operands.len() + operands.len() / 2);
        let mut is_first = true;
        for operand in operands {
          if !is_first {
            parts.push(GenExpressionPart::Static("&&"));
          }
          is_first = false;
          parts.push(GenExpressionPart::Nested(new_parts(operand)));
        }
        parts
      }
      LogicInner::Or(operands) => {
        let operands = operands.operands;
        let mut parts = Vec::with_capacity(operands.len() + operands.len() / 2);
        let mut is_first = true;
        for operand in operands {
          if !is_first {
            parts.push(GenExpressionPart::Static("||"));
          }
          is_first = false;
          parts.push(GenExpressionPart::Nested(new_parts(operand)));
        }
        parts
      }
      LogicInner::Not(not) => Vec::from([
        GenExpressionPart::Static("!"),
        GenExpressionPart::Nested(new_parts(*not)),
      ]),
      LogicInner::Exists(exists) => Vec::from([
        GenExpressionPart::Nested(new_parts(*exists)),
        GenExpressionPart::Static("!= null"),
      ]),
    },
    ExpressionInner::Control(control) => match control.control_inner.unwrap() {
      ControlInner::If(r#if) => {
        let mut parts = Vec::from([
          GenExpressionPart::Static("if("),
          GenExpressionPart::Nested(new_parts(*r#if.condition.unwrap())),
          GenExpressionPart::Static(") {"),
          GenExpressionPart::Nested(new_parts(*r#if.then.unwrap())),
        ]);
        if let Some(r#else) = r#if.r#else {
          parts.push(GenExpressionPart::Static("} else {"));
          parts.push(GenExpressionPart::Nested(new_parts(*r#else)));
        }
        parts.push(GenExpressionPart::Static("}"));
        parts
      }
      ControlInner::AnonymousFunction(f) => {
        let mut parts = Vec::from([GenExpressionPart::Static("(")]);
        let mut is_first = true;
        for arg in f.args {
          if !is_first {
            parts.push(GenExpressionPart::Static(","));
          }
          is_first = false;
          parts.push(GenExpressionPart::Dynamic(arg));
        }
        parts.push(GenExpressionPart::Static("{ return "));
        parts.push(GenExpressionPart::Nested(new_parts(*f.body.unwrap())));
        parts.push(GenExpressionPart::Static("; }"));
        parts
      }
      ControlInner::NamedFunction(f) => {
        let mut parts = Vec::from([GenExpressionPart::Static("(")]);
        let mut is_first = true;
        for arg in &f.args {
          if !is_first {
            parts.push(GenExpressionPart::Static(","));
          }
          is_first = false;
          parts.push(GenExpressionPart::Dynamic(arg.clone()));
        }
        parts.push(GenExpressionPart::Static(") {"));
        parts.push(GenExpressionPart::Dynamic(f.name.clone()));
        parts.push(GenExpressionPart::Static("("));
        let mut is_first = true;
        for arg in &f.args {
          if !is_first {
            parts.push(GenExpressionPart::Static(","));
          }
          is_first = false;
          parts.push(GenExpressionPart::Dynamic(arg.clone()));
        }
        parts.push(GenExpressionPart::Static(") { return "));
        parts.push(GenExpressionPart::Nested(new_parts(*f.body.unwrap())));
        parts.push(GenExpressionPart::Static("; } return "));
        parts.push(GenExpressionPart::Dynamic(f.name));
        parts.push(GenExpressionPart::Static("("));
        for arg in f.args {
          if !is_first {
            parts.push(GenExpressionPart::Static(","));
          }
          is_first = false;
          parts.push(GenExpressionPart::Dynamic(arg));
        }
        parts.push(GenExpressionPart::Static("); }"));
        parts
      }
      ControlInner::FunctionCall(call) => {
        let mut parts = Vec::from([
          GenExpressionPart::Nested(new_parts(*call.function.unwrap())),
          GenExpressionPart::Static("("),
        ]);
        let mut is_first = true;
        for arg in call.args {
          if !is_first {
            parts.push(GenExpressionPart::Static(","));
          }
          is_first = false;
          parts.push(GenExpressionPart::Nested(new_parts(arg)));
        }
        parts.push(GenExpressionPart::Static(")"));
        parts
      }
    },
    ExpressionInner::Arithmetic(arithmetic) => {
      match arithmetic.arithmetic_inner.unwrap() {
        ArithmeticInner::Opposite(value) => Vec::from([
          GenExpressionPart::Static("-"),
          GenExpressionPart::Nested(new_parts(*value)),
        ]),
        ArithmeticInner::Add(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("+"),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
        ArithmeticInner::Sub(binary) => Vec::from([
          GenExpressionPart::Nested(new_parts(*binary.left.unwrap())),
          GenExpressionPart::Static("-"),
          GenExpressionPart::Nested(new_parts(*binary.right.unwrap())),
        ]),
      }
    }
  }
}

pub enum GenExpressionPart {
  Nested(Vec<GenExpressionPart>),
  Static(&'static str),
  Dynamic(String),
}
