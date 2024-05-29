use std::collections::BTreeMap;

use serde::de::{self, MapAccess};
use serde_yaml::{from_value, Value};

use crate::ir::{FormatCommon, FormatInner, FormatList};

pub(super) fn list<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  let mut format = None;
  for (key, value) in keys {
    match key.as_str() {
      "format" => {
        if format.is_some() {
          return Err(de::Error::duplicate_field("format"));
        }
        format =
          Some(from_value(value).or_else(|e| Err(de::Error::custom(e)))?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["format"])),
    }
  }
  while let Some(key) = map.next_key()? {
    match key {
      "format" => {
        if format.is_some() {
          return Err(de::Error::duplicate_field("format"));
        }
        format = Some(map.next_value()?);
      }
      _ => return Err(de::Error::unknown_field(&key, &["format"])),
    }
  }
  let format = format.ok_or(de::Error::missing_field("format"))?;
  Ok(FormatInner::List(Box::new(FormatList {
    common: Some(FormatCommon::default()),
    format: Some(Box::new(format)),
  })))
}
