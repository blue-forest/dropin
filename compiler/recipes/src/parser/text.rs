use std::fmt::{self, Formatter};

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

use crate::ir::rich_text_part::RichTextInner;
use crate::ir::{RichText, RichTextPart};

use super::{expression::parse, Table};

impl<'de> Deserialize<'de> for RichText {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct RichTextVisitor;

    impl<'de> Visitor<'de> for RichTextVisitor {
      type Value = RichText;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a drop'in text")
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

    deserializer.deserialize_str(RichTextVisitor)
  }
}
