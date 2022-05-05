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

use wasm_encoder::{Function, Instruction, ValType};

use std::collections::VecDeque;

use dropin_modules::print_to;

use super::{MemoryAddress, MemoryBuilder};

pub struct FunctionBuilder<'a> {
  type_id:      u32,
  instructions: VecDeque<InstructionBuilder<'a>>,
  locals:       Locals,
}

impl<'a> FunctionBuilder<'a> {
  pub fn new(type_id: u32) -> Self {
    Self{ type_id, instructions: VecDeque::new(), locals: Locals::default() }
  }

  pub fn type_id(&self) -> u32 { self.type_id }

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

  pub fn basic(&mut self, instruction: Instruction<'a>) {
    self.instructions.push_back(InstructionBuilder::Basic(instruction));
  }

  pub fn local(
    &mut self, local: Local, cb: fn(u32) -> Instruction<'a>,
  ) {
    self.instructions.push_back(InstructionBuilder::Local(local, cb));
  }

  pub fn memory(
    &mut self, addr: &'a MemoryAddress, cb: fn(u32) -> Instruction<'a>,
  ) {
    self.instructions.push_back(InstructionBuilder::Memory(addr, cb));
  }

  pub fn build(mut self, memory: &MemoryBuilder) -> Function {
    let mut result = Function::new(self.locals.build());
    while let Some(i) = self.instructions.pop_front() {
      result.instruction(&i.build(memory));
    }
    result.instruction(&Instruction::End);
    result
  }
}

struct Locals {
  i32_:       u32,
  i64_:       u32,
  f32_:       u32,
  f64_:       u32,
  v128:       u32,
  func_ref:   u32,
  extern_ref: u32,
}

impl Default for Locals {
  fn default() -> Self {
    Self{
      i32_:       0,
      i64_:       0,
      f32_:       0,
      f64_:       0,
      v128:       0,
      func_ref:   0,
      extern_ref: 0,
    }
  }
}

impl Locals {
  fn resolve(local: &Local) -> u32 {
    if let &Local::I32(idx) = local {
      idx
    } else {
      unreachable!();
    }
  }

  fn build(&self) -> Vec<(u32, ValType)> {
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

enum InstructionBuilder<'a> {
  Basic(Instruction<'a>),
  Memory(&'a MemoryAddress, fn(u32) -> Instruction<'a>),
  Local(Local, fn(u32) -> Instruction<'a>),
}

impl<'a> InstructionBuilder<'a> {
  fn build(self, memory: &MemoryBuilder) -> Instruction<'a> {
    match self {
      Self::Basic(result) => result,
      Self::Memory(addr, cb) => cb(memory.resolve_addr(addr)),
      Self::Local(idx, cb) => cb(Locals::resolve(&idx)),
    }
  }
}

