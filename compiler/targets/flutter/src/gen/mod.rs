use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::Model;

use crate::{
  imports::ImportsState, listeners::ListenersState,
  objects_getter::ObjectGetterState, Stage, Stated,
};

use self::{
  classes::gen_classes,
  keys::{gen_keys, is_undefined},
  zones::gen_zone,
};

mod classes;
mod expressions;
mod formats;
mod keys;
mod zones;

const EXTENSION: &str = ".dart";

pub trait Sub<'a>:
  Stage
  + Stated<ObjectGetterState<'a>>
  + Stated<ListenersState<'a>>
  + Stated<ImportsState<'a>>
{
}

impl<'a, S> Sub<'a> for S where
  S: Stage
    + Stated<ObjectGetterState<'a>>
    + Stated<ListenersState<'a>>
    + Stated<ImportsState<'a>>
{
}

#[derive(Debug)]
pub struct Gen<'a, S>
where
  S: Sub<'a>,
{
  sub: &'a S,
}

impl<'a, S> Gen<'a, S>
where
  S: Sub<'a>,
{
  pub fn new(sub: &'a S) -> Self {
    Self { sub }
  }

  pub fn gen(self) -> Result<BTreeMap<String, String>, fmt::Error> {
    let mut files = BTreeMap::new();
    let ir = self.sub.ir();
    for component in &ir.components {
      let mut file = String::new();
      let term = component.term.as_str();
      let id = component.id.as_str();
      {
        let file = &mut file;
        for import in <S as Stated<ImportsState>>::state(self.sub)
          .imports
          .get(id)
          .unwrap_or(&Vec::new())
        {
          write!(file, "import '{import}';")?;
        }
        write!(file, "class {} extends StatelessWidget {{", term)?;
        if let Some(properties) = &component.properties {
          gen_keys(
            file,
            id,
            self.sub,
            &[],
            true,
            &properties.required,
            &properties.keys,
          )?;
        }
        if let Some(variables) = &component.variables {
          gen_keys(
            file,
            id,
            self.sub,
            &[],
            true,
            &variables.required,
            &variables.keys,
          )?;
        }
        write!(file, "{}({{super.key", component.term)?;
        if let Some(properties) = &component.properties {
          for key_format in &properties.keys {
            write!(file, ",")?;
            let default = properties.required.get(&key_format.key);
            if let Some(default) = default {
              if is_undefined(default) {
                write!(file, "required ")?;
              }
            }
            write!(file, "this.{}", key_format.key)?;
          }
        }
        write!(
          file,
          "}});\
          @override Widget build(BuildContext context){{ \
          return "
        )?;
        gen_zone(file, id, self.sub, &[], component.zone.as_ref().unwrap())?;
        write!(file, ";}}}}")?;
        gen_classes(file, id, self.sub)?;
      }
      let mut file_path = String::with_capacity(id.len() + EXTENSION.len());
      write!(&mut file_path, "{id}{EXTENSION}")?;
      files.insert(file_path, file);
    }
    Ok(files)
  }
}

impl<'a, S> Stage for Gen<'a, S>
where
  S: Sub<'a>,
{
  fn ir(&self) -> &Model {
    self.sub.ir()
  }
}
