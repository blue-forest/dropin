use alloc::collections::BTreeMap;
use dropin_compiler_recipes::ir::{
  Expression, ExpressionInner, Format, FormatInner, Getter, KeyFormat, Keys,
  RichTextInner, Value, ValueInner,
};

use crate::{visit::Visit, Stated};

#[derive(Debug, Default)]
pub struct FormatsState<'a> {
  properties: BTreeMap<&'a str, &'a Keys>,
  variables: BTreeMap<&'a str, &'a Keys>,
}

impl<'a> FormatsState<'a> {
  pub fn format_of(&self, component: &str, getter: &Getter) -> Option<&Format> {
    if let Some(properties) = self.properties.get(component) {
      if let Some(format) = find_in_keys(&properties.keys, &getter.ident) {
        return get_in_format(format, &getter.indexes);
      }
    }
    if let Some(variables) = self.variables.get(component) {
      if let Some(format) = find_in_keys(&variables.keys, &getter.ident) {
        return get_in_format(format, &getter.indexes);
      }
    }
    None
  }
}

impl<'a> Stated<FormatsState<'a>> for FormatsState<'a> {
  fn state(&self) -> &FormatsState<'a> {
    self
  }
}

impl<'a> Visit<'a, FormatsState<'a>> for FormatsState<'a> {
  fn build(self) -> FormatsState<'a> {
    self
  }

  fn visit_component(
    &mut self,
    component: &'a dropin_compiler_recipes::ir::Component,
    _index: usize,
  ) {
    if let Some(properties) = component.properties.as_ref() {
      self.properties.insert(&component.id, properties);
    }
    if let Some(variables) = component.variables.as_ref() {
      self.variables.insert(&component.id, variables);
    }
  }
}

fn get_in_format<'a>(
  format: &'a Format,
  keys: &[Expression],
) -> Option<&'a Format> {
  if keys.len() == 0 {
    return Some(format);
  }
  let key = if let ExpressionInner::Value(Value {
    value_inner: Some(ValueInner::Text(parts)),
  }) = keys[0].expression_inner.as_ref().unwrap()
  {
    if parts.parts.len() != 1 {
      "*"
    } else if let RichTextInner::Static(key) =
      &parts.parts[0].rich_text_inner.as_ref().unwrap()
    {
      key.as_str()
    } else {
      "*"
    }
  } else {
    "*"
  };
  match format.format_inner.as_ref().unwrap() {
    FormatInner::Index(index) => {
      return get_in_format(index.format.as_ref().unwrap(), &keys[1..]);
    }
    FormatInner::List(list) => {
      return get_in_format(list.format.as_ref().unwrap(), &keys[1..]);
    }
    FormatInner::Object(object) => {
      assert_ne!(key, "*", "can't dynamically index objects");
      let format = find_in_keys(&object.keys, key).unwrap();
      if keys.len() == 1 {
        return Some(format);
      }
      return get_in_format(format, &keys[1..]);
    }
    _ => panic!("can't find getter"),
  }
}

fn find_in_keys<'a>(keys: &'a [KeyFormat], key: &str) -> Option<&'a Format> {
  for key_format in keys {
    if key_format.key == key {
      return key_format.format.as_ref();
    }
  }
  None
}
