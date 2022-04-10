use wasm_encoder::{Instruction, MemArg, Module};
use wasm_encoder::ValType::I32;

use crate::expressions::Expression;

mod builder;
use builder::ModuleBuilder;

mod error;
pub use error::CompileError;

pub fn compile(expression: Expression) -> Result<Module, CompileError> {
  let mut builder = ModuleBuilder::default();
  let child = expression.iter().next().unwrap();
  match child.pattern() {
    "print" => print(&mut builder, child),
    pattern => { panic!("unexpected pattern: {}", pattern) }
  }
  Ok(builder.build())
}

fn print<'syntax, 'module>(
  builder:    &mut ModuleBuilder<'module>,
  expression: &Expression<'syntax, 'module>,
) {
  let mem = builder.memory();
  let message = expression.iter().next().unwrap().as_str();
  let message_addr = mem.data(message.as_bytes());
  let iovec_base = mem.buffer(I32);

  let start = builder.get_start();
  start.memory(iovec_base, |addr| { Instruction::I32Const(addr as i32) });
  start.memory(message_addr, |addr| { Instruction::I32Const(addr as i32) });
  start.basic(Instruction::I32Store(MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }));
  start.basic(Instruction::I32Const(1)); // fd = stdout
  start.basic(Instruction::Drop);
}
