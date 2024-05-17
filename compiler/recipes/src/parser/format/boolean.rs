use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatBoolean, FormatCommon, FormatInner};

pub(super) fn boolean<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Boolean(FormatBoolean {
    common: Some(FormatCommon::default()),
  }))
}
