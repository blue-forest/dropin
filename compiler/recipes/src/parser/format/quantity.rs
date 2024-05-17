use std::collections::BTreeMap;

use serde::de::MapAccess;
use serde_yaml::Value;

use crate::ir::{FormatCommon, FormatInner, FormatQuantity};

pub(super) fn quantity<'de, A>(
  _keys: BTreeMap<String, Value>,
  _map: A,
) -> Result<FormatInner, A::Error>
where
  A: MapAccess<'de>,
{
  Ok(FormatInner::Quantity(FormatQuantity {
    common: Some(FormatCommon::default()),
  }))
}
