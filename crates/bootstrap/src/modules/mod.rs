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
use crate::utils::escape_char;

mod builder;
use builder::{MemoryAddress, ModuleBuilder, WASI};

mod error;
pub use error::CompileError;

struct State<'memory> {
  pub print:     Option<PrintState>,
  pub wasi:      WASI<'memory>,
  pub addresses: Vec<MemoryAddress>,
  pub data:      Vec<Vec<u8>>,
}

struct PrintState {
  iovec_base:    usize,
  iovec_len:     usize,
  new_line_base: usize,
  new_line_len:  usize,
  new_line:      usize,
}

pub fn compile(expression: Expression) -> Result<Module, CompileError> {
  let mut builder = ModuleBuilder::default();
  let mut state = State{
    print:     None,
    wasi:      WASI::default(),
    addresses: vec![],
    data:      vec![],
  };
  let state_ptr = &mut state;
  let child = expression.iter().next().unwrap();
  match child.pattern() {
    "print" => print(&mut builder, state_ptr, child),
    pattern => { panic!("unexpected pattern: {}", pattern) }
  }
  Ok(builder.build())
}

fn print<'syntax, 'module, 'memory>(
  builder:    &mut ModuleBuilder<'module, 'memory>,
  state:      &'memory mut State<'memory>,
  expression: &Expression<'syntax, 'module>,
) {
  let mem = builder.memory();
  if state.print.is_none() {
    state.addresses.push(mem.data(&[10]));
    state.addresses.push(mem.buffer(I32));
    state.addresses.push(mem.buffer(I32));
    state.addresses.push(mem.buffer(I32));
    state.addresses.push(mem.buffer(I32));
    state.print = Some(PrintState{
      new_line:      state.addresses.len()-5,
      iovec_base:    state.addresses.len()-4,
      iovec_len:     state.addresses.len()-3,
      new_line_base: state.addresses.len()-2,
      new_line_len:  state.addresses.len()-1,
    });
  }

  let message = expression.iter().next().unwrap().as_str();
  let mut message_parsed = String::with_capacity(message.len());
  let mut is_escaped = false;
  for c in message.chars() {
    if !is_escaped && c == '\\' {
      is_escaped = true;
      continue;
    }
    message_parsed.push(if is_escaped { escape_char(c) } else { c });
    is_escaped = false;
  }

  state.data.push(message_parsed.as_bytes().to_vec());
  state.addresses.push(mem.data(state.data.last().unwrap()));
  let message_addr = state.addresses.get(state.addresses.len()-1).unwrap();

  let print_state = state.print.as_ref().unwrap();
  let iovec_base    = state.addresses.get( print_state.iovec_base    ).unwrap();
  let iovec_len     = state.addresses.get( print_state.iovec_len     ).unwrap();
  let new_line      = state.addresses.get( print_state.new_line      ).unwrap();
  let new_line_base = state.addresses.get( print_state.new_line_base ).unwrap();
  let new_line_len  = state.addresses.get( print_state.new_line_len  ).unwrap();

  let fd_write = builder.from_wasi(&state.wasi.fd_write);
  let start = builder.get_start();

  start.memory(iovec_base,    |addr| Instruction::I32Const(addr as i32));
  start.memory(message_addr,  |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  start.memory(iovec_len,     |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Const(message.len() as i32));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  start.memory(new_line_base, |addr| Instruction::I32Const(addr as i32));
  start.memory(new_line,      |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  start.memory(new_line_len,  |addr| Instruction::I32Const(addr as i32));
  start.basic(Instruction::I32Const(1));
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  /**/start.basic(Instruction::I32Const(0)); // errno -> trash to debug
  start.basic(Instruction::I32Const(1));            // fd = stdout
  start.memory(iovec_base,    |addr|                // iovec
    Instruction::I32Const(addr as i32)
  );
  start.basic(Instruction::I32Const(2));            // len
  start.basic(Instruction::I32Const(0));            // size = trash
  start.basic(Instruction::Call(fd_write));
  /**/start.basic(Instruction::I32Store(MemArg{ // errno -> trash to debug
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
}
