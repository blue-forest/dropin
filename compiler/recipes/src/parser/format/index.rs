use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatIndex, FormatInner};

pub(super) fn index<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Index(FormatIndex {
    common: Some(FormatCommon::default()),
  }))
}
