use std::fmt::{self, Formatter};

use serde::{
  de::{self, MapAccess, Visitor},
  Deserialize, Deserializer,
};

use crate::ir::Component;

impl<'de> Deserialize<'de> for Component {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
      Variables,
      Classes,
      Blocks,
    }

    struct ComponentVisitor;

    impl<'de> Visitor<'de> for ComponentVisitor {
      type Value = Component;

      fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a drop'in component")
      }

      fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut variables = None;
        let mut classes = None;
        let mut blocks = None;
        while let Some(key) = map.next_key()? {
          match key {
            Field::Variables => {
              if variables.is_some() {
                return Err(de::Error::duplicate_field("variables"));
              }
              variables = Some(map.next_value()?)
            }
            Field::Classes => {
              if classes.is_some() {
                return Err(de::Error::duplicate_field("classes"));
              }
              classes = Some(map.next_value()?)
            }
            Field::Blocks => {
              if blocks.is_some() {
                return Err(de::Error::duplicate_field("blocks"));
              }
              blocks = Some(map.next_value()?)
            }
          }
        }
        Ok(Component::new(
          variables,
          classes.unwrap_or(vec![]),
          blocks.unwrap_or(vec![]),
        ))
      }
    }

    const FIELDS: &[&str] = &["variables", "classes", "blocks"];
    deserializer.deserialize_struct("Component", FIELDS, ComponentVisitor)
  }
}
