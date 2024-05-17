use std::{
  collections::BTreeMap,
  fmt::{self, Formatter},
};

use serde::{
  de::{self, MapAccess, Visitor},
  Deserialize, Deserializer,
};
use serde_yaml::Value;

use crate::ir::ComponentChild;

use self::text::text;

mod text;

impl<'de> Deserialize<'de> for ComponentChild {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ChildVisitor;

    impl<'de> Visitor<'de> for ChildVisitor {
      type Value = ComponentChild;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a drop'in component child")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut keys = BTreeMap::<String, Value>::new();
        while let Some(key) = map.next_key()? {
          if &key == "type" {
            let r#type = map.next_value::<&str>()?;
            return Ok(ComponentChild::new(match r#type {
              "text" => text(keys, map)?,
              _ => return Err(de::Error::unknown_variant(r#type, &["text"])),
            }));
          } else {
            keys.insert(key, map.next_value()?);
          }
        }
        Err(de::Error::missing_field("type"))
      }
    }

    deserializer.deserialize_map(ChildVisitor)
  }
}
