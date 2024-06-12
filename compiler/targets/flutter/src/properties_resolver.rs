use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
use dropin_compiler_recipes::ir::{
  Component, ComponentChild, ComponentChildInner, Getter,
};

use crate::{visit::ExpressionTrace, Stated, Visit};

type PropertiesByComponent<'a> = BTreeMap<&'a str, PropertiesByProperty<'a>>;
type PropertiesByProperty<'a> =
  BTreeMap<&'a str, PropertiesByVariableOwner<'a>>;
type PropertiesByVariableOwner<'a> = BTreeMap<&'a str, Vec<&'a Getter>>;

#[derive(Debug)]
pub struct PropertiesResolverState<'a>(PropertiesByComponent<'a>);

impl<'a> Stated<PropertiesByComponent<'a>> for PropertiesResolverState<'a> {
  fn state(&self) -> &PropertiesByComponent<'a> {
    &self.0
  }
}

#[derive(Default)]
pub struct PropertiesResolver<'a> {
  component_id: Option<&'a str>,
  component_blocks: &'a [ComponentChild],
  component_variables: BTreeSet<&'a str>,
  properties: PropertiesByComponent<'a>,
}

impl<'a> Visit<'a, PropertiesResolverState<'a>> for PropertiesResolver<'a> {
  fn build(self) -> PropertiesResolverState<'a> {
    PropertiesResolverState(self.properties)
  }

  fn visit_component(&mut self, component: &'a Component, _index: usize) {
    self.component_variables.clear();
    if let Some(variables) = component.variables.as_ref() {
      for key_format in &variables.keys {
        self.component_variables.insert(&key_format.key);
      }
    }
    self.component_id = Some(&component.id);
    self.component_blocks = &component.zone.as_ref().unwrap().blocks;
  }

  fn visit_getter(
    &mut self,
    getter: &'a Getter,
    mut trace: &ExpressionTrace<'a, '_>,
  ) {
    while let ExpressionTrace::NestedQuantity { trace: parent, .. }
    | ExpressionTrace::NestedText { trace: parent, .. } = &trace
    {
      trace = parent
    }
    let ExpressionTrace::ComponentChild(trace) = trace else {
      return;
    };
    let child = &self.component_blocks[trace.indexes[0]];
    // TODO: dig into zones
    let ComponentChildInner::Extern(r#extern) =
      child.component_child_inner.as_ref().unwrap()
    else {
      return;
    };
    if self.component_variables.contains(getter.ident.as_str()) {
      self
        .properties
        .entry(self.component_id.unwrap())
        .or_insert(PropertiesByProperty::new())
        .entry(&getter.ident)
        .or_insert(PropertiesByVariableOwner::new())
        .entry(&r#extern.path)
        .or_insert(Vec::with_capacity(1))
        .push(getter);
    } else {
      todo!("indirect property")
    }
  }
}
