use super::{ComponentChild, ComponentChildInner};

impl ComponentChild {
  pub fn new(inner: ComponentChildInner) -> Self {
    Self {
      component_child_inner: Some(inner),
    }
  }
}
