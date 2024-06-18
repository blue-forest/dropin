use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::Model;

use crate::{
  imports::ImportsState, objects_getter::ObjectGetterState,
  setters_listeners::SettersAndListenersState, Stated, EXTENSION,
};

use self::{
  classes::gen_classes,
  expressions::gen_expressions,
  keys::{gen_keys, is_undefined},
  zones::gen_zone,
};

mod classes;
mod expressions;
mod formats;
mod keys;
mod zones;

pub trait Sub<'a>:
  Stated<ObjectGetterState<'a>>
  + Stated<SettersAndListenersState<'a>>
  + Stated<ImportsState<'a>>
{
}

impl<'a, S> Sub<'a> for S where
  S: Stated<ObjectGetterState<'a>>
    + Stated<SettersAndListenersState<'a>>
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

  pub fn gen(
    self,
    ir: &'a Model,
  ) -> Result<BTreeMap<String, String>, fmt::Error> {
    let mut files = BTreeMap::new();
    for component in &ir.components {
      let mut file = String::new();
      let term = component.term.as_str();
      let id = component.id.as_str();
      {
        let file = &mut file;
        for import in <S as Stated<ImportsState>>::state(self.sub)
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
            false,
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
            if let Some(default) = default {
              if !is_undefined(default) {
                write!(file, "=")?;
                gen_expressions(file, id, self.sub, &[], false, default)?;
              }
            }
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
