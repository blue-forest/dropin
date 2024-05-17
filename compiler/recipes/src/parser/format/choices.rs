use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatChoices, FormatCommon, FormatInner};

pub(super) fn choices<'de, A>(
  _keys: BTreeMap<String, Value>,
  _map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Choices(FormatChoices {
    common: Some(FormatCommon::default()),
  }))
}
