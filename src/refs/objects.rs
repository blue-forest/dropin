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
use std::sync::Arc;

use super::Ref;

#[derive(Debug)]
pub struct Object{
  data: HashMap<String, Arc<dyn Ref>>,
}

impl Object {
  pub fn new() -> Self {
    Self{
      data: HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: String, value: Arc<dyn Ref>) {
    if let Some(old_value) = self.data.insert(key, value) {
      panic!("overriding object value {:?}", old_value);
    }
  }
}

impl Ref for Object {
}
