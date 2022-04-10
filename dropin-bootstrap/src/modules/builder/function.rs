use wasm_encoder::{Function, Instruction};

pub struct FunctionBuilder<'a> {
  type_id:      u32,
  instructions: Vec<Instruction<'a>>,
}

impl<'a> FunctionBuilder<'a> {
  pub fn new(type_id: u32) -> Self {
    Self{ type_id, instructions: vec![] }
  }

  pub fn type_id(&self) -> u32 { self.type_id }

  pub fn instruction(&mut self, instruction: Instruction<'a>) {
    self.instructions.push(instruction);
  }

  pub fn build(&self) -> Function {
    let mut result = Function::new(vec![]);
    for i in self.instructions.iter() {
      result.instruction(&i);
    }
    result.instruction(&Instruction::End);
    result
  }
}
