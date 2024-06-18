use crate::properties_resolver::PropertiesResolverState;
use crate::visit::{ComponentChildTrace, Visit};
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use dropin_compiler_recipes::ir::{Component, ComponentInput, Getter};

#[derive(Debug)]
pub struct SettersState<'a>(BTreeMap<&'a str, Vec<Cow<'a, Getter>>>);

pub struct Setters<'a, 'b> {
  resolver: &'b PropertiesResolverState<'a>,
  component: Option<&'a str>,
  data: BTreeMap<&'a str, Vec<Cow<'a, Getter>>>,
}

impl<'a, 'b> Setters<'a, 'b> {
  pub fn new(resolver: &'b PropertiesResolverState<'a>) -> Self {
    Self {
      resolver,
      component: None,
      data: BTreeMap::default(),
    }
  }
}

impl<'a, 'b> Visit<'a, SettersState<'a>> for Setters<'a, 'b> {
  fn build(self) -> SettersState<'a> {
    SettersState(self.data)
  }

  fn visit_component(&mut self, component: &'a Component, _index: usize) {
    self.component = Some(&component.id);
  }

  fn visit_child_input(
    &mut self,
    input: &'a ComponentInput,
    _trace: &ComponentChildTrace,
  ) {
    let getter = input.on_change.as_ref().unwrap();
    for (owner, getters) in
      &self.resolver.0[self.component.unwrap()][getter.ident.as_str()]
    {
      self.data.insert(owner, getters.clone());
    }
  }
}
