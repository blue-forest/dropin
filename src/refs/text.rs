/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use wasm_ir::Compilable;

use super::Ref;

#[derive(Debug)]
pub struct Text {}

impl Text {
  pub fn new(_data: String) -> Self {
    todo!()
  }
}

impl Compilable for Text {
  fn compile(&self, _buf: &mut Vec<u8>) { todo!() }
}

impl Ref for Text {}
