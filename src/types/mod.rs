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

use wasm_ir::Module;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::Compilable;

mod text;
pub use text::Text;

pub trait Type: Compilable + Debug {}

pub const OBJECT  : u8 = 0x01;
pub const INDEX   : u8 = 0x02;
pub const LIST    : u8 = 0x03;
pub const TEXT    : u8 = 0x04;
pub const QUANTITY: u8 = 0x05;
pub const BOOLEANS: u8 = 0x06;

#[derive(Debug)]
pub struct CustomType {
  _id:        String,
  templates: HashMap<String, Format>,
}

impl CustomType {
  pub fn new(id: String) -> Self {
    Self{
      _id: id,
      templates: HashMap::new(),
    }
  }

  pub fn add_template(&mut self, key: String, format: Format) {
    self.templates.insert(key, format);
  }
}

impl Compilable for CustomType {
  fn compile(&self) -> Module { todo!() }
}

impl Type for CustomType {}

#[derive(Debug)]
pub struct Format {
  _type_:   Arc<dyn Type>,
  format:  HashMap<String, Format>,
  // TODO: options: Object,
}

impl Format {
  pub fn new(type_: Arc<dyn Type>) -> Self {
    Self{
      _type_: type_,
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
