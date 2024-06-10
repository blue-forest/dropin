use alloc::collections::BTreeSet;
use dropin_compiler_recipes::ir::{Component, Getter};

use crate::{Stage, Stated};

#[derive(Debug)]
pub struct Setters<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: SettersState<'a>,
}

impl<'a, S> Setters<'a, S>
where
  S: Stage,
{
  pub fn new(sub: &'a S) -> Self {
    let state = SettersState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for Setters<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Component {
    self.sub.ir()
  }
}

impl<'a, S> Stated<SettersState<'a>> for Setters<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &SettersState<'a> {
    &self.state
  }
}

#[derive(Debug, Default)]
pub struct SettersState<'a> {
  pub scopes: BTreeSet<&'a Getter>,
}

impl<'a> SettersState<'a> {
  fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    todo!()
    /*
    let mut self_ = Self::default();
    let ir = sub.ir();
    self_
    */
  }
}
