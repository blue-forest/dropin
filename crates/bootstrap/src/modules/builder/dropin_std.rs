use wasm_encoder::ValType::{self, I32};

pub struct STD<'a> {
  pub print: STDFunction<'a>,
  pub alloc: STDFunction<'a>,
}

impl<'a> Default for STD<'a> {
  fn default() -> Self {
    Self{
      print: STDFunction::new(
        "print", vec![I32, I32], vec![],
      ),
      alloc: STDFunction::new(
        "alloc", vec![I32, I32], vec![I32],
      ),
    }
  }
}

pub struct STDFunction<'a> {
  pub id:      Option<u32>,
  pub name:    &'a str,
  pub params:  Vec<ValType>,
  pub results: Vec<ValType>,
}

impl<'a> STDFunction<'a> {
  fn new(name: &'a str, params: Vec<ValType>, results: Vec<ValType>) -> Self {
    Self{ id: None, name, params, results }
  }
}
