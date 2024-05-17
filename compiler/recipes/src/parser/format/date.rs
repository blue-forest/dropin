use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatDate, FormatInner};

pub(super) fn date<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Date(FormatDate {
    common: Some(FormatCommon::default()),
  }))
}
