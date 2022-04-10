use wasm_encoder::{Instruction, Module};

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
  _expression: &Expression<'syntax, 'module>,
) {
  // let message = expression.iter().next().unwrap().as_str();
  // let offset = builder.data(message.as_bytes());
  let start = builder.get_start();
  start.instruction(Instruction::I32Const(1)); // fd = stdout
}
