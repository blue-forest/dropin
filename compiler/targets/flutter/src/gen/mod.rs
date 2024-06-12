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
      let name = component.name.as_str();
      {
        let file = &mut file;
        for import in <S as Stated<ImportsState>>::state(self.sub)
          .imports
          .get(name)
          .unwrap_or(&Vec::new())
        {
          write!(file, "import '{import}';")?;
        }
        write!(file, "class {} extends StatelessWidget {{", name)?;
        if let Some(properties) = &component.properties {
          gen_keys(
            file,
            name,
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
            name,
            self.sub,
            &[],
            true,
            &variables.required,
            &variables.keys,
          )?;
        }
        write!(file, "{}({{", ir.name,)?;
        if let Some(properties) = &component.properties {
          let mut is_first = true;
          for key_format in &properties.keys {
            if !is_first {
              write!(file, ",")?;
            }
            is_first = false;
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
        gen_zone(file, name, self.sub, &[], component.zone.as_ref().unwrap())?;
        write!(file, ";}}}}")?;
        gen_classes(file, name, self.sub)?;
      }
      files.insert(name.into(), file);
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
