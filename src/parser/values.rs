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

use pest::iterators::Pairs;

use std::sync::Arc;

use crate::refs::{Byte, List, Data};
use super::Rule;

pub fn read_value(mut pairs: Pairs<Rule>) -> Arc<dyn Data> {
  let pair = pairs.next().expect("expected value");
  let ref_ = match pair.as_rule() {
    Rule::binary => read_binary(pair.into_inner()),
    Rule::list => read_list(pair.into_inner()),
    _ => { panic!("unknown value: {}", pair.as_str()); }
  };
  ref_
}

pub fn read_binary(mut pairs: Pairs<Rule>) -> Arc<dyn Data> {
  let pair = pairs.next().expect("expected binary value");
  match pair.as_rule() {
    Rule::bits => { todo!() }
    Rule::hexa => {
      let hexa = pair.as_str().as_bytes();
      if hexa.len() > 2 {
        panic!("hexadecimal value overflow");
      }
      let mut byte = hexa_unit(*hexa.get(0).expect("expected hexa unit"));
      if let Some(unit) = hexa.get(1) {
        byte = byte * 16 + hexa_unit(*unit);
      }
      Byte::create(byte)
    }
    _ => { panic!("unknown binary value: {}", pair.as_str()); }
  }
}

pub fn read_list(pairs: Pairs<Rule>) -> Arc<dyn Data> {
  let mut data = Vec::new();
  for pair in pairs {
    data.push(read_value(pair.into_inner()));
  }
  List::create(data)
}

pub fn hexa_unit(unit: u8) -> u8 {
  let result = if unit >= b'a' {
    unit - b'a' + 10
  } else if unit >= b'A' {
    unit - b'A' + 10
  } else if unit >= b'0' {
    unit - b'0'
  } else {
    panic!("unknown hexa unit: {}", char::from(unit));
  };
  if result > 15 {
    panic!("unknown hexa unit: {}", char::from(unit));
  }
  result
}
