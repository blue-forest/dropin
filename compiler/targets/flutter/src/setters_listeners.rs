use crate::properties_resolver::PropertiesResolverState;
use crate::visit::{ComponentChildTrace, ExpressionTrace, Visit};
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::fmt::{self, Write};
use alloc::string::String;
use alloc::vec::Vec;
use dropin_compiler_common::to_upper_camelcase;
use dropin_compiler_recipes::ir::{
  Component, ComponentInput, ExpressionInner, Getter, RichText, RichTextInner,
  RichTextPart, Value, ValueInner,
};

#[derive(Debug)]
pub struct SettersAndListenersState<'a> {
  setters: BTreeMap<&'a str, Vec<&'a Getter>>,
  listeners: BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<Listener<'a>>>>,
}

#[derive(Debug)]
pub struct Listener<'a> {
  pub getter: &'a Getter,
  pub resolved: Vec<Resolved<'a>>,
}

#[derive(Debug)]
pub struct Resolved<'a> {
  pub owner: &'a str,
  pub getter: Cow<'a, Getter>,
}

impl<'a> SettersAndListenersState<'a> {
  pub fn get_listeners(
    &self,
    component: &str,
    trace: &[usize],
  ) -> Option<&[Listener<'a>]> {
    self
      .listeners
      .get(component)
      .and_then(|listeners| listeners.get(trace))
      .map(|listeners| listeners.as_slice())
  }

  pub fn get_updated_getters(&self, component: &str) -> &[&'a Getter] {
    self
      .setters
      .get(component)
      .map(|getters| getters.as_slice())
      .unwrap_or(&[])
  }
}

pub struct SettersAndListeners<'a, 'b> {
  resolver: &'b PropertiesResolverState<'a>,
  component: Option<&'a str>,
  setters: BTreeMap<&'a str, Vec<&'a Getter>>,
  listeners: BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<Listener<'a>>>>,
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
    self
      .setters
      .entry(component)
      .or_insert(Vec::with_capacity(1))
      .push(getter);
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
    let mut listener = Listener {
      getter,
      resolved: Vec::with_capacity(resolved.len()),
    };
    for (owner, getters) in resolved {
      listener.resolved.extend(
        getters
          .clone()
          .into_iter()
          .map(|getter| Resolved { owner, getter }),
      );
    }
    self
      .listeners
      .entry(component)
      .or_insert(BTreeMap::new())
      .entry(trace.indexes.clone())
      .or_insert(Vec::with_capacity(1))
      .push(listener)
  }
}

pub fn write_notifier_name(
  output: &mut String,
  getter: &Getter,
) -> fmt::Result {
  write!(output, "notifier{}", to_upper_camelcase(&getter.ident))?;
  for key in &getter.indexes {
    let ExpressionInner::Value(Value {
      value_inner: Some(value_inner),
    }) = key.expression_inner.as_ref().unwrap()
    else {
      break;
    };
    match value_inner {
      ValueInner::Text(RichText { parts }) => {
        if parts.len() == 1 {
          if let RichTextPart {
            rich_text_inner: Some(RichTextInner::Static(part)),
          } = &parts[0]
          {
            write!(output, "{}", to_upper_camelcase(part))?;
          }
        }
      }
      ValueInner::Quantity(index) => {
        write!(output, "{index}")?;
      }
      _ => break,
    }
  }
  write!(output, "_")?;
  Ok(())
}
