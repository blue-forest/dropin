use wasm_encoder::ValType::{self, I32};

pub struct WASI<'a> {
  pub fd_write: WASIFunction<'a>,
}

impl<'a> Default for WASI<'a> {
  fn default() -> Self {
    Self{
      fd_write: WASIFunction::new(
        "fd_write", vec![I32, I32, I32, I32], vec![I32],
      ),
    }
  }
}

pub struct WASIFunction<'a> {
  pub id:      Option<u32>,
  pub name:    &'a str,
  pub params:  Vec<ValType>,
  pub results: Vec<ValType>,
}

impl<'a> WASIFunction<'a> {
  fn new(name: &'a str, params: Vec<ValType>, results: Vec<ValType>) -> Self {
    Self{ id: None, name, params, results }
  }
}
