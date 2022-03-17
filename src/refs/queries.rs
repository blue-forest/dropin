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

#[derive(Debug)]
pub struct Query(String);

impl Query {
  pub fn new(query: String) -> Self {
    Self(query)
  }
}

/* TODO: query creates iterator
impl Iterator for Query {
  type Item = Result<&str, Issue>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() {
      None
    } else if self.0.starts_with("'") {
      match self.0.get(1..) {
        Some(trimmed) => {
          match trimmed.find("'") {
            Some(close_index) => {
              if close_index+2 < self.0.len() {
                let (key, rest) = self.0.split_at(close_index+2);
                self.0 = rest;
                Some(Ok(key.trim_end_matches(".")))
              } else {
                Some(Ok(""))
              }
            },
            None => Some(Err(Issue{})),
          }
        },
        None => Some(Err(Issue{})),
      }
    } else {
      let (key, rest) = match self.0.split_once(".") {
        Some((key, rest)) => (key, rest),
        None => (self.0, self.0.get(self.0.len()..).unwrap()),
      };
      self.0 = rest;
      Some(Ok(key))
    }
  }
}
*/
