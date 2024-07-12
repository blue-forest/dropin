use std::collections::BTreeMap;

use serde::de::{self, MapAccess};
use serde_yaml::{from_value, Value};

use crate::ir::{
  ComponentChildInner, ComponentCommon, ComponentExtern, Object,
};

pub(super) fn r#extern<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<ComponentChildInner, A::Error>
where
  A: MapAccess<'de>,
{
  let mut id = None;
  let mut properties = None;
  let mut classes = None;
  for (key, value) in keys {
    match key.as_str() {
      "id" => {
        if id.is_some() {
          return Err(de::Error::duplicate_field("id"));
        }
        id = Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      "properties" => {
        if properties.is_some() {
          return Err(de::Error::duplicate_field("properties"));
        }
        properties =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      "classes" => {
        if classes.is_some() {
          return Err(de::Error::duplicate_field("classes"));
        }
        classes =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      _ => {
        return Err(de::Error::unknown_field(
          &key,
          &["classes", "id", "properties"],
        ))
      }
    }
  }
  while let Some(key) = map.next_key()? {
    match key {
      "id" => {
        if id.is_some() {
          return Err(de::Error::duplicate_field("id"));
        }
        id = Some(map.next_value()?);
      }
      "properties" => {
        if properties.is_some() {
          return Err(de::Error::duplicate_field("properties"));
        }
        properties = Some(map.next_value()?);
      }
      "classes" => {
        if classes.is_some() {
          return Err(de::Error::duplicate_field("classes"));
        }
        classes = Some(map.next_value()?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["classes", "id"])),
    }
  }
  let id = id.ok_or(de::Error::missing_field("id"))?;
  Ok(ComponentChildInner::Extern(ComponentExtern {
    common: Some(ComponentCommon {
      classes: classes.unwrap_or_default(),
    }),
    id,
    properties: Some(Object {
      values: properties.unwrap_or_default(),
    }),
  }))
}
