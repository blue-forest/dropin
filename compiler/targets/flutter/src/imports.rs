use alloc::{string::String, vec::Vec};
use dropin_compiler_recipes::ir::Model;

use crate::{Stage, Stated};

#[derive(Debug)]
pub struct Imports<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: ImportsState,
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

impl<'a, S> Stated<ImportsState> for Imports<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &ImportsState {
    &self.state
  }
}

#[derive(Debug, Default)]
pub struct ImportsState {
  pub imports: Vec<String>,
}

impl ImportsState {
  fn new<S>(_sub: &S) -> Self
  where
    S: Stage,
  {
    let mut imports = Vec::with_capacity(1);
    imports.push("package:flutter/material.dart".into());
    Self { imports }
  }
}
