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

use crate::functions::{Handler, Set};
use crate::refs::Query;

use super::{read_value, Rule};

pub fn read_handlers(pairs: Pairs<Rule>) -> Vec<Box<dyn Handler>> {
  // print_pairs(pairs, 0);
  let mut handlers: Vec<Box<dyn Handler>> = Vec::new();
  for pair in pairs {
    let handler = match pair.as_rule() {
      Rule::set => read_set(pair.into_inner()),
      _ => { panic!("unknown handler: {}", pair.as_str()) }
    };
    handlers.push(Box::new(handler));
  }
  handlers
}

pub fn read_set(mut pairs: Pairs<Rule>) -> Set {
  let query = pairs.next().expect("expected query").as_str();
  let value = read_value(
    pairs.next().expect("expected value").into_inner()
  );
  Set::new(Query::new(query.to_string()), value)
}
