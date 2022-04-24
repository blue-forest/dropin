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

use wasm_encoder::{
  CodeSection, EntityType, Export, ExportSection, FunctionSection,
  ImportSection, MemorySection, MemoryType, Module, TypeSection,
};

use std::collections::VecDeque;

use crate::WasiUnwrap;

mod function;
pub use function::FunctionBuilder;

mod import;
use import::FunctionImport;

mod memory;
pub use memory::{MemoryAddress, MemoryBuilder};

mod wasi;
pub use self::wasi::{WASI, WASIFunction};

pub struct ModuleBuilder<'module, 'internals> {
  memory:             MemoryBuilder<'internals>,
  types:              TypeSection,
  functions_imported: Vec<FunctionImport<'internals>>,
  functions_local:    VecDeque<FunctionBuilder<'module, 'internals>>,
}

impl<'module, 'internals> Default for ModuleBuilder<'module, 'internals> {
  fn default() -> Self {
    let mut result = Self{
      memory:             MemoryBuilder::default(),
      types:              TypeSection::new(),
      functions_imported: vec![],
      functions_local:    VecDeque::from([FunctionBuilder::new(0)]),
    };
    result.types.function(vec![], vec![]); // _start
    result
  }
}

impl<'module, 'internals> ModuleBuilder<'module, 'internals> {
  pub fn get_start(&mut self) -> &mut FunctionBuilder<'module, 'internals> {
    self.functions_local.get_mut(0).wasi_unwrap()
  }

  pub fn memory(&mut self) -> &mut MemoryBuilder<'internals> { &mut self.memory }
  
  pub fn from_wasi(&mut self, f: &WASIFunction<'internals>) -> u32 {
    if let Some(id) = f.id {
      return id;
    }
    let type_id = self.types.len();
    self.types.function(f.params.clone(), f.results.clone());
    let result = self.functions_imported.len() as u32;
    self.functions_imported.push(FunctionImport{
      type_id, module: "wasi_unstable", name: f.name,
    });
    result
  }

  pub fn build(self) -> Module {
    let mut module = Module::new();
    self.build_type(&mut module)
      .build_import(&mut module)
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

  fn build_import(self, module: &mut Module) -> Self {
    let mut section = ImportSection::new();
    for f in self.functions_imported.iter() {
      section.import(f.module, f.name, EntityType::Function(f.type_id));
    }
    module.section(&section);
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

