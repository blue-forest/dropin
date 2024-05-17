use std::{
  collections::BTreeMap,
  fmt::{self, Formatter},
};

use serde::{
  de::{self, MapAccess, Visitor},
  Deserialize, Deserializer,
};
use serde_yaml::Value;

use crate::ir::Format;

use self::any::any;
use self::boolean::boolean;
use self::choices::choices;
use self::date::date;
use self::index::index;
use self::list::list;
use self::object::object;
use self::quantity::quantity;
use self::text::text;

mod any;
mod boolean;
mod choices;
mod date;
mod index;
mod list;
mod object;
mod quantity;
mod text;

impl<'de> Deserialize<'de> for Format {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct FormatVisitor;

    impl<'de> Visitor<'de> for FormatVisitor {
      type Value = Format;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a drop'in format")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut keys = BTreeMap::<String, Value>::new();
        while let Some(key) = map.next_key()? {
          if &key == "type" {
            let r#type = map.next_value::<&str>()?;
            return Ok(Format::new(match r#type {
              "any" => any(keys, map)?,
              "boolean" => boolean(keys, map)?,
              "choices" => choices(keys, map)?,
              "date" => date(keys, map)?,
              "index" => index(keys, map)?,
              "list" => list(keys, map)?,
              "object" => object(keys, map)?,
              "quantity" => quantity(keys, map)?,
              "text" => text(keys, map)?,
              _ => {
                return Err(de::Error::unknown_variant(
                  r#type,
                  &[
                    "any", "boolean", "choices", "date", "index", "list",
                    "object", "quantity", "text",
                  ],
                ))
              }
            }));
          } else {
            keys.insert(key, map.next_value()?);
          }
        }
        Err(de::Error::missing_field("type"))
      }
    }

    deserializer.deserialize_map(FormatVisitor)
  }
}
