use alloc::{collections::BTreeMap, vec::Vec};
use dropin_compiler_recipes::ir::FormatObject;

use crate::{visit::FormatTrace, Visit};

#[derive(Debug)]
pub struct ObjectGetterState<'a>(
  BTreeMap<&'a str, BTreeMap<Vec<&'a str>, &'a FormatObject>>,
);

#[derive(Default)]
pub struct ObjectGetter<'a> {
  objects: BTreeMap<&'a str, BTreeMap<Vec<&'a str>, &'a FormatObject>>,
}

impl<'a, 'b> Visit<'a, ObjectGetterState<'a>> for ObjectGetter<'a> {
  fn build(self) -> ObjectGetterState<'a> {
    ObjectGetterState(self.objects)
  }

  fn visit_format_object(
    &mut self,
    format: &'a FormatObject,
    trace: &FormatTrace<'a>,
  ) {
    self
      .objects
      .entry(trace.component)
      .or_insert(BTreeMap::new())
      .insert(trace.keys.clone(), format);
  }
}
