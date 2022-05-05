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

use std::path::{Path, PathBuf};
use std::str;

use crate::{WasiExpect, WasiUnwrap};

pub unsafe fn read_file(path: &Path) -> String {
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
}

pub fn get_model_path(id: &str) -> PathBuf {
  model_path(&mut id.split(':'))
}

fn model_path<'a>(iter: &mut str::Split<'a, char>) -> PathBuf {
  let owner   = iter.next().wasi_expect("expected owner");
  let model   = iter.next().wasi_expect("expected model");
  let version = iter.next().wasi_expect("expected version");
  let mut path = PathBuf::new();
  path.push(owner);
  path.push("models");
  path.push(model);
  path.push(version);
  path
}

pub fn get_recipe(collection: &str, id: &str) -> String {
  let mut split = id.split(':');
  let mut path = model_path(&mut split);
  let mut recipe = split.next().wasi_expect("expected recipe").to_string();
  recipe.push_str(".dropin");
  path.push(collection);
  path.push(recipe);
  unsafe { read_file(&path) }
}
