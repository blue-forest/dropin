/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

use alloc::string::String;

use crate::lexer::IndentLexer;

pub struct YamlReader<'a> {
  current: &'a [u8],
  indents: IndentLexer,
}

impl<'a> YamlReader<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      current: input.as_bytes(),
      indents: IndentLexer::default(),
    }
  }

  pub fn next_key(&mut self) -> Option<&'a str> {
    for i in 0..self.current.len() {
      if self.current[i] == b':' {
        let key = &self.current[..i];
        self.current = &self.current[i..];
        return Some(unsafe { alloc::str::from_utf8_unchecked(key) });
      }
    }
    None
  }

  pub fn next_list<'b>(&'b mut self) -> YamlList<'a, 'b> {
    let is_inline = loop {
      if self.current.is_empty() {
        panic!("list not found");
      }
      let char = self.current[0];
      self.current = &self.current[1..];
      if char == b'[' {
        break true;
      } else if char == b'-' {
        break false;
      }
    };
    YamlList::new(self, is_inline)
  }
}

pub struct YamlList<'a, 'b> {
  reader: &'b mut YamlReader<'a>,
  is_inline: bool,
  has_next: bool,
}

impl<'a, 'b> YamlList<'a, 'b> {
  fn new(reader: &'b mut YamlReader<'a>, is_inline: bool) -> Self {
    Self {
      reader,
      is_inline,
      has_next: true,
    }
  }

  pub fn next_text(&mut self) -> Option<String> {
    if !self.has_next || self.reader.current.is_empty() {
      return None;
    }
    if self.reader.current[0] == b']' {
      self.reader.current = &self.reader.current[1..];
      return None;
    }
    let quoted = if self.reader.current[0] == b'"' {
      self.reader.current = &self.reader.current[1..];
      true
    } else {
      false
    };
    for i in 0..self.reader.current.len() {
      if (quoted && self.reader.current[i] == b'"')
        || self.reader.current[i] == b'\n'
        || (!quoted && self.reader.current[i] == b',')
      {}
    }
    todo!()
  }
}
