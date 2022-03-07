/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

mod text;
pub use text::Text;

pub trait Type: Debug {}

#[derive(Debug)]
pub struct CustomType {
  _id:        String,
  templates: HashMap<String, Format>,
}

impl CustomType {
  pub fn new(id: String) -> Self {
    Self{
      _id: id,
      templates: HashMap::new(),
    }
  }

  pub fn add_template(&mut self, key: String, format: Format) {
    self.templates.insert(key, format);
  }
}

impl Type for CustomType {}

#[derive(Debug)]
pub struct Format {
  _type_:   Arc<dyn Type>,
  format:  HashMap<String, Format>,
  // TODO: options: Object,
}

impl Format {
  pub fn new(type_: Arc<dyn Type>) -> Self {
    Self{
      _type_: type_,
      format:  HashMap::new(),
      // options: Object::new(),
    }
  }

  pub fn set_format(&mut self, format: Format) {
    if !self.format.is_empty() {
      panic!("trying to set an existing format");
    }
    self.format.insert("".to_string(), format);
  }

  pub fn add_format(&mut self, key: String, format: Format) {
    if let Some(_) = self.format.insert(key, format) {
      panic!("trying to set an existing key format");
    }
  }
}
