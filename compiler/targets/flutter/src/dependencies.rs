use core::ops::Deref;

use alloc::collections::{BTreeMap, BTreeSet};
use dropin_compiler_recipes::ir::{Component, ComponentExtern};

use crate::visit::{ComponentChildTrace, Visit};

#[derive(Debug)]
pub struct DependenciesState<'a>(BTreeMap<&'a str, BTreeSet<&'a str>>);

impl<'a> Deref for DependenciesState<'a> {
  type Target = BTreeMap<&'a str, BTreeSet<&'a str>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a> DependenciesState<'a> {
  pub fn between(&self, from: &str, to: &str) -> BTreeSet<&'a str> {
    self[from]
      .iter()
      .filter_map(|from_dep| {
        if self
          .get(from_dep)
          .map(|dep| dep.contains(to))
          .unwrap_or(false)
        {
          Some(*from_dep)
        } else {
          None
        }
      })
      .collect::<BTreeSet<_>>()
  }
}

#[derive(Default)]
pub struct Dependencies<'a> {
  component: Option<&'a str>,
  deps: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> Visit<'a, DependenciesState<'a>> for Dependencies<'a> {
  fn build(mut self) -> DependenciesState<'a> {
    let mut is_modified = true;
    while is_modified {
      is_modified = false;
      let mut added = BTreeMap::new();
      for (&from, tos) in &self.deps {
        let added = added.entry(from).or_insert(BTreeSet::<&str>::new());
        for to in tos.iter() {
          added.extend(self.deps.get(to).unwrap_or(&BTreeSet::new()).iter());
        }
      }
      for (&from, tos) in &mut self.deps {
        let mut added = added.remove(from).unwrap();
        let old_len = tos.len();
        tos.append(&mut added);
        is_modified = is_modified || old_len != tos.len();
      }
    }
    DependenciesState(self.deps)
  }

  fn visit_component(&mut self, component: &'a Component, _index: usize) {
    self.component = Some(&component.id);
  }

  fn visit_child_extern(
    &mut self,
    r#extern: &'a ComponentExtern,
    _trace: &ComponentChildTrace,
  ) {
    self
      .deps
      .entry(self.component.unwrap())
      .or_insert(BTreeSet::new())
      .insert(&r#extern.id);
  }
}
