use wasm_encoder::{Function, Instruction};

use std::collections::VecDeque;

use super::{MemoryAddress, MemoryBuilder};

pub struct FunctionBuilder<'a, 'b> {
  type_id:      u32,
  instructions: VecDeque<InstructionBuilder<'a, 'b>>,
}

impl<'a, 'b> FunctionBuilder<'a, 'b> {
  pub fn new(type_id: u32) -> Self {
    Self{ type_id, instructions: VecDeque::new() }
  }

  pub fn type_id(&self) -> u32 { self.type_id }

  pub fn basic(&mut self, instruction: Instruction<'a>) {
    self.instructions.push_back(InstructionBuilder::Basic(instruction));
  }

  pub fn memory(
    &mut self,
    addr: &'b MemoryAddress,
    cb: fn(u32) -> Instruction<'a>,
  ) {
    self.instructions.push_back(InstructionBuilder::Memory(addr, cb));
  }

  pub fn build(mut self, memory: &MemoryBuilder) -> Function {
    let mut result = Function::new(vec![]);
    while let Some(i) = self.instructions.pop_front() {
      result.instruction(&i.build(memory));
    }
    result.instruction(&Instruction::End);
    result
  }
}

enum InstructionBuilder<'a, 'b> {
  Basic(Instruction<'a>),
  Memory(&'b MemoryAddress, fn(u32) -> Instruction<'a>),
}

impl<'a, 'b> InstructionBuilder<'a, 'b> {
  fn build(self, memory: &MemoryBuilder) -> Instruction<'a> {
    match self {
      Self::Basic(result) => result,
      Self::Memory(addr, cb) => cb(memory.resolve_addr(addr))
    }
  }
}

