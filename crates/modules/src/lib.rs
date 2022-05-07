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

#![no_std]

extern crate wee_alloc;

use core::alloc::{GlobalAlloc, Layout};

const NEW_LINE: &str = "\n";

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub fn print(message: &str) {
  print_to(message, 1)
}

pub fn print_to(message: &str, fd: u32) {
  let data = [
    wasi::Ciovec {
      buf: message.as_ptr(),
      buf_len: message.len(),
    },
    wasi::Ciovec {
      buf: NEW_LINE.as_ptr(),
      buf_len: NEW_LINE.len(),
    },
  ];
  unsafe { wasi::fd_write(fd, &data).unwrap() };
}

#[no_mangle]
pub fn alloc(size: usize, align: usize) -> *mut u8 {
  unsafe { ALLOC.alloc(Layout::from_size_align(size, align).unwrap()) }
}
