/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

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

