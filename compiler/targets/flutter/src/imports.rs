use alloc::{collections::BTreeMap, string::String, vec::Vec};
use dropin_compiler_recipes::ir::Model;

use crate::{Stage, Stated};

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
      imports.insert(component.name.as_str(), component_imports);
    }
    Self { imports }
  }
}
