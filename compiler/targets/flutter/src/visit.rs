use alloc::vec::Vec;
use dropin_compiler_recipes::ir::{
  AnonymousFunction, Binary, Component, ComponentChild, ComponentExtern,
  ComponentInput, ComponentText, ComponentZone, Expression, Format,
  FormatBoolean, FormatChoices, FormatCommon, FormatDate, FormatIndex,
  FormatList, FormatObject, FormatQuantity, FormatText, FunctionCall, Getter,
  If, Keys, List, NamedFunction, Object, Operands, RichText,
};

pub trait Visit<'a, T> {
  fn build(self) -> T;

  fn visit_component(&mut self, _component: &'a Component, _index: usize) {}

  fn visit_variables(
    &mut self,
    _variables: &'a Keys,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_properties(
    &mut self,
    _properties: &'a Keys,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format(&mut self, _format: &'a Format, _trace: &FormatTrace<'a>) {}

  fn visit_format_text(
    &mut self,
    _format: &'a FormatText,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_any(
    &mut self,
    _format: &'a FormatCommon,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_boolean(
    &mut self,
    _format: &'a FormatBoolean,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_choices(
    &mut self,
    _format: &'a FormatChoices,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_date(
    &mut self,
    _format: &'a FormatDate,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_index(
    &mut self,
    _format: &'a FormatIndex,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_list(
    &mut self,
    _format: &'a FormatList,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_object(
    &mut self,
    _format: &'a FormatObject,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_format_quantity(
    &mut self,
    _format: &'a FormatQuantity,
    _trace: &FormatTrace<'a>,
  ) {
  }

  fn visit_expression(
    &mut self,
    _expression: &'a Expression,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_text(
    &mut self,
    _text: &'a RichText,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_quantity(
    &mut self,
    _quantity: f64,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_boolean(
    &mut self,
    _boolean: bool,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_getter(
    &mut self,
    _getter: &'a Getter,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_list(&mut self, _list: &'a List, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_object(
    &mut self,
    _object: &'a Object,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_undefined(&mut self, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_equals_to(
    &mut self,
    _equals_to: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_different_from(
    &mut self,
    _different_from: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_in(&mut self, _in: &'a Binary, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_less_than(
    &mut self,
    _less_than: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_more_than(
    &mut self,
    _more_than: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_at_least(
    &mut self,
    _at_least: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_at_most(
    &mut self,
    _at_most: &'a Binary,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_and(
    &mut self,
    _and: &'a Operands,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_or(&mut self, _or: &'a Operands, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_not(
    &mut self,
    _not: &'a Expression,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_exists(
    &mut self,
    _exists: &'a Expression,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_if(&mut self, _if: &'a If, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_anonymous_function(
    &mut self,
    _anonymous_function: &'a AnonymousFunction,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_named_function(
    &mut self,
    _named_function: &'a NamedFunction,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_function_call(
    &mut self,
    _function_call: &'a FunctionCall,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_opposite(
    &mut self,
    _opposite: &'a Expression,
    _trace: &ExpressionTrace<'a, '_>,
  ) {
  }

  fn visit_add(&mut self, _add: &'a Binary, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_sub(&mut self, _sub: &'a Binary, _trace: &ExpressionTrace<'a, '_>) {}

  fn visit_zone(
    &mut self,
    _zone: &'a ComponentZone,
    _trace: &ComponentChildTrace,
  ) {
  }

  fn visit_child(
    &mut self,
    _child: &'a ComponentChild,
    _trace: &ComponentChildTrace,
  ) {
  }

  fn visit_child_text(
    &mut self,
    _text: &'a ComponentText,
    _trace: &ComponentChildTrace,
  ) {
  }

  fn visit_child_input(
    &mut self,
    _input: &'a ComponentInput,
    _trace: &ComponentChildTrace,
  ) {
  }

  fn visit_child_extern(
    &mut self,
    _extern: &'a ComponentExtern,
    _trace: &ComponentChildTrace,
  ) {
  }
}

pub struct FormatTrace<'a> {
  pub component: &'a str,
  pub is_property: bool,
  pub keys: Vec<&'a str>,
}

pub enum ExpressionTrace<'a, 'b> {
  FormatDefaultValue(&'b FormatTrace<'a>),
  ComponentChild(&'b ComponentChildTrace),
  NestedQuantity {
    parent: Option<&'a Expression>,
    index: usize,
    trace: &'b ExpressionTrace<'a, 'b>,
  },
  NestedText {
    parent: Option<&'a Expression>,
    index: &'a str,
    trace: &'b ExpressionTrace<'a, 'b>,
  },
}

impl ExpressionTrace<'_, '_> {
  pub fn set_quantity_index(&mut self, new_index: usize) {
    let Self::NestedQuantity { index, .. } = self else {
      unreachable!();
    };
    *index = new_index;
  }
}

#[derive(Debug)]
pub struct ComponentChildTrace {
  pub indexes: Vec<usize>,
}
