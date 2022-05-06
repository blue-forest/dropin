use wasm_encoder::Instruction;

use super::{Local, Locals};

pub enum InstructionBuilder<'a> {
  Basic(Instruction<'a>),
  Local(Local, fn(u32) -> Instruction<'a>),
}

impl<'a> InstructionBuilder<'a> {
  pub fn build(self) -> Instruction<'a> {
    match self {
      Self::Basic(result) => result,
      Self::Local(idx, cb) => cb(Locals::resolve(&idx)),
    }
  }
}
