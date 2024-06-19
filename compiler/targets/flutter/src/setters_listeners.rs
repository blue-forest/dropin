use crate::dependencies::DependenciesState;
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
  updated_getters: BTreeMap<&'a str, Vec<UpdatedGetter<'a>>>,
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

#[derive(Debug)]
pub struct UpdatedGetter<'a> {
  pub getter: Cow<'a, Getter>,
  pub is_external: bool,
  pub updated_by: BTreeMap<&'a str, Cow<'a, Getter>>,
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

  pub fn get_updated_getters(&self, component: &str) -> &[UpdatedGetter<'a>] {
    self
      .updated_getters
      .get(component)
      .map(|getters| getters.as_slice())
      .unwrap_or(&[])
  }
}

pub struct SettersAndListeners<'a, 'b> {
  resolver: &'b PropertiesResolverState<'a>,
  dependencies: &'b DependenciesState<'a>,
  component: Option<&'a str>,
  updated_getters: BTreeMap<&'a str, Vec<UpdatedGetter<'a>>>,
  listeners: BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<Listener<'a>>>>,
}

impl<'a, 'b> SettersAndListeners<'a, 'b> {
  pub fn new(
    resolver: &'b PropertiesResolverState<'a>,
    dependencies: &'b DependenciesState<'a>,
  ) -> Self {
    Self {
      resolver,
      dependencies,
      component: None,
      updated_getters: BTreeMap::default(),
      listeners: BTreeMap::default(),
    }
  }
}

impl<'a, 'b> Visit<'a, SettersAndListenersState<'a>>
  for SettersAndListeners<'a, 'b>
where
  'a: 'b,
{
  fn build(mut self) -> SettersAndListenersState<'a> {
    // insert indirect setters
    let mut updated_getters_added = BTreeMap::new();
    for (owner, updated_getters) in &mut self.updated_getters {
      for updated_getter in updated_getters.iter_mut() {
        let mut updated_by_added = BTreeMap::new();
        for (updater, updater_getter) in &updated_getter.updated_by {
          for wrapper in self.dependencies.between(owner, updater).iter() {
            let wrapper_getters = self
              .resolver
              .redirections
              .get(updater)
              .and_then(|resolved| resolved.get(updater_getter.ident.as_str()))
              .and_then(|resolved| resolved.get(wrapper))
              .map(|resolved| resolved.as_slice())
              .unwrap_or(&[]);
            for wrapper_getter in wrapper_getters {
              updated_getters_added.insert(
                *wrapper,
                Vec::from([UpdatedGetter {
                  getter: wrapper_getter.clone(),
                  is_external: true,
                  updated_by: BTreeMap::from([(
                    *updater,
                    updater_getter.clone(),
                  )]),
                }]),
              );
              updated_by_added.insert(*wrapper, wrapper_getter.clone());
            }
          }
        }
        updated_getter.updated_by.extend(updated_by_added);
      }
    }
    self.updated_getters.extend(updated_getters_added);

    SettersAndListenersState {
      updated_getters: self.updated_getters,
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
    if let Some(resolved) = self
      .resolver
      .get(component)
      .and_then(|resolved| resolved.get(getter.ident.as_str()))
    {
      for (owner, getters) in resolved {
        for resolved_getter in getters {
          self
            .updated_getters
            .entry(owner)
            .or_insert_with(|| {
              let mut setters = Vec::with_capacity(1);
              setters.push(UpdatedGetter {
                getter: resolved_getter.clone(),
                is_external: false,
                updated_by: BTreeMap::new(),
              });
              setters
            })
            .last_mut()
            .unwrap()
            .updated_by
            .insert(component, Cow::Borrowed(getter));
        }
      }
      self
        .updated_getters
        .entry(component)
        .or_insert(Vec::with_capacity(1))
        .push(UpdatedGetter {
          getter: Cow::Borrowed(getter),
          is_external: true,
          updated_by: BTreeMap::new(),
        });
    } else {
      self
        .updated_getters
        .entry(component)
        .or_insert(Vec::with_capacity(1))
        .push(UpdatedGetter {
          getter: Cow::Borrowed(getter),
          is_external: false,
          updated_by: BTreeMap::new(),
        });
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
