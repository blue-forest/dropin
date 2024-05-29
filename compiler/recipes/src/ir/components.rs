use super::{
  Component, ComponentChild, ComponentCommon, ComponentZone, Keys, RichText,
};

impl Component {
  pub fn new(
    variables: Option<Keys>,
    classes: Vec<RichText>,
    blocks: Vec<ComponentChild>,
  ) -> Self {
    Self {
      name: String::new(),
      variables,
      zone: Some(ComponentZone {
        common: Some(ComponentCommon { classes }),
        blocks,
      }),
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
}
