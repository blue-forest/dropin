use crate::{
  imports::{Imports, ImportsState},
  objects_getter::{ObjectGetter, ObjectGetterState},
  properties_resolver::{PropertiesResolver, PropertiesResolverState},
  visit::Visit,
  Stated,
};

#[derive(Debug)]
pub struct Stage0State<'a> {
  pub resolver: PropertiesResolverState<'a>,
  pub object_getter: ObjectGetterState<'a>,
  pub imports: ImportsState<'a>,
}

impl<'a> Stated<PropertiesResolverState<'a>> for Stage0State<'a> {
  fn state(&self) -> &PropertiesResolverState<'a> {
    &self.resolver
  }
}

impl<'a> Stated<ObjectGetterState<'a>> for Stage0State<'a> {
  fn state(&self) -> &ObjectGetterState<'a> {
    &self.object_getter
  }
}

impl<'a> Stated<ImportsState<'a>> for Stage0State<'a> {
  fn state(&self) -> &ImportsState<'a> {
    &self.imports
  }
}

#[derive(Default)]
pub struct Stage0<'a> {
  resolver: PropertiesResolver<'a>,
  object_getter: ObjectGetter<'a>,
  imports: Imports<'a>,
}

impl<'a> Visit<'a, Stage0State<'a>> for Stage0<'a> {
  fn build(self) -> Stage0State<'a> {
    let resolver = self.resolver.build();
    let object_getter = self.object_getter.build();
    let imports = self.imports.build();
    Stage0State {
      resolver,
      object_getter,
      imports,
    }
  }

  fn visit_component(
    &mut self,
    component: &'a dropin_compiler_recipes::ir::Component,
    index: usize,
  ) {
    self.resolver.visit_component(component, index);
    self.object_getter.visit_component(component, index);
    self.imports.visit_component(component, index);
  }

  fn visit_variables(
    &mut self,
    variables: &'a dropin_compiler_recipes::ir::Keys,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_variables(variables, trace);
    self.object_getter.visit_variables(variables, trace);
    self.imports.visit_variables(variables, trace);
  }

  fn visit_properties(
    &mut self,
    properties: &'a dropin_compiler_recipes::ir::Keys,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_properties(properties, trace);
    self.object_getter.visit_properties(properties, trace);
    self.imports.visit_properties(properties, trace);
  }

  fn visit_format(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::Format,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format(format, trace);
    self.object_getter.visit_format(format, trace);
    self.imports.visit_format(format, trace);
  }

  fn visit_format_text(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatText,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_text(format, trace);
    self.object_getter.visit_format_text(format, trace);
    self.imports.visit_format_text(format, trace);
  }

  fn visit_format_any(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatCommon,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_any(format, trace);
    self.object_getter.visit_format_any(format, trace);
    self.imports.visit_format_any(format, trace);
  }

  fn visit_format_boolean(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatBoolean,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_boolean(format, trace);
    self.object_getter.visit_format_boolean(format, trace);
    self.imports.visit_format_boolean(format, trace);
  }

  fn visit_format_choices(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatChoices,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_choices(format, trace);
    self.object_getter.visit_format_choices(format, trace);
    self.imports.visit_format_choices(format, trace);
  }

  fn visit_format_date(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatDate,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_date(format, trace);
    self.object_getter.visit_format_date(format, trace);
    self.imports.visit_format_date(format, trace);
  }

  fn visit_format_index(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatIndex,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_index(format, trace);
    self.object_getter.visit_format_index(format, trace);
    self.imports.visit_format_index(format, trace);
  }

  fn visit_format_list(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatList,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_list(format, trace);
    self.object_getter.visit_format_list(format, trace);
    self.imports.visit_format_list(format, trace);
  }

  fn visit_format_object(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatObject,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_object(format, trace);
    self.object_getter.visit_format_object(format, trace);
    self.imports.visit_format_object(format, trace);
  }

  fn visit_format_quantity(
    &mut self,
    format: &'a dropin_compiler_recipes::ir::FormatQuantity,
    trace: &crate::visit::FormatTrace<'a>,
  ) {
    self.resolver.visit_format_quantity(format, trace);
    self.object_getter.visit_format_quantity(format, trace);
    self.imports.visit_format_quantity(format, trace);
  }

  fn visit_expression(
    &mut self,
    expression: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_expression(expression, trace);
    self.object_getter.visit_expression(expression, trace);
    self.imports.visit_expression(expression, trace);
  }

  fn visit_text(
    &mut self,
    text: &'a dropin_compiler_recipes::ir::RichText,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_text(text, trace);
    self.object_getter.visit_text(text, trace);
    self.imports.visit_text(text, trace);
  }

  fn visit_quantity(
    &mut self,
    quantity: f64,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_quantity(quantity, trace);
    self.object_getter.visit_quantity(quantity, trace);
    self.imports.visit_quantity(quantity, trace);
  }

  fn visit_boolean(
    &mut self,
    boolean: bool,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_boolean(boolean, trace);
    self.object_getter.visit_boolean(boolean, trace);
    self.imports.visit_boolean(boolean, trace);
  }

  fn visit_getter(
    &mut self,
    getter: &'a dropin_compiler_recipes::ir::Getter,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_getter(getter, trace);
    self.object_getter.visit_getter(getter, trace);
    self.imports.visit_getter(getter, trace);
  }

  fn visit_list(
    &mut self,
    list: &'a dropin_compiler_recipes::ir::List,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_list(list, trace);
    self.object_getter.visit_list(list, trace);
    self.imports.visit_list(list, trace);
  }

  fn visit_object(
    &mut self,
    object: &'a dropin_compiler_recipes::ir::Object,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_object(object, trace);
    self.object_getter.visit_object(object, trace);
    self.imports.visit_object(object, trace);
  }

  fn visit_undefined(&mut self, trace: &crate::visit::ExpressionTrace<'a, '_>) {
    self.resolver.visit_undefined(trace);
    self.object_getter.visit_undefined(trace);
    self.imports.visit_undefined(trace);
  }

  fn visit_equals_to(
    &mut self,
    equals_to: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_equals_to(equals_to, trace);
    self.object_getter.visit_equals_to(equals_to, trace);
    self.imports.visit_equals_to(equals_to, trace);
  }

  fn visit_different_from(
    &mut self,
    different_from: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_different_from(different_from, trace);
    self
      .object_getter
      .visit_different_from(different_from, trace);
    self.imports.visit_different_from(different_from, trace);
  }

  fn visit_in(
    &mut self,
    r#in: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_in(r#in, trace);
    self.object_getter.visit_in(r#in, trace);
    self.imports.visit_in(r#in, trace);
  }

  fn visit_less_than(
    &mut self,
    less_than: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_less_than(less_than, trace);
    self.object_getter.visit_less_than(less_than, trace);
    self.imports.visit_less_than(less_than, trace);
  }

  fn visit_more_than(
    &mut self,
    more_than: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_more_than(more_than, trace);
    self.object_getter.visit_more_than(more_than, trace);
    self.imports.visit_more_than(more_than, trace);
  }

  fn visit_at_least(
    &mut self,
    at_least: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_at_least(at_least, trace);
    self.object_getter.visit_at_least(at_least, trace);
    self.imports.visit_at_least(at_least, trace);
  }

  fn visit_at_most(
    &mut self,
    at_most: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_at_most(at_most, trace);
    self.object_getter.visit_at_most(at_most, trace);
    self.imports.visit_at_most(at_most, trace);
  }

  fn visit_and(
    &mut self,
    and: &'a dropin_compiler_recipes::ir::Operands,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_and(and, trace);
    self.object_getter.visit_and(and, trace);
    self.imports.visit_and(and, trace);
  }

  fn visit_or(
    &mut self,
    or: &'a dropin_compiler_recipes::ir::Operands,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_or(or, trace);
    self.object_getter.visit_or(or, trace);
    self.imports.visit_or(or, trace);
  }

  fn visit_not(
    &mut self,
    not: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_not(not, trace);
    self.object_getter.visit_not(not, trace);
    self.imports.visit_not(not, trace);
  }

  fn visit_exists(
    &mut self,
    exists: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_exists(exists, trace);
    self.object_getter.visit_exists(exists, trace);
    self.imports.visit_exists(exists, trace);
  }

  fn visit_if(
    &mut self,
    r#if: &'a dropin_compiler_recipes::ir::If,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_if(r#if, trace);
    self.object_getter.visit_if(r#if, trace);
    self.imports.visit_if(r#if, trace);
  }

  fn visit_anonymous_function(
    &mut self,
    anonymous_function: &'a dropin_compiler_recipes::ir::AnonymousFunction,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self
      .resolver
      .visit_anonymous_function(anonymous_function, trace);
    self
      .object_getter
      .visit_anonymous_function(anonymous_function, trace);
    self
      .imports
      .visit_anonymous_function(anonymous_function, trace);
  }

  fn visit_named_function(
    &mut self,
    named_function: &'a dropin_compiler_recipes::ir::NamedFunction,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_named_function(named_function, trace);
    self
      .object_getter
      .visit_named_function(named_function, trace);
    self.imports.visit_named_function(named_function, trace);
  }

  fn visit_function_call(
    &mut self,
    function_call: &'a dropin_compiler_recipes::ir::FunctionCall,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_function_call(function_call, trace);
    self.object_getter.visit_function_call(function_call, trace);
    self.imports.visit_function_call(function_call, trace);
  }

  fn visit_opposite(
    &mut self,
    opposite: &'a dropin_compiler_recipes::ir::Expression,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_opposite(opposite, trace);
    self.object_getter.visit_opposite(opposite, trace);
    self.imports.visit_opposite(opposite, trace);
  }

  fn visit_add(
    &mut self,
    add: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_add(add, trace);
    self.object_getter.visit_add(add, trace);
    self.imports.visit_add(add, trace);
  }

  fn visit_sub(
    &mut self,
    sub: &'a dropin_compiler_recipes::ir::Binary,
    trace: &crate::visit::ExpressionTrace<'a, '_>,
  ) {
    self.resolver.visit_sub(sub, trace);
    self.object_getter.visit_sub(sub, trace);
    self.imports.visit_sub(sub, trace);
  }

  fn visit_zone(
    &mut self,
    zone: &'a dropin_compiler_recipes::ir::ComponentZone,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.resolver.visit_zone(zone, trace);
    self.object_getter.visit_zone(zone, trace);
    self.imports.visit_zone(zone, trace);
  }

  fn visit_child(
    &mut self,
    child: &'a dropin_compiler_recipes::ir::ComponentChild,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.resolver.visit_child(child, trace);
    self.object_getter.visit_child(child, trace);
    self.imports.visit_child(child, trace);
  }

  fn visit_child_text(
    &mut self,
    text: &'a dropin_compiler_recipes::ir::ComponentText,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.resolver.visit_child_text(text, trace);
    self.object_getter.visit_child_text(text, trace);
    self.imports.visit_child_text(text, trace);
  }

  fn visit_child_input(
    &mut self,
    input: &'a dropin_compiler_recipes::ir::ComponentInput,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.resolver.visit_child_input(input, trace);
    self.object_getter.visit_child_input(input, trace);
    self.imports.visit_child_input(input, trace);
  }

  fn visit_child_extern(
    &mut self,
    r#extern: &'a dropin_compiler_recipes::ir::ComponentExtern,
    trace: &crate::visit::ComponentChildTrace,
  ) {
    self.resolver.visit_child_extern(r#extern, trace);
    self.object_getter.visit_child_extern(r#extern, trace);
    self.imports.visit_child_extern(r#extern, trace);
  }
}
