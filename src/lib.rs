/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use wasm_ir::Module;

pub mod parser;
pub mod refs;
pub mod types;

#[macro_use]
extern crate pest_derive;

pub trait Compilable {
  fn compile(&self) -> Module;
}
