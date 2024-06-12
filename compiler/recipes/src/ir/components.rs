use dropin_compiler_common::to_upper_camelcase;

use super::{
  Component, ComponentChild, ComponentCommon, ComponentZone, Keys, RichText,
};

impl Component {
  pub fn new(
    properties: Option<Keys>,
    variables: Option<Keys>,
    classes: Vec<RichText>,
    blocks: Vec<ComponentChild>,
  ) -> Self {
    Self {
      id: String::new(),
      term: String::new(),
      properties,
      variables,
      zone: Some(ComponentZone {
        common: Some(ComponentCommon { classes }),
        blocks,
      }),
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.term = to_upper_camelcase(&id);
    self.id = id;
  }
}
