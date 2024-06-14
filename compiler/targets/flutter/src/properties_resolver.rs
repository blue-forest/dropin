use alloc::{
  borrow::Cow,
  collections::{BTreeMap, BTreeSet, VecDeque},
  vec::Vec,
};
use dropin_compiler_common::Key;
use dropin_compiler_recipes::ir::{
  Component, ComponentChild, ComponentChildInner, Getter,
};
use itertools::iproduct;

use crate::{visit::ExpressionTrace, Stated, Visit};

type PropertiesByComponent<'a> = BTreeMap<&'a str, PropertiesByProperty<'a>>;
type PropertiesByProperty<'a> =
  BTreeMap<&'a str, PropertiesByVariableOwner<'a>>;
type PropertiesByVariableOwner<'a> = BTreeMap<&'a str, Vec<Cow<'a, Getter>>>;

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
  redirections: PropertiesByComponent<'a>,
}

impl<'a> Visit<'a, PropertiesResolverState<'a>> for PropertiesResolver<'a> {
  fn build(mut self) -> PropertiesResolverState<'a> {
    let mut to_insert = PropertiesByComponent::new();
    for (redirect_component, redirect_by_property) in &self.redirections {
      for (redirect_property, redirect_by_component) in redirect_by_property {
        for (mut redirect_owner, redirect_getters) in redirect_by_component {
          assert_eq!(
            redirect_getters.len(),
            1,
            "TODO: deal with several getters"
          );
          let redirect_getter = &redirect_getters[0];
          let mut suffix = BTreeMap::new();

          let props_by_property = loop {
            if let Some(props_by_property) = self.properties.get(redirect_owner)
            {
              break props_by_property;
            }
            let indirect = self
              .redirections
              .get(redirect_owner)
              .unwrap()
              .get(redirect_getter.ident.as_str())
              .unwrap();
            assert_eq!(indirect.len(), 1, "TODO: deal with several callers");
            let (new_owner, getters) = indirect.first_key_value().unwrap();
            let suffix = suffix
              .entry(redirect_getter.ident.as_str())
              .or_insert(VecDeque::new());
            for index in getters[0].indexes.iter().rev() {
              suffix.push_front(index.clone());
            }
            redirect_owner = new_owner;
          };
          let suffix = suffix
            .iter_mut()
            .map(|(key, suffix)| (*key, suffix.make_contiguous()))
            .collect::<BTreeMap<_, _>>();

          // for (prop_property, props_by_owner) in props_by_property {
          for (prop_component, prop_getters) in
            &props_by_property[redirect_getter.ident.as_str()]
          {
            let getters = &iproduct!(prop_getters, redirect_getters)
              .map(|(prop, redirect)| {
                Cow::Owned(Getter {
                  ident: prop.ident.clone(),
                  indexes: [
                    prop.indexes.as_slice(),
                    suffix
                      .get(redirect_getter.ident.as_str())
                      .unwrap_or(&[].as_mut_slice()),
                    &redirect.indexes,
                  ]
                  .concat(),
                })
              })
              .collect::<Vec<_>>();
            to_insert
              .entry(redirect_component)
              .or_insert(PropertiesByProperty::new())
              .entry(redirect_property)
              .or_insert(PropertiesByVariableOwner::new())
              .entry(prop_component)
              .and_modify(|current| current.extend_from_slice(getters))
              .or_insert(getters.clone());
          }
          // }
        }
      }
    }

    todo!("\n{:#?}", to_insert);
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
    let mut key = None;
    loop {
      match &trace {
        ExpressionTrace::NestedQuantity {
          trace: parent,
          index,
          ..
        } => {
          key = Some(Key::Quantity(*index));
          trace = parent;
        }
        ExpressionTrace::NestedText {
          trace: parent,
          index,
          ..
        } => {
          key = Some(Key::Text(*index));
          trace = parent;
        }
        _ => break,
      }
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
    let Some(Key::Text(property_key)) = key else {
      unreachable!();
    };
    let to_insert = if self.component_variables.contains(getter.ident.as_str())
    {
      &mut self.properties
    } else {
      &mut self.redirections
    };
    let path = &r#extern.path;
    to_insert
      .entry(path)
      .or_insert(PropertiesByProperty::new())
      .entry(property_key)
      .or_insert(PropertiesByVariableOwner::new())
      .entry(self.component_id.unwrap())
      .or_insert(Vec::with_capacity(1))
      .push(Cow::Borrowed(getter));
  }
}
