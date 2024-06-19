use alloc::{
  fmt::{self, Write},
  string::String,
  vec::Vec,
};
use dropin_compiler_common::to_upper_camelcase;
use dropin_compiler_recipes::ir::{ComponentChildInner, ComponentZone};

use crate::{
  gen::expressions::gen_rich_text,
  updated_listeners::{write_notifier_name, UpdatedAndListenersState},
  Stated,
};

use super::{
  expressions::{gen_expressions, gen_getter},
  Sub,
};

pub fn gen_zone<'a, S>(
  output: &mut String,
  component: &str,
  state: &S,
  trace: &[usize],
  zone: &ComponentZone,
) -> fmt::Result
where
  S: Sub<'a>,
{
  write!(output, "Row(children: [")?;
  let updated_listeners = <S as Stated<UpdatedAndListenersState>>::state(state);
  let updated_getters = updated_listeners.get_updated_getters(component);
  for (i, child) in zone.blocks.iter().enumerate() {
    let trace = &[trace, &[i]].concat();
    let updated_listeners = updated_listeners
      .get_listeners(component, trace)
      .map(|listeners| {
        listeners
          .iter()
          .filter(|listener| {
            updated_getters
              .iter()
              .position(|updated| updated.getter.as_ref() == listener.getter)
              .is_some()
          })
          .collect::<Vec<_>>()
      });
    let mut is_listenable = false;
    if let Some(updated_listeners) = updated_listeners {
      if !updated_listeners.is_empty() {
        assert_eq!(updated_listeners.len(), 1, "TODO: add Listenable.merge()");
        is_listenable = true;
        let listener = &updated_listeners[0];
        write!(
          output,
          "ListenableBuilder(\
          listenable:",
        )?;
        write_notifier_name(output, listener.getter)?;
        write!(
          output,
          ", builder: (BuildContext context, Widget? child) => "
        )?;
      }
    }
    if i != 0 {
      write!(output, ",")?;
    }
    match child.component_child_inner.as_ref().unwrap() {
      ComponentChildInner::Text(text) => {
        write!(output, "Text(")?;
        gen_rich_text(
          output,
          component,
          state,
          &[],
          text.content.as_ref().unwrap(),
        )?;
        write!(output, ")")?;
      }
      ComponentChildInner::Input(input) => {
        write!(
          output,
          "SizedBox(width: 250, child: TextFormField(initialValue:"
        )?;
        gen_getter(
          output,
          component,
          state,
          input.on_change.as_ref().unwrap(),
        )?;
        write!(output, ", onChanged: (newText_) {{")?;
        let on_change = input.on_change.as_ref().unwrap();
        gen_getter(output, component, state, on_change)?;
        write!(output, "= newText_;")?;
        write_notifier_name(output, on_change)?;
        write!(output, ".notifyListeners();}}))")?;
      }
      ComponentChildInner::Extern(r#extern) => {
        write!(output, "{}(", to_upper_camelcase(&r#extern.path))?;
        let mut is_first = true;
        for (key, value) in &r#extern.properties.as_ref().unwrap().values {
          if !is_first {
            write!(output, ",")?;
          }
          is_first = false;
          write!(output, "{key}:")?;
          gen_expressions(
            output,
            component,
            state,
            &[key.as_str()],
            false,
            value,
          )?;
        }
        for updated_getter in updated_getters {
          if let Some(updated_by) =
            updated_getter.updated_by.get(r#extern.path.as_str())
          {
            write!(output, ",")?;
            write_notifier_name(output, updated_by)?;
            write!(output, ":")?;
            write_notifier_name(output, &updated_getter.getter)?;
          }
        }
        write!(output, ")")?;
      }
    }
    if is_listenable {
      write!(output, ")")?;
    }
  }
  write!(output, "])")?;
  Ok(())
}
