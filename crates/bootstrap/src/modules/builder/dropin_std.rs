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

use wasm_encoder::ValType::{self, I32};

use super::ModuleBuilder;
use super::import::FunctionImport;

pub struct Std<'a> {
  pub print: Stdfunction<'a>,
  pub alloc: Stdfunction<'a>,
}

impl<'a> Default for Std<'a> {
  fn default() -> Self {
    Self{
      print: Stdfunction::new(
        "print", vec![I32, I32], vec![],
      ),
      alloc: Stdfunction::new(
        "alloc", vec![I32, I32], vec![I32],
      ),
    }
  }
}

impl<'module> ModuleBuilder<'module> {
  pub fn get_std(&mut self, f: &Stdfunction<'module>) -> u32 {
    if let Some(id) = f.id {
      return id;
    }
    let type_id = self.types.len();
    self.types.function(f.params.clone(), f.results.clone());
    let result = self.functions_imported.len() as u32;
    self.functions_imported.push(FunctionImport{
      type_id, module: "blueforest:dropin-std:v1", name: f.name,
    });
    result
  }
}

pub struct Stdfunction<'a> {
  pub id:      Option<u32>,
  pub name:    &'a str,
  pub params:  Vec<ValType>,
  pub results: Vec<ValType>,
}

impl<'a> Stdfunction<'a> {
  fn new(name: &'a str, params: Vec<ValType>, results: Vec<ValType>) -> Self {
    Self{ id: None, name, params, results }
  }
}
