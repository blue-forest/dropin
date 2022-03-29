/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation under version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use wasm_ir::{Local, LocalBuilder, Module};

use std::collections::HashMap;
use std::sync::Arc;

use crate::Recipe;
use crate::functions::Handler;
use crate::types::Format;

#[derive(Debug)]
pub struct Method {
  #[allow(dead_code)]
  handlers:  Vec<Box<dyn Handler>>,
  #[allow(dead_code)]
  variables: HashMap<String, Format>,
}

impl Method {
  pub fn new(
    variables: HashMap<String, Format>,
    handlers:  Vec<Box<dyn Handler>>,
  ) -> Self {
    Self{ handlers, variables }
  }

  pub fn compile(&self, _recipe: &dyn Recipe, module: &mut Module) {
    // TODO: add variables to root
    let mut instructions = Vec::new();
    let mut local_builder = LocalBuilder::new();
    for handler in self.handlers.iter() {
      handler.compile(&mut local_builder, &mut instructions)
    }
    todo!()
  }
}

#[inline(always)]
pub fn param_self() -> Arc<Local> {
  Local::with_param(0)
}

#[inline(always)]
pub fn param_head() -> Arc<Local> {
  Local::with_param(1)
}

#[inline(always)]
pub fn param_argument() -> Arc<Local> {
  Local::with_param(2)
}
