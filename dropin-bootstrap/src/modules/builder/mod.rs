use wasm_encoder::{
  CodeSection, Export, ExportSection, FunctionSection, MemorySection,
  MemoryType, Module, TypeSection,
};

use std::collections::VecDeque;

mod function;
pub use function::FunctionBuilder;

mod memory;
pub use memory::{MemoryAddress, MemoryBuilder};

mod wasi;
use wasi::WASI;

pub struct ModuleBuilder<'module> {
  memory:             MemoryBuilder<'module>,
  types:              TypeSection,
  functions_imported: Vec<u32>,
  functions_local:    VecDeque<FunctionBuilder<'module>>,
  wasi:               WASI,
}

impl<'module> Default for ModuleBuilder<'module> {
  fn default() -> Self {
    let mut result = Self{
      memory:             MemoryBuilder::default(),
      types:              TypeSection::new(),
      functions_imported: vec![],
      functions_local:    VecDeque::from([FunctionBuilder::new(0)]),
      wasi:               WASI::default(),
    };
    result.types.function(vec![], vec![]); // _start
    result
  }
}

impl<'module> ModuleBuilder<'module> {
  pub fn get_start(&mut self) -> &mut FunctionBuilder<'module> {
    self.functions_local.get_mut(0).unwrap()
  }

  pub fn memory(&mut self) -> &mut MemoryBuilder<'module> { &mut self.memory }

  pub fn build(self) -> Module {
    let mut module = Module::new();
    self.build_type(&mut module)
      .build_function(&mut module)
      .build_memory(&mut module)
      .build_export(&mut module)
      .build_code(&mut module)
      .build_data(&mut module);
    module
  }

  fn build_type(self, module: &mut Module) -> Self {
    module.section(&self.types);
    self
  }

  fn build_function(self, module: &mut Module) -> Self {
    let mut section = FunctionSection::new();
    for f in self.functions_local.iter() {
      section.function(f.type_id());
    }
    module.section(&section);
    self
  }

  fn build_memory(self, module: &mut Module) -> Self {
    let mut section = MemorySection::new();
    section.memory(MemoryType{
      minimum:  1,
      maximum:  None,
      memory64: false,
    });
    module.section(&section);
    self
  }

  fn build_export(self, module: &mut Module) -> Self {
    let mut section = ExportSection::new();
    section.export("memory", Export::Memory(0));
    section.export(
      "_start", Export::Function(self.functions_imported.len() as u32),
    );
    module.section(&section);
    self
  }

  fn build_code(mut self, module: &mut Module) -> Self {
    let mut section = CodeSection::new();
    while let Some(f) = self.functions_local.pop_front() {
      section.function(&f.build(&self.memory));
    }
    module.section(&section);
    self
  }

  fn build_data(self, module: &mut Module) -> Self {
    if let Some(section) = self.memory.build_data() {
      module.section(&section);
    }
    self
  }
}

