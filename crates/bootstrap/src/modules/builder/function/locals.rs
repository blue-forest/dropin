use wasm_encoder::{Instruction, ValType};

use dropin_modules::print_to;

use super::{FunctionBuilder, InstructionBuilder};

impl<'a> FunctionBuilder<'a> {
  pub fn local(
    &mut self, local: Local, cb: fn(u32) -> Instruction<'a>,
  ) {
    self.instructions.push_back(InstructionBuilder::Local(local, cb));
  }

  pub fn add_local(&mut self, type_: ValType) -> Local {
    match type_ {
      ValType::I32 => {
        let result = Local::I32(self.locals.i32_);
        self.locals.i32_ += 1;
        result
      }
      _ => {
        print_to(&format!("unknown type: {}", type_ as u32), 2);
        unsafe { wasi::proc_exit(1) };
        unreachable!();
      }
    }
  }
}

pub struct Locals {
  pub i32_:       u32,
  // pub i64_:       u32,
  // pub f32_:       u32,
  // pub f64_:       u32,
  // pub v128:       u32,
  // pub func_ref:   u32,
  // pub extern_ref: u32,
}

impl Default for Locals {
  fn default() -> Self {
    Self{
      i32_:       0,
      // i64_:       0,
      // f32_:       0,
      // f64_:       0,
      // v128:       0,
      // func_ref:   0,
      // extern_ref: 0,
    }
  }
}

impl Locals {
  pub fn resolve(local: &Local) -> u32 {
    /*
    if let &Local::I32(idx) = local {
     idx
    } else {
      unreachable!();
    }
    */
    let &Local::I32(idx) = local;
    idx
  }

  pub fn build(&self) -> Vec<(u32, ValType)> {
    let mut result = vec![];
    if self.i32_ != 0 {
      result.push((self.i32_, ValType::I32));
    }
    result
  }
}

#[derive(Clone)]
pub enum Local {
  I32(u32),
}

