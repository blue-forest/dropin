use alloc::{collections::BTreeSet, vec::Vec};
use dropin_compiler_recipes::ir::{Component, Format, FormatInner, KeyFormat};

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
  pub objects: BTreeSet<Vec<&'a str>>,
}

impl<'a> ObjectGetterState<'a> {
  pub fn new<S>(sub: &'a S) -> Self
  where
    S: Stage,
  {
    let mut objects = BTreeSet::new();

    let ir = sub.ir();
    let mut iters = Vec::new();
    iters.push(FormatStackNode::Keys(
      ir.variables.as_ref().unwrap().keys.iter(),
    ));
    let mut keys = Vec::new();

    while !iters.is_empty() {
      let node = iters.last_mut().unwrap();
      let (key, format): (&str, &Format) = match node {
        FormatStackNode::Keys(iter) => {
          let Some(key) = iter.next() else {
            iters.pop();
            keys.pop();
            continue;
          };
          let format = key.format.as_ref().unwrap();
          (&key.key, format)
        }
        FormatStackNode::Format(format) => ("*", format),
      };
      let format = format.format_inner.as_ref().unwrap();
      match format {
        FormatInner::Index(sub) => {
          iters.push(FormatStackNode::Format(sub.format.as_ref().unwrap()));
          keys.push(key);
        }
        FormatInner::List(sub) => {
          iters.push(FormatStackNode::Format(sub.format.as_ref().unwrap()));
          keys.push(key);
        }
        FormatInner::Object(sub) => {
          iters.push(FormatStackNode::Keys(sub.keys.iter()));
          keys.push(key);
          objects.insert(keys.clone());
        }
        _ => {}
      }
    }

    Self { objects }
  }
}

enum FormatStackNode<'a, I>
where
  I: Iterator<Item = &'a KeyFormat>,
{
  Keys(I),
  Format(&'a Format),
}

/*
use alloc::{
  fmt::{self, Write},
  string::{String, ToString},
  vec::Vec,
};
use anyhow::Result;
use dropin_compiler_recipes::ir::Component;

use crate::keys::GenKeys;

pub struct GenComponent {
  imports: Vec<String>,
  name: String,
  variables: GenKeys,
}

impl From<Component> for GenComponent {
  fn from(value: Component) -> Self {
    let imports = Vec::from(["package:flutter/widgets.dart".to_string()]);
    let name = value.name;
    Self {
      imports,
      name,
      variables: value.variables.unwrap().into(),
    }
  }
}

impl GenComponent {
  pub fn gen(self) -> Result<String, fmt::Error> {
    let mut output = String::new();
    {
      let output = &mut output;
      for import in self.imports {
        write!(output, "import '{import}';")?;
      }
      write!(
        output,
        "class {} extends StatelessWidget {{ final Core _core;",
        self.name
      )?;
      self.variables.gen(output)?;
      write!(output, "}}")?;
    }
    Ok(output)
  }
}
*/
