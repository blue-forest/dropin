use std::collections::BTreeMap;

use serde::de::{self, MapAccess};
use serde_yaml::{from_value, Value};

use crate::ir::{ComponentChildInner, ComponentCommon, ComponentText};

pub(super) fn text<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<ComponentChildInner, A::Error>
where
  A: MapAccess<'de>,
{
  let mut content = None;
  let mut classes = None;
  for (key, value) in keys {
    match key.as_str() {
      "content" => {
        if content.is_some() {
          return Err(de::Error::duplicate_field("content"));
        }
        content =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      "classes" => {
        if classes.is_some() {
          return Err(de::Error::duplicate_field("classes"));
        }
        classes =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["classes", "content"])),
    }
  }
  while let Some(key) = map.next_key()? {
    match key {
      "content" => {
        if content.is_some() {
          return Err(de::Error::duplicate_field("content"));
        }
        content = Some(map.next_value()?);
      }
      "classes" => {
        if classes.is_some() {
          return Err(de::Error::duplicate_field("classes"));
        }
        classes = Some(map.next_value()?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["classes", "content"])),
    }
  }
  Ok(ComponentChildInner::Text(ComponentText {
    common: Some(ComponentCommon {
      classes: classes.unwrap_or_default(),
    }),
    content: content.unwrap_or_default(),
  }))
}
