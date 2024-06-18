use crate::properties_resolver::PropertiesResolverState;
use crate::visit::{ComponentChildTrace, ExpressionTrace, Visit};
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use dropin_compiler_recipes::ir::{Component, ComponentInput, Getter};

#[derive(Debug)]
pub struct SettersAndListenersState<'a> {
  setters: BTreeMap<&'a str, Vec<Cow<'a, Getter>>>,
  listeners:
    BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<(&'a str, Cow<'a, Getter>)>>>,
}

pub struct SettersAndListeners<'a, 'b> {
  resolver: &'b PropertiesResolverState<'a>,
  component: Option<&'a str>,
  setters: BTreeMap<&'a str, Vec<Cow<'a, Getter>>>,
  listeners:
    BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<(&'a str, Cow<'a, Getter>)>>>,
}

impl<'a, 'b> SettersAndListeners<'a, 'b> {
  pub fn new(resolver: &'b PropertiesResolverState<'a>) -> Self {
    Self {
      resolver,
      component: None,
      setters: BTreeMap::default(),
      listeners: BTreeMap::default(),
    }
  }
}

impl<'a, 'b> Visit<'a, SettersAndListenersState<'a>>
  for SettersAndListeners<'a, 'b>
{
  fn build(self) -> SettersAndListenersState<'a> {
    SettersAndListenersState {
      setters: self.setters,
      listeners: self.listeners,
    }
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
    let component = self.component.unwrap();
    let resolved = if let Some(resolved) = self
      .resolver
      .0
      .get(component)
      .and_then(|resolved| resolved.get(getter.ident.as_str()))
    {
      resolved
    } else {
      &BTreeMap::from([(component, Vec::from([Cow::Borrowed(getter)]))])
    };
    for (owner, getters) in resolved {
      self
        .setters
        .entry(owner)
        .or_insert(Vec::with_capacity(getters.len()))
        .extend(getters.clone());
    }
  }

  fn visit_getter(
    &mut self,
    getter: &'a Getter,
    mut trace: &ExpressionTrace<'a, '_>,
  ) {
    loop {
      match &trace {
        ExpressionTrace::NestedQuantity { trace: parent, .. } => {
          trace = parent;
        }
        ExpressionTrace::NestedText { trace: parent, .. } => {
          trace = parent;
        }
        _ => break,
      }
    }
    let ExpressionTrace::ComponentChild(trace) = trace else {
      return;
    };
    let component = self.component.unwrap();
    let resolved = if let Some(resolved) = self
      .resolver
      .0
      .get(component)
      .and_then(|resolved| resolved.get(getter.ident.as_str()))
    {
      resolved
    } else {
      &BTreeMap::from([(component, Vec::from([Cow::Borrowed(getter)]))])
    };
    for (owner, getters) in resolved {
      self
        .listeners
        .entry(component)
        .or_insert(BTreeMap::new())
        .entry(trace.indexes.clone())
        .or_insert(Vec::with_capacity(getters.len()))
        .extend(getters.clone().into_iter().map(|getter| (*owner, getter)));
    }
  }
}
