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

/*
use alloc::{collections::BTreeMap, fmt::Write, string::String, vec::Vec};
use dropin_compiler_recipes::ir::{ComponentChildInner, ComponentZone, Model};

use crate::{gen::EXTENSION, Stage, Stated};

#[derive(Debug)]
pub struct Imports<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: ImportsState<'a>,
}

impl<'a, S> Imports<'a, S>
where
  S: Stage,
{
  pub fn new(sub: &'a S) -> Self {
    let state = ImportsState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for Imports<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Model {
    self.sub.ir()
  }
}

impl<'a, S> Stated<ImportsState<'a>> for Imports<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &ImportsState<'a> {
    &self.state
  }
}

#[derive(Debug, Default)]
pub struct ImportsState<'a> {
  pub imports: BTreeMap<&'a str, Vec<String>>,
}

impl<'a> ImportsState<'a> {
  fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    let mut imports = BTreeMap::new();
    let ir = sub.ir();
    for component in &ir.components {
      let mut component_imports = Vec::with_capacity(1);
      component_imports.push("package:flutter/material.dart".into());
      Self::zone(&mut component_imports, component.zone.as_ref().unwrap());
      imports.insert(component.id.as_str(), component_imports);
    }
    Self { imports }
  }

  fn zone(imports: &mut Vec<String>, zone: &ComponentZone) {
    for component in &zone.blocks {
      if let ComponentChildInner::Extern(r#extern) =
        component.component_child_inner.as_ref().unwrap()
      {
        let mut import =
          String::with_capacity(r#extern.path.len() + EXTENSION.len());
        write!(&mut import, "{}{EXTENSION}", r#extern.path).unwrap();
        imports.push(import);
      }
    }
  }
}
*/
