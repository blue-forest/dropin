/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use wasm_ir::Module;

use crate::Compilable;
use super::Type;

#[derive(Debug)]
pub struct Text {
}

impl Text {
  pub fn new() -> Self {
    Self{}
  }
}

impl Compilable for Text {
  fn compile(&self) -> Module { todo!() }
}

impl Type for Text {}
