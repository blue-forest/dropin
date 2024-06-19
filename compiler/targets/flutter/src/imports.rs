use core::ops::Deref;

use alloc::{collections::BTreeMap, fmt::Write, string::String, vec::Vec};
use dropin_compiler_recipes::ir::{Component, ComponentExtern};

use crate::{
  visit::{ComponentChildTrace, Visit},
  EXTENSION,
};

#[derive(Debug)]
pub struct ImportsState<'a>(BTreeMap<&'a str, Vec<String>>);

impl<'a> Deref for ImportsState<'a> {
  type Target = BTreeMap<&'a str, Vec<String>>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Default)]
pub struct Imports<'a> {
  component: Option<&'a str>,
  imports: BTreeMap<&'a str, Vec<String>>,
}

impl<'a> Visit<'a, ImportsState<'a>> for Imports<'a> {
  fn build(self) -> ImportsState<'a> {
    ImportsState(self.imports)
  }

  fn visit_component(&mut self, component: &'a Component, _index: usize) {
    self.component = Some(&component.id);
    let mut imports = Vec::with_capacity(1);
    imports.push("package:flutter/material.dart".into());
    self.imports.insert(&component.id, imports);
  }

  fn visit_child_extern(
    &mut self,
    r#extern: &'a ComponentExtern,
    _trace: &ComponentChildTrace,
  ) {
    let imports = self.imports.get_mut(self.component.unwrap()).unwrap();
    let mut import =
      String::with_capacity(r#extern.path.len() + EXTENSION.len());
    write!(&mut import, "{}{EXTENSION}", r#extern.path).unwrap();
    imports.push(import);
  }
}
