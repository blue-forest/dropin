use core::cmp::min;

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
pub struct UpdatedAndListenersState<'a> {
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
  #[allow(unused)] // useful later
  pub owner: &'a str,
  #[allow(unused)] // useful later
  pub getter: Cow<'a, Getter>,
}

#[derive(Debug, Clone)]
pub struct UpdatedGetter<'a> {
  pub getter: Cow<'a, Getter>,
  pub is_external: bool,
  pub is_nested: bool,
  pub updated_by: BTreeMap<&'a str, Cow<'a, Getter>>,
}

impl<'a> UpdatedAndListenersState<'a> {
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

  pub fn get_notifiers(&self, component: &str) -> Vec<UpdatedGetter<'a>> {
    let updated_getters = self.get_updated_getters(component);
    let mut result = Vec::<UpdatedGetter>::with_capacity(updated_getters.len());
    for current in updated_getters {
      let mut is_added_modified = false;
      for added in &mut result {
        let Some(common_notifier) =
          get_common_notifier(&added.getter, &current.getter)
        else {
          continue;
        };
        added.getter = Cow::Owned(common_notifier);

        for (owner, current_getter) in &current.updated_by {
          if let Some((common_notifier, added_getter)) =
            added.updated_by.get_mut(owner).and_then(|added_getter| {
              get_common_notifier(&added_getter, &current_getter)
                .map(|common_notifier| (common_notifier, added_getter))
            })
          {
            *added_getter = Cow::Owned(common_notifier);
          } else {
            added.updated_by.insert(owner, current_getter.clone());
          }
        }
        is_added_modified = true;
      }
      if is_added_modified {
        continue;
      }
      result.push(current.clone())
    }
    result
  }
}

pub struct UpdatedAndListeners<'a, 'b> {
  resolver: &'b PropertiesResolverState<'a>,
  dependencies: &'b DependenciesState<'a>,
  component: Option<&'a str>,
  updated_getters: BTreeMap<&'a str, Vec<UpdatedGetter<'a>>>,
  listeners: BTreeMap<&'a str, BTreeMap<Vec<usize>, Vec<Listener<'a>>>>,
}

impl<'a, 'b> UpdatedAndListeners<'a, 'b> {
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

impl<'a, 'b> Visit<'a, UpdatedAndListenersState<'a>>
  for UpdatedAndListeners<'a, 'b>
where
  'a: 'b,
{
  fn build(mut self) -> UpdatedAndListenersState<'a> {
    // insert indirect updated
    let mut updated_getters_added = BTreeMap::new();
    for (owner, updated_getters) in &mut self.updated_getters {
      for updated_getter in updated_getters.iter_mut() {
        let mut updated_by_added = BTreeMap::new();
        for (updater, updater_getter) in &updated_getter.updated_by {
          add(
            &self.dependencies,
            &self.resolver,
            None,
            owner,
            updater,
            &updater_getter,
            &mut updated_getters_added,
            &mut updated_by_added,
          );
        }
        updated_getter.updated_by.extend(updated_by_added);
      }
    }
    self.updated_getters.extend(updated_getters_added);

    // todo!("{:#?}", self.updated_getters);

    UpdatedAndListenersState {
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
              let mut updated = Vec::with_capacity(1);
              updated.push(UpdatedGetter {
                getter: resolved_getter.clone(),
                is_external: false,
                is_nested: false,
                updated_by: BTreeMap::new(),
              });
              updated
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
          is_nested: false,
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
          is_nested: false,
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
  write!(output, "notifier")?;
  write_getter_name(output, getter)
}

pub fn write_updater_name(output: &mut String, getter: &Getter) -> fmt::Result {
  write!(output, "updater")?;
  write_getter_name(output, getter)
}

fn write_getter_name(output: &mut String, getter: &Getter) -> fmt::Result {
  write!(output, "{}", to_upper_camelcase(&getter.ident))?;
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

fn add<'a>(
  dependencies: &DependenciesState<'a>,
  resolver: &PropertiesResolverState<'a>,
  add_to: Option<&[(&'a str, Cow<'a, Getter>)]>,
  owner: &'a str,
  updater: &'a str,
  updater_getter: &Cow<'a, Getter>,
  updated_getters_added: &mut BTreeMap<&'a str, Vec<UpdatedGetter<'a>>>,
  updated_by_added: &mut BTreeMap<&'a str, Cow<'a, Getter>>,
) {
  for wrapper in dependencies.between(owner, updater).iter() {
    let wrapper_getters = resolver
      .redirections
      .get(updater)
      .and_then(|resolved| resolved.get(updater_getter.ident.as_str()))
      .and_then(|resolved| resolved.get(wrapper))
      .map(|resolved| resolved.as_slice())
      .unwrap_or(&[]);
    let add_to = add_to.unwrap_or(&[]);
    let is_nested = !dependencies.between(wrapper, updater).is_empty();
    for wrapper_getter in wrapper_getters {
      if !is_nested && updated_getters_added.contains_key(*wrapper) {
        continue;
      }
      updated_getters_added
        .entry(*wrapper)
        .or_insert(Vec::with_capacity(1))
        .push(UpdatedGetter {
          getter: wrapper_getter.clone(),
          is_external: true,
          is_nested,
          updated_by: BTreeMap::from([(updater, updater_getter.clone())]),
        });
      let mut last = (*wrapper, wrapper_getter);
      let mut is_first = true;
      for (outer, outer_getter) in add_to.iter().rev() {
        updated_getters_added
          .entry(*outer)
          .or_insert(Vec::with_capacity(1))
          .push(UpdatedGetter {
            getter: outer_getter.clone(),
            is_external: true,
            is_nested: !is_first,
            updated_by: BTreeMap::from([(last.0, last.1.clone())]),
          });
        is_first = false;
        last = (outer, outer_getter);
      }
      add(
        dependencies,
        resolver,
        Some(&[add_to, &[(wrapper, wrapper_getter.clone())]].concat()),
        wrapper,
        updater,
        updater_getter,
        updated_getters_added,
        updated_by_added,
      );
      updated_by_added.insert(*wrapper, wrapper_getter.clone());
    }
  }
}

fn get_common_notifier(getter1: &Getter, getter2: &Getter) -> Option<Getter> {
  if getter1.ident != getter2.ident {
    return None;
  }

  let mut common_split = None;
  for (i, (added_index, current_index)) in
    getter1.indexes.iter().zip(&getter2.indexes).enumerate()
  {
    let ExpressionInner::Value(Value { value_inner }) =
      added_index.expression_inner.as_ref().unwrap()
    else {
      common_split = Some(i);
      break;
    };
    match value_inner.as_ref().unwrap() {
      ValueInner::Text(_) => {}
      ValueInner::Quantity(_) => {}
      _ => {
        common_split = Some(i);
        break;
      }
    }
    if added_index != current_index {
      return None;
    }
  }
  let common_split =
    common_split.unwrap_or(min(getter1.indexes.len(), getter2.indexes.len()));
  Some(Getter {
    ident: getter1.ident.clone(),
    indexes: if getter1.indexes.len() > getter2.indexes.len() {
      getter1.indexes[..common_split].to_vec()
    } else {
      getter2.indexes[..common_split].to_vec()
    },
  })
}
