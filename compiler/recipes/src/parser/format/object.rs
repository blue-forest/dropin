use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatInner, FormatObject};

pub(super) fn object<'de, A>(
  keys: BTreeMap<String, Value>,
  mut map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Object(FormatObject {
    common: Some(FormatCommon::default()),
  }))
}
