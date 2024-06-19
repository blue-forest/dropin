use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::Model;

use crate::{
  imports::ImportsState,
  objects_getter::ObjectGetterState,
  updated_listeners::{write_notifier_name, UpdatedAndListenersState},
  Stated, EXTENSION,
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
  + Stated<UpdatedAndListenersState<'a>>
  + Stated<ImportsState<'a>>
{
}

impl<'a, S> Sub<'a> for S where
  S: Stated<ObjectGetterState<'a>>
    + Stated<UpdatedAndListenersState<'a>>
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

        let updated_listeners =
          <S as Stated<UpdatedAndListenersState>>::state(&self.sub);
        let updated_getters =
          updated_listeners.get_updated_getters(&component.id);
        for updated_getter in updated_getters {
          write!(file, "final ChangeNotifier ")?;
          write_notifier_name(file, &updated_getter.getter)?;
          if !updated_getter.is_external {
            write!(file, "= ChangeNotifier()")?;
          }
          write!(file, ";")?;
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
        for updated_getter in updated_getters {
          if updated_getter.is_external {
            write!(file, ",")?;
            write!(file, "required this.")?;
            write_notifier_name(file, &updated_getter.getter)?;
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
