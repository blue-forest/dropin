use std::collections::BTreeMap;

use super::{Expression, KeyFormat, Keys};

impl Keys {
  pub fn new(
    required: BTreeMap<String, Expression>,
    keys: Vec<KeyFormat>,
  ) -> Self {
    Self { required, keys }
  }
}
