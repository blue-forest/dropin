use alloc::{collections::BTreeMap, vec::Vec};
use dropin_compiler_recipes::ir::{
  ArithmeticInner, ComparisonInner, ComponentChildInner, ControlInner,
  Expression, ExpressionInner, LogicInner, Model, RichText, RichTextInner,
  ValueInner,
};

use crate::{Stage, Stated};

#[derive(Debug)]
pub struct Listeners<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: ListenersState<'a>,
}

impl<'a, S> Listeners<'a, S>
where
  S: Stage,
{
  pub fn new(sub: &'a S) -> Self {
    let state = ListenersState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for Listeners<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Model {
    self.sub.ir()
  }
}

impl<'a, S> Stated<ListenersState<'a>> for Listeners<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &ListenersState<'a> {
    &self.state
  }
}

#[derive(Debug, Default)]
pub struct ListenersState<'a> {
  pub scopes: BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<Vec<Key<'a>>>>>,
}

#[derive(Debug)]
pub enum Key<'a> {
  Text(&'a str),
  Quantity(f64),
}

impl<'a> ListenersState<'a> {
  fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    let mut self_ = Self::default();
    let ir = sub.ir();
    for component in &ir.components {
      let name = &component.name;
      for (i, child) in
        component.zone.as_ref().unwrap().blocks.iter().enumerate()
      {
        match child.component_child_inner.as_ref().unwrap() {
          ComponentChildInner::Text(child) => {
            self_.rich_text(name, &[i], child.content.as_ref().unwrap());
          }
          ComponentChildInner::Input(_child) => {}
          ComponentChildInner::Extern(_child) => {}
        }
      }
    }
    self_
  }

  fn rich_text(&mut self, name: &'a str, trace: &[usize], text: &'a RichText) {
    for part in &text.parts {
      if let RichTextInner::Dynamic(expression) =
        part.rich_text_inner.as_ref().unwrap()
      {
        self.expression(name, trace, expression)
      }
    }
  }

  fn expression(
    &mut self,
    name: &'a str,
    trace: &[usize],
    expression: &'a Expression,
  ) {
    match expression.expression_inner.as_ref().unwrap() {
      ExpressionInner::Value(value) => {
        if let ValueInner::Getter(getter) = value.value_inner.as_ref().unwrap()
        {
          let mut path = Vec::with_capacity(getter.indexes.len() + 1);
          path.push(Key::Text(&getter.ident));
          for index in &getter.indexes {
            let ExpressionInner::Value(index) =
              index.expression_inner.as_ref().unwrap()
            else {
              break;
            };
            match index.value_inner.as_ref().unwrap() {
              ValueInner::Text(text) => {
                if text.parts.len() != 1 {
                  break;
                }
                let RichTextInner::Static(text) =
                  text.parts[0].rich_text_inner.as_ref().unwrap()
                else {
                  break;
                };
                path.push(Key::Text(text));
              }
              ValueInner::Quantity(quantity) => {
                path.push(Key::Quantity(*quantity))
              }
              _ => {
                break;
              }
            }
          }
          self
            .scopes
            .entry(name)
            .or_insert(BTreeMap::new())
            .entry(trace.to_vec())
            .or_insert(Vec::with_capacity(1))
            .push(path);
        }
      }
      ExpressionInner::Comparison(comparison) => {
        match comparison.comparison_inner.as_ref().unwrap() {
          ComparisonInner::EqualsTo(equals_to) => {
            self.expression(name, trace, equals_to.left.as_ref().unwrap());
            self.expression(name, trace, equals_to.right.as_ref().unwrap());
          }
          ComparisonInner::DifferentFrom(different_from) => {
            self.expression(name, trace, different_from.left.as_ref().unwrap());
            self.expression(
              name,
              trace,
              different_from.right.as_ref().unwrap(),
            );
          }
          ComparisonInner::In(r#in) => {
            self.expression(name, trace, r#in.left.as_ref().unwrap());
            self.expression(name, trace, r#in.right.as_ref().unwrap());
          }
          ComparisonInner::LessThan(less_than) => {
            self.expression(name, trace, less_than.left.as_ref().unwrap());
            self.expression(name, trace, less_than.right.as_ref().unwrap());
          }
          ComparisonInner::MoreThan(more_than) => {
            self.expression(name, trace, more_than.left.as_ref().unwrap());
            self.expression(name, trace, more_than.right.as_ref().unwrap());
          }
          ComparisonInner::AtLeast(at_least) => {
            self.expression(name, trace, at_least.left.as_ref().unwrap());
            self.expression(name, trace, at_least.right.as_ref().unwrap());
          }
          ComparisonInner::AtMost(at_most) => {
            self.expression(name, trace, at_most.left.as_ref().unwrap());
            self.expression(name, trace, at_most.right.as_ref().unwrap());
          }
        }
      }
      ExpressionInner::Logic(logic) => {
        match logic.logic_inner.as_ref().unwrap() {
          LogicInner::And(and) => {
            for operand in &and.operands {
              self.expression(name, trace, operand);
            }
          }
          LogicInner::Or(or) => {
            for operand in &or.operands {
              self.expression(name, trace, operand);
            }
          }
          LogicInner::Not(not) => self.expression(name, trace, not),
          LogicInner::Exists(exists) => self.expression(name, trace, exists),
        }
      }
      ExpressionInner::Control(control) => {
        match control.control_inner.as_ref().unwrap() {
          ControlInner::If(r#if) => {
            self.expression(name, trace, r#if.condition.as_ref().unwrap());
            self.expression(name, trace, r#if.then.as_ref().unwrap());
            if let Some(r#else) = &r#if.r#else {
              self.expression(name, trace, r#else);
            }
          }
          ControlInner::AnonymousFunction(_) => {}
          ControlInner::NamedFunction(_) => {}
          ControlInner::FunctionCall(function_call) => {
            for arg in &function_call.args {
              self.expression(name, trace, arg);
            }
          }
        }
      }
      ExpressionInner::Arithmetic(arithmetic) => {
        match arithmetic.arithmetic_inner.as_ref().unwrap() {
          ArithmeticInner::Opposite(opposite) => {
            self.expression(name, trace, opposite)
          }
          ArithmeticInner::Add(add) => {
            self.expression(name, trace, add.left.as_ref().unwrap());
            self.expression(name, trace, add.right.as_ref().unwrap());
          }
          ArithmeticInner::Sub(sub) => {
            self.expression(name, trace, sub.left.as_ref().unwrap());
            self.expression(name, trace, sub.right.as_ref().unwrap());
          }
        }
      }
    }
  }
}
