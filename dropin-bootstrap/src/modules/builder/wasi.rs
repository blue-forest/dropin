use wasm_encoder::ValType::{self, I32, I64};

use super::ModuleBuilder;

pub struct WASI {
  pub fd_write: WASIFunction,
}

impl Default for WASI {
  fn default() -> Self {
    Self{
      fd_write: WASIFunction::new(vec![I32, I32, I32, I32], vec![I32]),
    }
  }
}

pub struct WASIFunction {
  id:      Option<i32>,
  params:  Vec<ValType>,
  results: Vec<ValType>,
}

impl WASIFunction {
  fn new(params: Vec<ValType>, results: Vec<ValType>) -> Self {
    Self{ id: None, params, results }
  }

  pub fn get(&mut self, builder: &mut ModuleBuilder) {
  }
}
