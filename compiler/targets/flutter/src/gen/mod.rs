use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::Component;

use crate::{
  listeners::ListenersState, objects_getter::ObjectGetterState, Stage, Stated,
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
  Stage + Stated<ObjectGetterState<'a>> + Stated<ListenersState<'a>>
{
}

impl<'a, S> Sub<'a> for S where
  S: Stage + Stated<ObjectGetterState<'a>> + Stated<ListenersState<'a>>
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

  pub fn gen(self) -> Result<String, fmt::Error> {
    let mut output = String::new();
    {
      let ir = self.sub.ir();
      let output = &mut output;
      // for import in self.imports {
      //   write!(output, "import '{import}';")?;
      // }
      write!(output, "class {} extends StatelessWidget {{", ir.name)?;
      if let Some(properties) = &ir.properties {
        gen_keys(
          output,
          self.sub,
          &[],
          true,
          &properties.required,
          &properties.keys,
        )?;
      }
      if let Some(variables) = &ir.variables {
        gen_keys(
          output,
          self.sub,
          &[],
          true,
          &variables.required,
          &variables.keys,
        )?;
      }
      write!(output, "{}({{", ir.name,)?;
      if let Some(properties) = &ir.properties {
        let mut is_first = true;
        for key_format in &properties.keys {
          if !is_first {
            write!(output, ",")?;
          }
          is_first = false;
          let default = properties.required.get(&key_format.key);
          if let Some(default) = default {
            if is_undefined(default) {
              write!(output, "required ")?;
            }
          }
          write!(output, "this.{}", key_format.key)?;
        }
      }
      write!(
        output,
        "}});\
        @override Widget build(BuildContext context){{ \
        return "
      )?;
      gen_zone(output, self.sub, &[], ir.zone.as_ref().unwrap())?;
      write!(output, ";}}}}")?;
      gen_classes(output, self.sub)?;
    }
    Ok(output)
  }
}

impl<'a, S> Stage for Gen<'a, S>
where
  S: Sub<'a>,
{
  fn ir(&self) -> &Component {
    self.sub.ir()
  }
}
