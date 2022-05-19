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

use std::path::Path;

use crate::error::PortableUnwrap;

mod path;
pub use path::*;

#[cfg(target_family = "wasm")]
pub fn read(path: &Path) -> String {
	unsafe {
		let fd = wasi::path_open(
			3, // preopened fd
			wasi::LOOKUPFLAGS_SYMLINK_FOLLOW,
			path.to_str().punwrap(),
			0,
			wasi::RIGHTS_FD_READ,
			wasi::RIGHTS_FD_READ,
			0,
		)
		.punwrap();
		let mut content = String::new();
		loop {
			let mut buf = [0; 3];
			let size = wasi::fd_read(
				fd,
				&[wasi::Iovec {
					buf: buf.as_mut_ptr(),
					buf_len: buf.len(),
				}],
			)
			.punwrap();
			content
				.push_str(std::str::from_utf8(buf.get(..size).punwrap()).punwrap());
			if size < buf.len() || size == 0 {
				break;
			}
		}
		content
	}
}

#[cfg(not(target_family = "wasm"))]
pub fn read(path: &Path) -> String {
	std::fs::read_to_string(&path).punwrap()
}

#[cfg(target_family = "wasm")]
pub fn write(path: &Path, content: &[u8]) {
	unsafe {
		let fd = wasi::path_open(
			3, // preopened fd
			wasi::LOOKUPFLAGS_SYMLINK_FOLLOW,
			path.to_str().punwrap(),
			wasi::OFLAGS_CREAT | wasi::OFLAGS_TRUNC,
			wasi::RIGHTS_FD_WRITE
				| wasi::RIGHTS_PATH_CREATE_DIRECTORY
				| wasi::RIGHTS_PATH_CREATE_FILE,
			wasi::RIGHTS_FD_WRITE
				| wasi::RIGHTS_PATH_CREATE_DIRECTORY
				| wasi::RIGHTS_PATH_CREATE_FILE,
			0,
		)
		.punwrap();
		let data = [wasi::Ciovec {
			buf: content.as_ptr(),
			buf_len: content.len(),
		}];
		wasi::fd_write(fd, &data).punwrap();
	}
}

#[cfg(not(target_family = "wasm"))]
pub fn write(path: &Path, content: &[u8]) {
	std::fs::write(path, content).punwrap();
}

pub fn read_recipe(
	root: &Path,
	owner: &str,
	model: &str,
	version: &str,
	directory: &str,
	id: &str,
) -> String {
	let mut path = model_path(root, owner, model, version);
	path.push(directory);
	path.push(format!("{}.dropin", id));
	read(&path)
}
