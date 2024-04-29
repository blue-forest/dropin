/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

#![no_std]

extern crate alloc;

#[global_allocator]
static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;

use alloc::{boxed::Box, string::String};
use dlmalloc::GlobalDlmalloc;
use dropin_compiler_common::ir::Expression;
use dropin_compiler_parser_lib::Table;
#[cfg(debug_assertions)]
use lazy_static::lazy_static;
#[cfg(debug_assertions)]
use wasi::cli::stdout::OutputStream;

#[cfg(debug_assertions)]
lazy_static! {
  static ref STDOUT: OutputStream = wasi::cli::stdout::get_stdout();
}

#[cfg(debug_assertions)]
struct Printer;

#[cfg(debug_assertions)]
impl core::fmt::Write for Printer {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    crate::STDOUT.write(s.as_bytes()).unwrap();
    Ok(())
  }
}

#[no_mangle]
pub fn create_table() -> *mut Table {
  Box::into_raw(Box::new(Table::default()))
}

#[no_mangle]
pub fn parse(
  input: String,
  main_non_terminal: Option<String>,
  table: *mut Table,
) -> *mut Expression {
  let table = unsafe { Box::from_raw(table) };
  let expr = dropin_compiler_parser_lib::parse(
    #[cfg(debug_assertions)]
    &mut Printer,
    input,
    main_non_terminal,
    table.as_ref(),
  );
  Box::into_raw(Box::new(expr))
}
