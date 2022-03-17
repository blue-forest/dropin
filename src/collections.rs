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

use std::collections::HashMap;

use crate::functions::Handler;
use crate::types::Format;

#[derive(Debug)]
pub struct Method {
  #[allow(dead_code)]
  argument:  Format,
  #[allow(dead_code)]
  handlers:  Vec<Box<dyn Handler>>,
  #[allow(dead_code)]
  variables: HashMap<String, Format>,
}

impl Method {
  pub fn new(
    argument:  Format,
    variables: HashMap<String, Format>,
    handlers:  Vec<Box<dyn Handler>>,
  ) -> Self {
    Self{ argument, handlers, variables }
  }
}

pub type MethodBody = (HashMap<String, Format>, Vec<Box<dyn Handler>>);
