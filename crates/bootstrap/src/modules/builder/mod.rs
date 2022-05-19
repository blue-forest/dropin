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
	CodeSection, EntityType, Export, ExportSection, Function, FunctionSection,
	ImportSection, MemoryType, Module, TypeSection, ValType,
};

use std::collections::VecDeque;

mod function;
pub use function::{FunctionBuilder, Local, Locals};

mod import;
use import::FunctionImport;

mod memory;
pub use memory::MemoryBuilder;

mod dropin_core;
pub use self::dropin_core::{Core, CoreFunction};

pub struct ModuleBuilder<'module> {
	memory: MemoryBuilder<'module>,
	types: TypeSection,
	functions_imported: Vec<FunctionImport<'module>>,
	functions_local: VecDeque<FunctionBuilder<'module>>,
}

impl<'module> Default for ModuleBuilder<'module> {
	fn default() -> Self {
		Self {
			memory: MemoryBuilder::default(),
			types: TypeSection::new(),
			functions_imported: vec![],
			functions_local: VecDeque::default(),
		}
	}
}

impl<'module> ModuleBuilder<'module> {
	pub fn type_(
		&mut self,
		params: Vec<ValType>,
		results: Vec<ValType>,
	) -> u32 {
		let result = self.types.len() as u32;
		self.types.function(params, results);
		result
	}

	pub fn build(mut self) -> Module {
		let mut module = Module::new();
		let mut function_id = self.functions_imported.len() as u32;
		let mut types: Vec<u32> = vec![];
		let mut functions: Vec<Function> = vec![];
		let mut exports: Vec<(&str, u32)> = vec![];
		while let Some(f) = self.functions_local.pop_front() {
			types.push(f.type_id());
			let (function, export) = f.build();
			functions.push(function);
			if let Some(name) = export {
				exports.push((name, function_id));
			}
			function_id += 1;
		}
		self.build_type(&mut module)
			.build_import(&mut module)
			.build_function(&mut module, types)
			.build_export(&mut module, exports)
			.build_data_count(&mut module)
			.build_code(&mut module, functions)
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
		section.import(
			"blueforest:dropin-core:v1",
			"memory",
			MemoryType {
				minimum: 1,
				maximum: None,
				memory64: false,
			},
		);
		module.section(&section);
		self
	}

	fn build_function(self, module: &mut Module, types: Vec<u32>) -> Self {
		let mut section = FunctionSection::new();
		for t in types {
			section.function(t);
		}
		module.section(&section);
		self
	}

	fn build_export(
		self,
		module: &mut Module,
		exports: Vec<(&str, u32)>,
	) -> Self {
		let mut section = ExportSection::new();
		for (name, id) in exports {
			section.export(name, Export::Function(id));
		}
		module.section(&section);
		self
	}

	fn build_code(self, module: &mut Module, functions: Vec<Function>) -> Self {
		let mut section = CodeSection::new();
		for f in functions {
			section.function(&f);
		}
		module.section(&section);
		self
	}

	fn build_data_count(self, module: &mut Module) -> Self {
		if let Some(section) = self.memory.build_data_count() {
			module.section(&section);
		}
		self
	}

	fn build_data(self, module: &mut Module) -> Self {
		if let Some(section) = self.memory.build_data() {
			module.section(&section);
		}
		self
	}
}
