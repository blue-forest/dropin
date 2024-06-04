use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::Component;

use crate::{objects_getter::ObjectGetterState, Stage, Stated};

use self::{classes::gen_classes, keys::gen_keys};

mod classes;
mod expressions;
mod formats;
mod keys;

pub trait Sub<'a>: Stage + Stated<ObjectGetterState<'a>> {}

impl<'a, S> Sub<'a> for S where S: Stage + Stated<ObjectGetterState<'a>> {}

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
      write!(
        output,
        "class {} extends StatelessWidget {{ final Core _core;",
        ir.name
      )?;
      let variables = ir.variables.as_ref().unwrap();
      gen_keys(
        output,
        self.sub,
        &[],
        true,
        &variables.required,
        &variables.keys,
      )?;
      write!(output, "}}")?;
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
