use alloc::{
  collections::BTreeMap,
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_recipes::ir::Model;

use crate::{
  formats::FormatsState,
  imports::ImportsState,
  objects_getter::ObjectGetterState,
  properties_resolver::PropertiesResolverState,
  updated_listeners::{
    write_notifier_name, write_updater_name, UpdatedAndListenersState,
  },
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
  + Stated<PropertiesResolverState<'a>>
  + Stated<FormatsState<'a>>
{
}

impl<'a, S> Sub<'a> for S where
  S: Stated<ObjectGetterState<'a>>
    + Stated<UpdatedAndListenersState<'a>>
    + Stated<ImportsState<'a>>
    + Stated<PropertiesResolverState<'a>>
    + Stated<FormatsState<'a>>
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
        write!(file, "class {term}_State extends State<{term}> {{")?;
        if let Some(variables) = &component.variables {
          gen_keys(
            file,
            id,
            self.sub,
            &[],
            true,
            false,
            &variables.required,
            &variables.keys,
          )?;
        }

        write!(
          file,
          "{term}_State();\
          @override Widget build(BuildContext context){{ \
          return "
        )?;
        gen_zone(file, id, self.sub, &[], component.zone.as_ref().unwrap())?;
        write!(file, ";}}}}")?;
        write!(file, "class {term} extends StatefulWidget {{")?;

        let updated_listeners =
          <S as Stated<UpdatedAndListenersState>>::state(&self.sub);
        let notifiers = updated_listeners.get_notifiers(&component.id);
        for notifier in &notifiers {
          write!(file, "final ChangeNotifier ")?;
          write_notifier_name(file, &notifier.getter)?;
          if !notifier.is_external {
            write!(file, "= ChangeNotifier()")?;
          } else {
            write!(
              file,
              ";\
              final void Function() "
            )?;
            write_updater_name(file, &notifier.getter)?;
          }
          write!(file, ";")?;
        }

        if let Some(properties) = &component.properties {
          gen_keys(
            file,
            id,
            self.sub,
            &[],
            false,
            true,
            &properties.required,
            &properties.keys,
          )?;
        }
        write!(
          file,
          "@override State<{term}> createState() => {term}_State();\
          {term}({{super.key",
        )?;
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
        for notifier in notifiers {
          if notifier.is_external {
            write!(file, ",")?;
            write!(file, "required this.")?;
            write_notifier_name(file, &notifier.getter)?;
            write!(file, ",")?;
            write!(file, "required this.")?;
            write_updater_name(file, &notifier.getter)?;
          }
        }
        write!(file, "}});}}")?;
        gen_classes(file, id, self.sub)?;
      }
      let mut file_path = String::with_capacity(id.len() + EXTENSION.len());
      write!(&mut file_path, "{id}{EXTENSION}")?;
      files.insert(file_path, file);
    }
    Ok(files)
  }
}
