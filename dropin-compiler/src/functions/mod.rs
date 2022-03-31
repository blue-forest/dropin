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

use wasm_ir::{Instruction, LocalBuilder};

use std::fmt::Debug;
use std::sync::Arc;

use crate::refs::{Query, Data};

pub trait Handler: Debug + Send + Sync {
  fn compile(
    &self,
    local_builder: &mut LocalBuilder,
    intructions: &mut Vec<Box<dyn Instruction>>,
  );
}

#[derive(Debug)]
pub struct Set {
  #[allow(dead_code)]
  query: Query,
  #[allow(dead_code)]
  value: Arc<dyn Data>,
}

impl Set {
  pub fn new(query: Query, value: Arc<dyn Data>) -> Self {
    Self{ query, value }
  }
}

impl Handler for Set {
  fn compile(
    &self,
    _local_builder: &mut LocalBuilder,
    intructions: &mut Vec<Box<dyn Instruction>>,
  ) {
    intructions.extend(vec![
    ]);
    todo!()
  }
}
