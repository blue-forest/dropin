/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use wasm_ir::Compilable;

use std::collections::HashMap;
use std::sync::Arc;

use super::Ref;

#[derive(Debug)]
pub struct Object{
  data: HashMap<String, Arc<dyn Ref>>,
}

impl Object {
  pub fn new() -> Self {
    Self{
      data: HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: String, value: Arc<dyn Ref>) {
    if let Some(old_value) = self.data.insert(key, value) {
      panic!("overriding object value {:?}", old_value);
    }
  }
}

impl Compilable for Object {
  fn compile(&self, _buf: &mut Vec<u8>) { todo!() }
}

impl Ref for Object {}
