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

use std::path::PathBuf;
use std::str;

use crate::{WasiExpect, WasiUnwrap};

pub fn get_path(collection: &str, id: &str) -> PathBuf {
  let mut split = id.split(':');
  let owner = split.next().wasi_expect("expected owner");
  let model = split.next().wasi_expect("expected model");
  let version = split.next().wasi_expect("expected version");
  let mut recipe = split.next().wasi_expect("expected recipe").to_string();
  recipe.push_str(".dropin");
  let mut result = PathBuf::new();
  result.push(owner);
  result.push("models");
  result.push(model);
  result.push(version);
  result.push(collection);
  result.push(recipe);
  result
}

pub fn get_recipe(collection: &str, id: &str) -> String {
  let path = get_path(collection, id);
  let content = unsafe {
    let fd = wasi::path_open(
      3, // preopened fd
      wasi::LOOKUPFLAGS_SYMLINK_FOLLOW,
      &path.to_str().wasi_unwrap(),
      0, 1073741823, 1073741823, 0,
    ).wasi_unwrap();
    let mut content = String::new();
    loop {
      let mut buf = [0; 3];
      let size = wasi::fd_read(fd, &[
        wasi::Iovec{
          buf:     buf.as_mut_ptr(),
          buf_len: buf.len(),
        }
      ]).wasi_unwrap();
      content.push_str(
        str::from_utf8(buf.get(..size).wasi_unwrap()).wasi_unwrap()
      );
      if size < buf.len() || size == 0 {
        break
      }
    }
    content
  };
  let header_split = content.find("\n===").wasi_unwrap();
  let start = content
    .get(header_split+4..)
    .wasi_unwrap()
    .find("\n")
    .wasi_unwrap()
    + header_split + 5;
  content.get(start..).wasi_unwrap().to_string()
}
