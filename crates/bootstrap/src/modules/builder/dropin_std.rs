use wasm_encoder::ValType::{self, I32};

use super::ModuleBuilder;
use super::import::FunctionImport;

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

impl<'module> ModuleBuilder<'module> {
  pub fn from_std(&mut self, f: &STDFunction<'module>) -> u32 {
    if let Some(id) = f.id {
      return id;
    }
    let type_id = self.types.len();
    self.types.function(f.params.clone(), f.results.clone());
    let result = self.functions_imported.len() as u32;
    self.functions_imported.push(FunctionImport{
      type_id, module: "blueforest:dropin-std:v1", name: f.name,
    });
    result
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
