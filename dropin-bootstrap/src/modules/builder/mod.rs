use wasm_encoder::{
  CodeSection, DataSection, ElementSection, Export, ExportSection,
  Function, FunctionSection, GlobalSection, ImportSection, Instruction, MemorySection,
  MemoryType, Module, StartSection, TableSection, TypeSection,
};

mod function;
pub use function::FunctionBuilder;

mod wasi;
use wasi::WASI;

pub struct ModuleBuilder<'module> {
  data:               Vec<&'module [u8]>,
  data_len:           usize,
  types:              TypeSection,
  functions_imported: Vec<u32>,
  functions_local:    Vec<FunctionBuilder<'module>>,
  wasi:               WASI,
}

impl<'module> Default for ModuleBuilder<'module> {
  fn default() -> Self {
    let mut result = Self{
      data:               vec![],
      data_len:           0,
      types:              TypeSection::new(),
      functions_imported: vec![],
      functions_local:    vec![FunctionBuilder::new(0)],
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

  pub fn build(&self) -> Module {
    let mut module = Module::new();
    self.build_type(&mut module);
    self.build_function(&mut module);
    self.build_memory(&mut module);
    self.build_export(&mut module);
    self.build_code(&mut module);
    self.build_data(&mut module);
    module
  }

  fn build_type(&self, module: &mut Module) {
    module.section(&self.types);
  }

  fn build_function(&self, module: &mut Module) {
    let mut section = FunctionSection::new();
    for f in self.functions_local.iter() {
      section.function(f.type_id());
    }
    module.section(&section);
  }

  fn build_memory(&self, module: &mut Module) {
    let mut section = MemorySection::new();
    section.memory(MemoryType{
      minimum:  1,
      maximum:  None,
      memory64: false,
    });
    module.section(&section);
  }

  fn build_export(&self, module: &mut Module) {
    let mut section = ExportSection::new();
    section.export("memory", Export::Memory(0));
    section.export(
      "_start", Export::Function(self.functions_imported.len() as u32),
    );
    module.section(&section);
  }

  fn build_code(&self, module: &mut Module) {
    let mut section = CodeSection::new();
    for f in self.functions_local.iter() {
      section.function(&f.build());
    }
    module.section(&section);
  }

  fn build_data(&self, module: &mut Module) {
    let mut offset = 0;
    if !self.data.is_empty() {
      let mut section = DataSection::new();
      for d in self.data.iter() {
        section.active(
          0, &Instruction::I32Const(offset as i32), d.iter().copied(),
        );
        offset += d.len();
      }
      module.section(&section);
    }
  }

  pub fn data(&mut self, data: &'module [u8]) -> usize {
    self.data_len += data.len();
    self.data.push(data);
    self.data_len
  }
}

