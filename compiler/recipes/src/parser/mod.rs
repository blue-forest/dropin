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

use std::fmt::{self, Formatter};

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

use crate::ir::rich_text_part::RichTextInner;
use crate::ir::{RichText, RichTextPart};

pub use self::lexer::lexer;
use self::snippet::parse;
pub use self::token::Token;

mod lexer;
mod snippet;
mod token;

#[dropin_compiler_recipes_macros::table(
  grammar = "compiler/recipes/src/parser/grammar.abnf"
)]
pub struct Table;

impl<'de> Deserialize<'de> for RichText {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(SnippetVisitor)
  }
}

struct SnippetVisitor;

impl<'de> Visitor<'de> for SnippetVisitor {
  type Value = RichText;

  fn expecting(&self, f: &mut Formatter) -> fmt::Result {
    f.write_str("a drop'in expression")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let mut parts = Vec::with_capacity(1);
    let mut i = 0;
    while let Some(len) = v[i..].find("${") {
      if len != 0 {
        parts.push(RichTextPart::r#static(v[i..i + len].into()));
      }
      i += len;
      let mut end = i + 2;
      let mut depth = 1;
      let bytes = v.as_bytes();
      loop {
        if end >= bytes.len() {
          panic!("didn't find closing brace");
        }
        if bytes[end] == b'{' {
          depth += 1;
        } else if bytes[end] == b'}' {
          depth -= 1;
          if depth == 0 {
            break;
          }
        }
        end += 1;
      }
      let expr = parse(&v[i + 2..end], None, &Table::default());
      parts.push(RichTextPart::dynamic(expr));
      i = end + 1;
    }
    if i != v.len() {
      parts.push(RichTextPart {
        rich_text_inner: Some(RichTextInner::Static(v[i..].into())),
      });
    }
    Ok(RichText { parts })
  }
}
