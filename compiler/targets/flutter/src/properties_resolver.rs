use alloc::{
  collections::{BTreeMap, BTreeSet},
  vec::Vec,
};
use dropin_compiler_recipes::ir::{
  Component, ComponentChild, ComponentChildInner, Getter,
};

use crate::{visit::ExpressionTrace, Visit};

type PropertiesByComponent<'a> = BTreeMap<&'a str, PropertiesByProperty<'a>>;
type PropertiesByProperty<'a> =
  BTreeMap<&'a str, PropertiesByVariableOwner<'a>>;
type PropertiesByVariableOwner<'a> = BTreeMap<&'a str, Vec<&'a Getter>>;

#[derive(Default)]
pub struct PropertiesResolver<'a> {
  component_id: Option<&'a str>,
  component_blocks: &'a [ComponentChild],
  component_variables: BTreeSet<&'a str>,
  properties: PropertiesByComponent<'a>,
}

impl<'a> Visit<'a, PropertiesByComponent<'a>> for PropertiesResolver<'a> {
  fn build(self) -> PropertiesByComponent<'a> {
    self.properties
  }

  fn visit_component(&mut self, component: &'a Component, _index: usize) {
    self.component_variables.clear();
    for key_format in &component.variables.as_ref().unwrap().keys {
      self.component_variables.insert(&key_format.key);
    }
    self.component_id = Some(&component.id);
    self.component_blocks = &component.zone.as_ref().unwrap().blocks;
  }

  fn visit_getter(
    &mut self,
    getter: &'a Getter,
    trace: ExpressionTrace<'a, '_>,
  ) {
    let mut trace = &trace;
    while let ExpressionTrace::Nested(_, parent) = &trace {
      trace = parent
    }
    let ExpressionTrace::ComponentChild(trace) = trace else {
      return;
    };
    let child = &self.component_blocks.unwrap()[trace.indexes[0]];
    // TODO: dig into zones
    let ComponentChildInner::Extern(r#extern) = child else {
      return;
    };
    if self.component_variables.contains(&getter.ident) {
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

/*
use crate::{Stage, Stated};

#[derive(Debug)]
pub struct PropertiesResolver<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: PropertiesResolverState<'a>,
}

impl<'a, S> PropertiesResolver<'a, S>
where
  S: Stage,
{
  pub fn new(sub: &'a S) -> Self {
    let state = PropertiesResolverState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for PropertiesResolver<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Model {
    self.sub.ir()
  }
}

impl<'a, S> Stated<PropertiesResolverState<'a>> for PropertiesResolver<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &PropertiesResolverState<'a> {
    &self.state
  }
}


#[derive(Debug, Default)]
pub struct PropertiesResolverState<'a> {
  pub properties: PropertiesByComponent<'a>,
}

impl<'a> PropertiesResolverState<'a> {
  fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    let mut self_ = Self::default();
    let ir = sub.ir();
    for component in &ir.components {
      self_.zone(component.zone.as_ref().unwrap());
    }
    self_
  }

  fn zone(&mut self, zone: &'a ComponentZone) {
    for component in &zone.blocks {
      if let ComponentChildInner::Extern(r#extern) =
        component.component_child_inner.as_ref().unwrap()
      {
        r#extern.properties
      }
    }
  }
}
*/
