use core::marker::PhantomData;

use dropin_compiler_recipes::ir::Component;

use crate::{Stage, Stated};

#[derive(Debug)]
pub struct Listeners<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: ListenersState<'a>,
}

impl<'a, S> Listeners<'a, S>
where
  S: Stage,
{
  pub fn new(sub: &'a S) -> Self {
    let state = ListenersState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for Listeners<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Component {
    self.sub.ir()
  }
}

impl<'a, S> Stated<ListenersState<'a>> for Listeners<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &ListenersState<'a> {
    &self.state
  }
}

#[derive(Debug)]
pub struct ListenersState<'a> {
  tmp: PhantomData<&'a ()>,
}

impl<'a> ListenersState<'a> {
  fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    todo!()
  }
}
