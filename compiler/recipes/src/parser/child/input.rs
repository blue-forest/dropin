use std::collections::BTreeMap;

use serde::de::{self, MapAccess};
use serde_yaml::{from_value, Value as YamlValue};

use crate::ir::{
  ComponentChildInner, ComponentCommon, ComponentInput, Expression,
  ExpressionInner, Value, ValueInner,
};

pub(super) fn input<'de, A>(
  keys: BTreeMap<String, YamlValue>,
  mut map: A,
) -> Result<ComponentChildInner, A::Error>
where
  A: MapAccess<'de>,
{
  let mut on_change: Option<Expression> = None;
  let mut classes = None;
  for (key, value) in keys {
    match key.as_str() {
      "on_change" => {
        if on_change.is_some() {
          return Err(de::Error::duplicate_field("on_change"));
        }
        on_change =
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
        return Err(de::Error::unknown_field(&key, &["classes", "on_change"]))
      }
    }
  }
  while let Some(key) = map.next_key()? {
    match key {
      "on_change" => {
        if on_change.is_some() {
          return Err(de::Error::duplicate_field("on_change"));
        }
        on_change = Some(map.next_value()?);
      }
      "classes" => {
        if classes.is_some() {
          return Err(de::Error::duplicate_field("classes"));
        }
        classes = Some(map.next_value()?);
      }
      _ => {
        return Err(de::Error::unknown_field(&key, &["classes", "on_change"]))
      }
    }
  }
  let on_change = on_change.ok_or(de::Error::missing_field("on_change"))?;
  let ExpressionInner::Value(Value {
    value_inner: Some(ValueInner::Getter(on_change)),
  }) = on_change.expression_inner.unwrap()
  else {
    return Err(de::Error::custom("on_change is not a getter"));
  };

  Ok(ComponentChildInner::Input(ComponentInput {
    common: Some(ComponentCommon {
      classes: classes.unwrap_or_default(),
    }),
    on_change: Some(on_change),
  }))
}
