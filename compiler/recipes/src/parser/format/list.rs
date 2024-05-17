use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatInner, FormatList};

pub(super) fn list<'de, A>(
  _keys: BTreeMap<String, Value>,
  _map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::List(FormatList {
    common: Some(FormatCommon::default()),
  }))
}
