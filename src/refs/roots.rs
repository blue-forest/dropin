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

use std::collections::HashMap;

use crate::types::Format;
use super::Data;

pub struct Root {
  #[allow(dead_code)]
  nodes: HashMap<String, Ref>
}

impl Root {
  pub fn new(nodes: HashMap<String, Ref>) -> Self {
    Self{ nodes }
  }
}

pub struct Ref {
  #[allow(dead_code)]
  data:   Box<dyn Data>,
  #[allow(dead_code)]
  format: Format,
}

impl Ref {
  pub fn new(data: Box<dyn Data>, format: Format) -> Self {
    // TODO: format.validate(data)
    Self{ data, format }
  }
}
