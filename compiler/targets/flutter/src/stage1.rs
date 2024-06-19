use crate::{
  setters_listeners::{UpdatedAndListeners, UpdatedAndListenersState},
  stage0::Stage0State,
  visit::Visit,
  Stated,
};

#[derive(Debug)]
pub struct Stage1State<'a, 'b> {
  pub stage0: &'b Stage0State<'a>,
  pub setters_listeners: UpdatedAndListenersState<'a>,
}

impl<'a, 'b, T> Stated<T> for Stage1State<'a, 'b>
where
  Stage0State<'a>: Stated<T>,
{
  fn state(&self) -> &T {
    self.stage0.state()
  }
}

impl<'a, 'b> Stated<UpdatedAndListenersState<'a>> for Stage1State<'a, 'b> {
  fn state(&self) -> &UpdatedAndListenersState<'a> {
    &self.setters_listeners
  }
}

pub struct Stage1<'a, 'b> {
  stage0: &'b Stage0State<'a>,
  setters_listeners: UpdatedAndListeners<'a, 'b>,
}

impl<'a, 'b> Stage1<'a, 'b> {
  pub fn new(stage0: &'b Stage0State<'a>) -> Self {
    Self {
      stage0,
      setters_listeners: UpdatedAndListeners::new(
        &stage0.resolver,
        &stage0.dependencies,
      ),
    }
  }
}

impl<'a, 'b> Visit<'a, Stage1State<'a, 'b>> for Stage1<'a, 'b> {
  fn build(self) -> Stage1State<'a, 'b> {
    Stage1State {
      stage0: self.stage0,
      setters_listeners: self.setters_listeners.build(),
    }
  }

  fn visit_component(
    &mut self,
    component: &'a dropin_compiler_recipes::ir::Component,
    index: usize,
  ) {
    self.setters_listeners.visit_component(component, index);
  }

  fn visit_variables(
    &mut self,
    variables: &'a dropin_compiler_recipes::ir::Keys,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_variables(variables, trace);
  }

  fn visit_properties(
    &mut self,
    properties: &'a dropin_compiler_recipes::ir::Keys,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_properties(properties, trace);
  }

  fn visit_format(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::Format,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format(format, trace);
  }

  fn visit_format_text(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatText,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_text(format, trace);
  }

  fn visit_format_any(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatCommon,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_any(format, trace);
  }

  fn visit_format_boolean(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatBoolean,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_boolean(format, trace);
  }

  fn visit_format_choices(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatChoices,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_choices(format, trace);
  }

  fn visit_format_date(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatDate,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_date(format, trace);
  }

  fn visit_format_index(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatIndex,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_index(format, trace);
  }

  fn visit_format_list(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatList,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_list(format, trace);
  }

  fn visit_format_object(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatObject,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_object(format, trace);
  }

  fn visit_format_quantity(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatQuantity,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.setters_listeners.visit_format_quantity(format, trace);
  }

  fn visit_expression(
    &mut self,
    expression: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_expression(expression, trace);
  }

  fn visit_text(
    &mut self,
    text: &'a dropin_compiler_recipes::ir::RichText,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_text(text, trace);
  }

  fn visit_quantity(
    &mut self,
    quantity: f64,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_quantity(quantity, trace);
  }

  fn visit_boolean(
    &mut self,
    boolean: bool,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_boolean(boolean, trace);
  }

  fn visit_getter(
    &mut self,
    getter: &'a dropin_compiler_recipes::ir::Getter,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_getter(getter, trace);
  }

  fn visit_list(
    &mut self,
    list: &'a dropin_compiler_recipes::ir::List,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_list(list, trace);
  }

  fn visit_object(
    &mut self,
    object: &'a dropin_compiler_recipes::ir::Object,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_object(object, trace);
  }

  fn visit_undefined(&mut self, trace: &crate::visit::ExpressionTrace<'a, '_>) {
    self.setters_listeners.visit_undefined(trace);
  }

  fn visit_equals_to(
    &mut self,
    equals_to: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_equals_to(equals_to, trace);
  }

  fn visit_different_from(
    &mut self,
    different_from: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self
      .setters_listeners
      .visit_different_from(different_from, trace);
  }

  fn visit_in(
    &mut self,
    r#in: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_in(r#in, trace);
  }

  fn visit_less_than(
    &mut self,
    less_than: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_less_than(less_than, trace);
  }

  fn visit_more_than(
    &mut self,
    more_than: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_more_than(more_than, trace);
  }

  fn visit_at_least(
    &mut self,
    at_least: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_at_least(at_least, trace);
  }

  fn visit_at_most(
    &mut self,
    at_most: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_at_most(at_most, trace);
  }

  fn visit_and(
    &mut self,
    and: &'a dropin_compiler_recipes::ir::Operands,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_and(and, trace);
  }

  fn visit_or(
    &mut self,
    or: &'a dropin_compiler_recipes::ir::Operands,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_or(or, trace);
  }

  fn visit_not(
    &mut self,
    not: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_not(not, trace);
  }

  fn visit_exists(
    &mut self,
    exists: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_exists(exists, trace);
  }

  fn visit_if(
    &mut self,
    r#if: &'a dropin_compiler_recipes::ir::If,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_if(r#if, trace);
  }

  fn visit_anonymous_function(
    &mut self,
    anonymous_function: &'a dropin_compiler_recipes::ir::AnonymousFunction,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self
      .setters_listeners
      .visit_anonymous_function(anonymous_function, trace);
  }

  fn visit_named_function(
    &mut self,
    named_function: &'a dropin_compiler_recipes::ir::NamedFunction,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self
      .setters_listeners
      .visit_named_function(named_function, trace);
  }

  fn visit_function_call(
    &mut self,
    function_call: &'a dropin_compiler_recipes::ir::FunctionCall,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self
      .setters_listeners
      .visit_function_call(function_call, trace);
  }

  fn visit_opposite(
    &mut self,
    opposite: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_opposite(opposite, trace);
  }

  fn visit_add(
    &mut self,
    add: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_add(add, trace);
  }

  fn visit_sub(
    &mut self,
    sub: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.setters_listeners.visit_sub(sub, trace);
  }

  fn visit_zone(
    &mut self,
    zone: &'a dropin_compiler_recipes::ir::ComponentZone,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.setters_listeners.visit_zone(zone, trace);
  }

  fn visit_child(
    &mut self,
    child: &'a dropin_compiler_recipes::ir::ComponentChild,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.setters_listeners.visit_child(child, trace);
  }

  fn visit_child_text(
    &mut self,
    text: &'a dropin_compiler_recipes::ir::ComponentText,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.setters_listeners.visit_child_text(text, trace);
  }

  fn visit_child_input(
    &mut self,
    input: &'a dropin_compiler_recipes::ir::ComponentInput,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.setters_listeners.visit_child_input(input, trace);
  }

  fn visit_child_extern(
    &mut self,
    r#extern: &'a dropin_compiler_recipes::ir::ComponentExtern,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.setters_listeners.visit_child_extern(r#extern, trace);
  }
}
