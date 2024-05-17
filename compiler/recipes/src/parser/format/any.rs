use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatInner};

pub(super) fn any<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Any(FormatCommon::default()))
}
