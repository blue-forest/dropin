use alloc::{
  fmt::{self, Write},
  string::String,
};
use dropin_compiler_recipes::ir::Component;

use crate::{objects_getter::ObjectGetterState, Stage, Stated};

use self::keys::gen_keys;

mod formats;
mod keys;

#[derive(Debug)]
pub struct Gen<'a, S>
where
  S: Stage + Stated<ObjectGetterState<'a>>,
{
  sub: &'a S,
}

impl<'a, S> Gen<'a, S>
where
  S: Stage + Stated<ObjectGetterState<'a>>,
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
      gen_keys(output, ir.variables.as_ref().unwrap())?;
      // self.variables.gen(output)?;
      write!(output, "}}")?;
    }
    Ok(output)
  }
}

impl<'a, S> Stage for Gen<'a, S>
where
  S: Stage + Stated<ObjectGetterState<'a>>,
{
  fn ir(&self) -> &Component {
    self.sub.ir()
  }
}
