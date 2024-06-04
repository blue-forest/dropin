use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::{
  Component, Format, FormatInner, FormatObject, KeyFormat, Keys,
};

use crate::{Stage, Stated};

#[derive(Debug)]
pub struct ObjectGetter<'a, S>
where
  S: Stage,
{
  sub: &'a S,
  state: ObjectGetterState<'a>,
}

impl<'a, S> ObjectGetter<'a, S>
where
  S: Stage + 'a,
{
  pub fn new(sub: &'a S) -> Self {
    let state = ObjectGetterState::new(sub);
    Self { sub, state }
  }
}

impl<'a, S> Stage for ObjectGetter<'a, S>
where
  S: Stage,
{
  fn ir(&self) -> &Component {
    self.sub.ir()
  }
}

impl<'a, S> Stated<ObjectGetterState<'a>> for ObjectGetter<'a, S>
where
  S: Stage,
{
  fn state(&self) -> &ObjectGetterState<'a> {
    &self.state
  }
}

#[derive(Debug)]
pub struct ObjectGetterState<'a> {
  pub objects: BTreeMap<Vec<&'a str>, &'a FormatObject>,
}

impl<'a> ObjectGetterState<'a> {
  pub fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    let mut objects = BTreeMap::new();
    let ir = sub.ir();
    if let Some(variables) = &ir.variables {
      fill_keys(&mut objects, variables);
    }
    if let Some(properties) = &ir.properties {
      fill_keys(&mut objects, properties);
    }

    Self { objects }
  }
}

fn fill_keys<'a>(
  objects: &mut BTreeMap<Vec<&'a str>, &'a FormatObject>,
  keys: &'a Keys,
) {
  let mut nodes = Vec::new();
  nodes.push(FormatStackNode::Keys(keys.keys.iter()));
  let mut keys = Vec::new();

  while !nodes.is_empty() {
    let node = nodes.last_mut().unwrap();
    let (key, format): (&str, &Format) = match node {
      FormatStackNode::Keys(iter) => {
        let Some(key) = iter.next() else {
          nodes.pop();
          keys.pop();
          continue;
        };
        let format = key.format.as_ref().unwrap();
        (&key.key, format)
      }
      FormatStackNode::Format(format) => {
        let Some(format) = format.take() else {
          nodes.pop();
          continue;
        };
        ("*", format)
      }
    };
    let format = format.format_inner.as_ref().unwrap();
    match format {
      FormatInner::Index(sub) => {
        nodes.push(FormatStackNode::Format(Some(sub.format.as_ref().unwrap())));
        keys.push(key);
      }
      FormatInner::List(sub) => {
        nodes.push(FormatStackNode::Format(Some(sub.format.as_ref().unwrap())));
        keys.push(key);
      }
      FormatInner::Object(sub) => {
        nodes.push(FormatStackNode::Keys(sub.keys.iter()));
        keys.push(key);
        objects.insert(keys.clone(), sub);
      }
      _ => {}
    }
  }
}

enum FormatStackNode<'a, I>
where
  I: Iterator<Item = &'a KeyFormat>,
{
  Keys(I),
  Format(Option<&'a Format>),
}

pub fn write_class_name(output: &mut String, trace: &[&str]) -> fmt::Result {
  for key in trace {
    match *key {
      "*" => {
        write!(output, "_")?;
      }
      "_" => write!(output, "__")?,
      _ => {
        let mut is_capital = true;
        for c in key.chars() {
          if c == '_' {
            is_capital = true;
            continue;
          }
          if is_capital {
            output.push(c.to_ascii_uppercase());
          } else {
            output.push(c);
          }
          is_capital = false;
        }
      }
    }
  }
  Ok(())
}
