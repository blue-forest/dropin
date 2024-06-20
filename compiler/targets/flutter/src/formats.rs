use alloc::collections::BTreeMap;
use dropin_compiler_recipes::ir::Keys;

use crate::{visit::Visit, Stated};

#[derive(Debug, Default)]
pub struct FormatsState<'a> {
  properties: BTreeMap<&'a str, &'a Keys>,
  variables: BTreeMap<&'a str, &'a Keys>,
}

impl<'a> Stated<FormatsState<'a>> for FormatsState<'a> {
  fn state(&self) -> &FormatsState<'a> {
    self
  }
}

impl<'a> Visit<'a, FormatsState<'a>> for FormatsState<'a> {
  fn build(self) -> FormatsState<'a> {
    self
  }

  fn visit_component(
    &mut self,
    component: &'a dropin_compiler_recipes::ir::Component,
    _index: usize,
  ) {
    self
      .properties
      .insert(&component.id, component.properties.as_ref().unwrap());
    self
      .variables
      .insert(&component.id, component.variables.as_ref().unwrap());
  }
}
