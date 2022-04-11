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

use wasm_encoder::{Instruction, MemArg, Module};
use wasm_encoder::ValType::I32;

use crate::expressions::Expression;

mod builder;
use builder::{MemoryAddress, ModuleBuilder, WASI};

mod error;
pub use error::CompileError;

pub fn compile(expression: Expression) -> Result<Module, CompileError> {
  let mut builder = ModuleBuilder::default();
  let mut wasi = WASI::default();
  let mut addresses = Vec::new();
  let child = expression.iter().next().unwrap();
  match child.pattern() {
    "print" => print(&mut builder, &mut wasi, &mut addresses, child),
    pattern => { panic!("unexpected pattern: {}", pattern) }
  }
  Ok(builder.build())
}

fn print<'syntax, 'module, 'memory>(
  builder:    &mut ModuleBuilder<'module, 'memory>,
  wasi:       &mut WASI<'memory>,
  addresses:  &'memory mut Vec<MemoryAddress>,
  expression: &Expression<'syntax, 'module>,
) {
  let fd_write = builder.from_wasi(&wasi.fd_write);
  let mem = builder.memory();
  let message = expression.iter().next().unwrap().as_str();
  addresses.push(mem.data(message.as_bytes()));
  addresses.push(mem.buffer(I32));
  addresses.push(mem.buffer(I32));
  let message_addr = addresses.get(addresses.len()-3).unwrap();
  let iovec_base   = addresses.get(addresses.len()-2).unwrap();
  let iovec_len    = addresses.get(addresses.len()-1).unwrap();

  let start = builder.get_start();
  start.memory(iovec_base,   |addr| Instruction::I32Const(addr as i32));
  start.memory(message_addr, |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  start.memory(iovec_len,    |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Const(message.len() as i32));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  /**/start.basic(Instruction::I32Const(0)); // errno -> trash to debug
  start.basic(Instruction::I32Const(1)); // fd = stdout
  start.memory(iovec_base,   |addr|      // iovec
    Instruction::I32Const(addr as i32)
  );
  start.basic(Instruction::I32Const(1)); // len
  start.basic(Instruction::I32Const(0)); // size = trash
  start.basic(Instruction::Call(fd_write));
  /**/start.basic(Instruction::I32Store(MemArg{ // errno -> trash to debug
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
}
