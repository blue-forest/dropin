use core::marker::PhantomData;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use dropin_compiler_recipes::ir::{
  ArithmeticInner, ComparisonInner, ComponentChildInner, ComponentZone,
  ControlInner, Expression, ExpressionInner, Format, FormatInner, KeyFormat,
  LogicInner, Model, Object, RichText, RichTextInner, ValueInner,
};

use crate::visit::{ComponentChildTrace, ExpressionTrace, FormatTrace, Visit};

pub struct Stage<'a, T, V: Visit<'a, T>> {
  state: PhantomData<&'a T>,
  visit: V,
}

impl<'a, T, V: Visit<'a, T>> Stage<'a, T, V> {
  pub fn new(visit: V) -> Self {
    Self {
      state: PhantomData,
      visit,
    }
  }

  pub fn build(mut self, model: &'a Model) -> T {
    for (i, component) in model.components.iter().enumerate() {
      self.visit.visit_component(component, i);

      if let Some(properties) = component.properties.as_ref() {
        let mut trace = FormatTrace {
          component: &component.id,
          is_property: true,
          keys: Vec::new(),
        };
        self.visit.visit_properties(properties, &trace);
        self.keys(&properties.keys, &properties.required, &mut trace);
      }

      if let Some(variables) = component.variables.as_ref() {
        let mut trace = FormatTrace {
          component: &component.id,
          is_property: false,
          keys: Vec::new(),
        };
        self.visit.visit_variables(variables, &trace);
        self.keys(&variables.keys, &variables.required, &mut trace);
      }

      let mut trace = ComponentChildTrace {
        indexes: Vec::new(),
      };
      self.zone(component.zone.as_ref().unwrap(), &mut trace);
    }
    self.visit.build()
  }

  fn keys(
    &mut self,
    keys: &'a [KeyFormat],
    required: &'a BTreeMap<String, Expression>,
    trace: &mut FormatTrace<'a>,
  ) {
    trace.keys.push("");
    for key_format in keys {
      *trace.keys.last_mut().unwrap() = &key_format.key;
      self.format(key_format.format.as_ref().unwrap(), trace);
      if let Some(default) = required.get(&key_format.key) {
        let mut trace = ExpressionTrace::FormatDefaultValue(trace);
        self.expression(default, &mut trace);
      }
    }
    trace.keys.pop();
  }

  pub fn format(&mut self, format: &'a Format, trace: &mut FormatTrace<'a>) {
    self.visit.visit_format(format, trace);
    match format.format_inner.as_ref().unwrap() {
      FormatInner::Any(any) => self.visit.visit_format_any(any, trace),
      FormatInner::Boolean(boolean) => {
        self.visit.visit_format_boolean(boolean, trace)
      }
      FormatInner::Choices(choices) => {
        self.visit.visit_format_choices(choices, trace)
      }
      FormatInner::Date(date) => self.visit.visit_format_date(date, trace),
      FormatInner::Index(index) => {
        self.visit.visit_format_index(index, trace);
        trace.keys.push("*");
        self.format(index.format.as_ref().unwrap(), trace);
        trace.keys.pop();
      }
      FormatInner::List(list) => {
        self.visit.visit_format_list(list, trace);
        trace.keys.push("*");
        self.format(list.format.as_ref().unwrap(), trace);
        trace.keys.pop();
      }
      FormatInner::Object(object) => {
        self.visit.visit_format_object(object, trace);
        self.keys(&object.keys, &object.required, trace);
      }
      FormatInner::Quantity(quantity) => {
        self.visit.visit_format_quantity(quantity, trace)
      }
      FormatInner::Text(text) => self.visit.visit_format_text(text, trace),
    }
  }

  fn expression(
    &mut self,
    expression: &'a Expression,
    trace: &mut ExpressionTrace<'a, '_>,
  ) {
    self.visit.visit_expression(expression, trace);
    match expression.expression_inner.as_ref().unwrap() {
      ExpressionInner::Value(value) => {
        match value.value_inner.as_ref().unwrap() {
          ValueInner::Text(text) => {
            self.text(text, trace, Some(expression));
          }
          ValueInner::Quantity(quantity) => {
            self.visit.visit_quantity(*quantity, trace)
          }
          ValueInner::Boolean(boolean) => {
            self.visit.visit_boolean(*boolean, trace)
          }
          ValueInner::Getter(getter) => self.visit.visit_getter(getter, trace),
          ValueInner::List(list) => {
            self.visit.visit_list(list, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            for (index, element) in list.values.iter().enumerate() {
              trace.set_quantity_index(index);
              self.expression(element, &mut trace);
            }
          }
          ValueInner::Object(object) => {
            self.object(object, trace, Some(expression))
          }
          ValueInner::Undefined(_) => self.visit.visit_undefined(trace),
        }
      }
      ExpressionInner::Comparison(comparison) => {
        match comparison.comparison_inner.as_ref().unwrap() {
          ComparisonInner::EqualsTo(equals_to) => {
            self.visit.visit_equals_to(equals_to, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(equals_to.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(equals_to.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::DifferentFrom(different_from) => {
            self.visit.visit_different_from(different_from, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(different_from.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(different_from.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::In(r#in) => {
            self.visit.visit_in(r#in, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(r#in.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(r#in.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::LessThan(less_than) => {
            self.visit.visit_less_than(less_than, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(less_than.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(less_than.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::MoreThan(more_than) => {
            self.visit.visit_more_than(more_than, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(more_than.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(more_than.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::AtLeast(at_least) => {
            self.visit.visit_at_least(at_least, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(at_least.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(at_least.right.as_ref().unwrap(), &mut trace);
          }
          ComparisonInner::AtMost(at_most) => {
            self.visit.visit_at_most(at_most, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(at_most.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(at_most.right.as_ref().unwrap(), &mut trace);
          }
        }
      }
      ExpressionInner::Logic(logic) => {
        match logic.logic_inner.as_ref().unwrap() {
          LogicInner::And(and) => {
            self.visit.visit_and(and, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            for (index, operand) in and.operands.iter().enumerate() {
              trace.set_quantity_index(index);
              self.expression(operand, &mut trace);
            }
          }
          LogicInner::Or(or) => {
            self.visit.visit_or(or, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            for (index, operand) in or.operands.iter().enumerate() {
              trace.set_quantity_index(index);
              self.expression(operand, &mut trace);
            }
          }
          LogicInner::Not(not) => {
            self.visit.visit_not(not, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(not, &mut trace);
          }
          LogicInner::Exists(exists) => {
            self.visit.visit_exists(exists, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(exists, &mut trace);
          }
        }
      }
      ExpressionInner::Control(control) => {
        match control.control_inner.as_ref().unwrap() {
          ControlInner::If(r#if) => {
            self.visit.visit_if(r#if, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(r#if.condition.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(r#if.then.as_ref().unwrap(), &mut trace);
            if let Some(r#else) = r#if.r#else.as_ref() {
              trace.set_quantity_index(2);
              self.expression(r#else, &mut trace);
            }
          }
          ControlInner::AnonymousFunction(anonymous_function) => {
            self
              .visit
              .visit_anonymous_function(anonymous_function, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(
              anonymous_function.body.as_ref().unwrap(),
              &mut trace,
            );
          }
          ControlInner::NamedFunction(named_function) => {
            self.visit.visit_named_function(named_function, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(named_function.body.as_ref().unwrap(), &mut trace);
          }
          ControlInner::FunctionCall(function_call) => {
            self.visit.visit_function_call(function_call, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            for (i, arg) in function_call.args.iter().enumerate() {
              trace.set_quantity_index(i);
              self.expression(arg, &mut trace);
            }
          }
        }
      }
      ExpressionInner::Arithmetic(arithmetic) => {
        match arithmetic.arithmetic_inner.as_ref().unwrap() {
          ArithmeticInner::Opposite(opposite) => {
            self.visit.visit_opposite(opposite, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(&opposite, &mut trace);
          }
          ArithmeticInner::Add(add) => {
            self.visit.visit_add(add, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(add.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(add.right.as_ref().unwrap(), &mut trace);
          }
          ArithmeticInner::Sub(sub) => {
            self.visit.visit_sub(sub, trace);
            let mut trace = ExpressionTrace::NestedQuantity {
              parent: Some(expression),
              index: 0,
              trace,
            };
            self.expression(sub.left.as_ref().unwrap(), &mut trace);
            trace.set_quantity_index(1);
            self.expression(sub.right.as_ref().unwrap(), &mut trace);
          }
        }
      }
    }
  }

  fn text(
    &mut self,
    text: &'a RichText,
    trace: &mut ExpressionTrace<'a, '_>,
    parent: Option<&'a Expression>,
  ) {
    self.visit.visit_text(text, trace);
    let mut trace = ExpressionTrace::NestedQuantity {
      parent,
      index: 0,
      trace,
    };
    for (i, part) in text.parts.iter().enumerate() {
      if let RichTextInner::Dynamic(part) =
        part.rich_text_inner.as_ref().unwrap()
      {
        trace.set_quantity_index(i);
        self.expression(part, &mut trace);
      }
    }
  }

  fn object(
    &mut self,
    object: &'a Object,
    trace: &mut ExpressionTrace<'a, '_>,
    parent: Option<&'a Expression>,
  ) {
    self.visit.visit_object(object, trace);
    for (index, element) in object.values.iter() {
      let mut trace = ExpressionTrace::NestedText {
        parent,
        index,
        trace,
      };
      self.expression(element, &mut trace);
    }
  }

  fn zone(&mut self, zone: &'a ComponentZone, trace: &mut ComponentChildTrace) {
    self.visit.visit_zone(zone, trace);
    trace.indexes.push(0);
    for (i, child) in zone.blocks.iter().enumerate() {
      *trace.indexes.last_mut().unwrap() = i;
      self.visit.visit_child(child, trace);
      match child.component_child_inner.as_ref().unwrap() {
        ComponentChildInner::Text(text) => {
          self.visit.visit_child_text(text, trace);
          let mut trace = ExpressionTrace::ComponentChild(trace);
          self.text(text.content.as_ref().unwrap(), &mut trace, None);
        }
        ComponentChildInner::Input(input) => {
          self.visit.visit_child_input(input, trace)
        }
        ComponentChildInner::Extern(r#extern) => {
          self.visit.visit_child_extern(r#extern, trace);
          if let Some(properties) = r#extern.properties.as_ref() {
            let mut trace = ExpressionTrace::ComponentChild(trace);
            self.object(properties, &mut trace, None);
          }
        }
      }
    }
    trace.indexes.pop();
  }
}
