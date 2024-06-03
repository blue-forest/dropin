use std::collections::BTreeMap;

use serde::de::{self, MapAccess};
use serde_yaml::{from_value, Value};

use crate::{
  ir::{FormatCommon, FormatInner, FormatObject},
  parser::keys::IndexMap,
};

pub(super) fn object<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  let mut object_keys: Option<IndexMap> = None;
  let mut object_required = None;
  for (key, value) in keys {
    match key.as_str() {
      "keys" => {
        if object_keys.is_some() {
          return Err(de::Error::duplicate_field("keys"));
        }
        object_keys =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      "required" => {
        if object_required.is_some() {
          return Err(de::Error::duplicate_field("required"));
        }
        object_required =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["keys"])),
    }
  }
  while let Some(key) = map.next_key()? {
    match key {
      "keys" => {
        if object_keys.is_some() {
          return Err(de::Error::duplicate_field("keys"));
        }
        object_keys = Some(map.next_value()?);
      }
      "required" => {
        if object_required.is_some() {
          return Err(de::Error::duplicate_field("required"));
        }
        object_required = Some(map.next_value()?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["keys"])),
    }
  }
  Ok(FormatInner::Object(FormatObject {
    common: Some(FormatCommon::default()),
    required: object_required.unwrap_or_default(),
    keys: object_keys
      .ok_or(de::Error::missing_field("keys"))?
      .into_vec(),
  }))
}
