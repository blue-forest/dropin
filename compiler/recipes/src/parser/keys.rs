use std::fmt::{self, Formatter};

use serde::{
  de::{self, MapAccess, Visitor},
  Deserialize, Deserializer,
};

use crate::ir::{KeyFormat, Keys};

impl<'de> Deserialize<'de> for Keys {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Required,
      Keys,
    }

    struct KeysVisitor;

    impl<'de> Visitor<'de> for KeysVisitor {
      type Value = Keys;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("drop'in keys")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut required = None;
        let mut keys = None;
        while let Some(key) = map.next_key()? {
          match key {
            Field::Required => {
              if required.is_some() {
                return Err(de::Error::duplicate_field("required"));
              }
              required = Some(map.next_value()?)
            }
            Field::Keys => {
              if keys.is_some() {
                return Err(de::Error::duplicate_field("keys"));
              }
              keys = Some(map.next_value::<IndexMap>()?.into_vec())
            }
          }
        }
        Ok(Keys::new(
          required.unwrap_or_default(),
          keys.unwrap_or_default(),
        ))
      }
    }

    const FIELDS: &[&str] = &["required", "keys"];
    deserializer.deserialize_struct("Keys", FIELDS, KeysVisitor)
  }
}

#[derive(Default)]
struct IndexMap {
  value: Vec<KeyFormat>,
}

impl<'de> Deserialize<'de> for IndexMap {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct KeysVisitor;

    impl<'de> Visitor<'de> for KeysVisitor {
      type Value = IndexMap;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a key-format pair")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut result = IndexMap::default();
        while let Some((key, format)) = map.next_entry()? {
          result.value.push(KeyFormat { key, format });
        }
        Ok(result)
      }
    }

    deserializer.deserialize_map(KeysVisitor)
  }
}

impl IndexMap {
  fn into_vec(self) -> Vec<KeyFormat> {
    self.value
  }
}
