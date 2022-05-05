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

use std::path::Path;

use dropin_modules::{print, print_to};

use crate::{Recipe, WasiUnwrap};
use crate::expressions::Expression;
use crate::path::get_recipe;
use crate::utils::escape_char;

mod builder;
use builder::{Local, MemoryAddress, ModuleBuilder, STD};

mod error;
pub use error::CompileError;

struct State<'a> {
  pub print:     Option<PrintState>,
  pub std_:      STD<'a>,
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

pub struct Compiler<'syntax, 'module> {
  module:  Recipe<'syntax, 'module>,
}

impl<'syntax, 'module> Compiler<'syntax, 'module> {
  pub fn new(module: Recipe<'syntax, 'module>) -> Self {
    Self{ module }
  }

  pub fn compile(&self, _path: &Path) -> Result<Module, CompileError> {
    let mut builder = ModuleBuilder::default();
    let mut state = State{
      print:     None,
      std_:      STD::default(),
      addresses: vec![],
      data:      vec![],
    };

    let mut iter = self.module.expression.iter();
    iter.next(); // skip syntax
    let mut function = iter.next().wasi_unwrap().iter();
    let _function_name = function.next().wasi_unwrap().as_str();
    let commands = function.next().wasi_unwrap();

    for command in commands.iter() {
      let command_child = command.iter().next().wasi_unwrap();
      match command.pattern() {
        "metaCommand" => {
          self.meta_command(command_child);
        }
        "localCommand" => {
          self.local_command(
            &mut builder, &mut state, command_child.iter().next().wasi_unwrap(),
          );
        }
        _ => { unreachable!() }
      }
    }
    Ok(builder.build())
  }

  fn meta_command(&self, expression: &Expression) {
    match expression.pattern() {
      "print" => print_to(expression.iter().next().unwrap().as_str(), 2),
      pattern => {
        print_to(&format!("unknown command: {}", pattern), 2);
        unsafe { wasi::proc_exit(1) };
      }
    }
  }

  fn local_command(
    &self,
    builder:    &mut ModuleBuilder<'module>,
    state:      &mut State<'module>,
    expression: &Expression<'_, 'module>,
  ) {
    match expression.pattern() {
      "print" => {
        let message = expression.iter().next().unwrap().as_str();
        let alloc = builder.from_std(&state.std_.alloc);
        let print = builder.from_std(&state.std_.print);
        let data = builder.memory().passive(message.as_bytes()) as u32;
        let start = builder.get_start();
        let ptr = start.add_local(I32);
        start.basic(Instruction::I32Const(message.len() as i32)); // size
        start.basic(Instruction::I32Const(1));                    // align
        start.basic(Instruction::Call(alloc));                    // -> ptr
        start.local(ptr.clone(), |ptr| Instruction::LocalSet(ptr));
        start.local(ptr.clone(), |ptr| Instruction::LocalGet(ptr));
        start.basic(Instruction::I32Const(0));                    // offset
        start.basic(Instruction::I32Const(message.len() as i32)); // size
        start.basic(Instruction::MemoryInit{ mem: 0, data });
        start.local(ptr, |ptr| Instruction::LocalGet(ptr));
        start.basic(Instruction::I32Const(message.len() as i32)); // len
        start.basic(Instruction::Call(print));
      }
      pattern => {
        print_to(&format!("unknown command: {}", pattern), 2);
        unsafe { wasi::proc_exit(1) };
      }
    }
  }

  pub fn get_syntax(&self) -> String {
    let id = self.module.expression.iter().next().wasi_unwrap().as_str();
    get_recipe("syntaxes", id)
  }
}

/*
fn print<'syntax, 'module>(
  builder:    &mut ModuleBuilder<'module>,
  state:      &'module mut State<'module>,
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

  let message = expression.iter().next().wasi_unwrap().as_str();
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
  state.addresses.push(mem.data(state.data.last().wasi_unwrap()));
  let message_addr = state.addresses.get(state.addresses.len()-1).wasi_unwrap();

  let print_state = state.print.as_ref().wasi_unwrap();
  let iovec_base    = state.addresses.get( print_state.iovec_base    )
    .wasi_unwrap();
  let iovec_len     = state.addresses.get( print_state.iovec_len     )
    .wasi_unwrap();
  let new_line      = state.addresses.get( print_state.new_line      )
    .wasi_unwrap();
  let new_line_base = state.addresses.get( print_state.new_line_base )
    .wasi_unwrap();
  let new_line_len  = state.addresses.get( print_state.new_line_len  )
    .wasi_unwrap();

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
*/
