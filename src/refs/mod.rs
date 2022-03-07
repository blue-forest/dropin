/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| drop'in © 2019-2022 Blue Forest
 *              |_|
 * This code is free software distributed under GPLv3.
 */

use wasm_ir::Compilable;

use std::fmt::Debug;

mod object;
pub use object::Object;

mod text;
pub use text::Text;

pub trait Ref: Compilable + Debug {}

