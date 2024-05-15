/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

/*
pub use self::arithmetic::Arithmetic;
pub use self::comparison::Comparison;
pub use self::component::{Component, Zone};
pub use self::control::Control;
pub use self::expression::Expression;
pub use self::logic::Logic;
pub use self::value::Value;

mod arithmetic;
mod comparison;
mod component;
pub mod control;
mod expression;
mod logic;
mod value;
*/

include!(concat!(env!("OUT_DIR"), "/dropin.components.rs"));

use std::collections::HashMap;

use comparison::ComparisonInner;
use expression::ExpressionInner;
use logic::LogicInner;
use rich_text_part::RichTextInner;
use value::ValueInner;

use self::{arithmetic::ArithmeticInner, control::ControlInner};

impl Expression {
  // --------------------------------------------------------------------- VALUE
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

  pub fn object(values: HashMap<String, Expression>) -> Self {
    Self {
      expression_inner: Some(ExpressionInner::Value(Value {
        value_inner: Some(ValueInner::Object(Object { values })),
      })),
    }
  }

  // ---------------------------------------------------------------- COMPARISON
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

  // --------------------------------------------------------------------- LOGIC
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

  // ---------------------------------------------------------------- ARITHMETIC
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

impl RichTextPart {
  pub fn r#static(content: String) -> Self {
    Self {
      rich_text_inner: Some(RichTextInner::Static(content)),
    }
  }

  pub fn dynamic(content: Expression) -> Self {
    Self {
      rich_text_inner: Some(RichTextInner::Dynamic(content)),
    }
  }
}
