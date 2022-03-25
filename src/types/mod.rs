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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use wasm_ir::{Body, FunctionType, Import, Limit, LocalBuilder, Module};

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::Recipe;
use crate::collections::Method;

lazy_static! {
  pub static ref BYTE:  Arc<Type> = Arc::new(Type::new(
    "byte".to_string(),
    HashMap::new(),
    Methods::new(Method::new(HashMap::new(), Vec::new()))
  ));
  pub static ref BYTES: Arc<Type> = Arc::new(Type::new(
    "bytes".to_string(),
    HashMap::new(),
    Methods::new(Method::new(HashMap::new(), Vec::new())),
  ));
}

#[derive(Debug)]
pub struct Type {
  #[allow(dead_code)]
  id:        String,
  #[allow(dead_code)]
  methods:   Methods,
  #[allow(dead_code)]
  templates: HashMap<String, Format>,
}

impl Type {
  pub fn new(
    id:        String,
    templates: HashMap<String, Format>,
    methods:   Methods,
  ) -> Self {
    Self{ id, methods, templates }
  }

  pub fn compile(self) -> Module {
    let mut module = Module::new();
    module.import_memory(
      Import::new("env".to_string(), "memory".to_string()),
      Limit::new(1, None)
    );

    // TODO: compile templates & options
    module.export_function(
      "_start".to_string(),
      FunctionType::new(Vec::new(), Vec::new()),
      Body::new(LocalBuilder::new(), vec![])
    );

    self.methods.encode.compile(&self, &mut module);
    module
  }
}

impl Recipe for Type {}

#[derive(Debug)]
pub struct Methods {
  #[allow(dead_code)]
  encode: Method,
}

impl Methods {
  pub fn new(encode: Method) -> Self {
    Self{ encode }
  }
}

#[derive(Debug)]
pub struct Format {
  #[allow(dead_code)]
  type_:  Arc<Type>,
  format: HashMap<String, Format>,
  // TODO: options: Object,
}

impl Format {
  pub fn new(type_: Arc<Type>) -> Self {
    Self{
      type_,
      format:  HashMap::new(),
      // options: Object::new(),
    }
  }

  pub fn set_format(&mut self, format: Format) {
    if !self.format.is_empty() {
      panic!("trying to set an existing format");
    }
    self.format.insert("".to_string(), format);
  }

  pub fn add_format(&mut self, key: String, format: Format) {
    if let Some(_) = self.format.insert(key, format) {
      panic!("trying to set an existing key format");
    }
  }
}
