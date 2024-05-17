#![no_std]

extern crate alloc;

#[global_allocator]
static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;

use alloc::{boxed::Box, ffi::CString, fmt::Write, string::String};
use anyhow::Result;
use dlmalloc::GlobalDlmalloc;
use dropin_compiler_recipes::ir::{
  Arithmetic, Comparison, Component, Control, Expression, ExpressionInner,
  Format, FormatInner, KeyFormat, Keys, Logic, RichTextInner, Value,
  ValueInner,
};
#[cfg(debug_assertions)]
use lazy_static::lazy_static;
use prost::Message;
#[cfg(debug_assertions)]
use wasi::cli::stdout::OutputStream;

#[cfg(debug_assertions)]
lazy_static! {
  static ref STDOUT: OutputStream = wasi::cli::stdout::get_stdout();
}

#[cfg(debug_assertions)]
struct Printer;

#[cfg(debug_assertions)]
impl core::fmt::Write for Printer {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    crate::STDOUT.write(s.as_bytes()).unwrap();
    Ok(())
  }
}

#[no_mangle]
pub fn codegen(protobuf: *mut [u8]) -> CString {
  let protobuf = unsafe { Box::from_raw(protobuf) };
  let component = Component::decode(protobuf.as_ref()).unwrap();
  let mut buf = String::new();
  write!(&mut buf, "(component\n").unwrap();
  let variables = component.variables.unwrap();
  if !variables.keys.is_empty() {
    write!(&mut buf, " variables\n").unwrap();
    gen_keys(&mut buf, 2, variables).unwrap();
  }
  write!(&mut buf, ")").unwrap();
  CString::new(buf).unwrap()
}

fn gen_keys(buf: &mut String, indent: usize, mut keys: Keys) -> Result<()> {
  if !keys.required.is_empty() {
    for KeyFormat { key, format } in keys.keys {
      write!(buf, "{:>1$}let {key}: ", "", indent)?;
      gen_format(buf, indent, format.unwrap())?;
      if let Some(default) = keys.required.remove(&key) {
        write!(buf, " = ")?;
        gen_expression(buf, indent, default)?;
      } else {
        write!(buf, " = null")?;
      }
      write!(buf, "\n")?;
    }
  }
  Ok(())
}

fn gen_format(buf: &mut String, _indent: usize, format: Format) -> Result<()> {
  let format = format.format_inner.unwrap();
  match format {
    FormatInner::Any(_) => write!(buf, "any"),
    FormatInner::Boolean(_) => write!(buf, "boolean"),
    FormatInner::Choices(_) => write!(buf, "choices"),
    FormatInner::Date(_) => write!(buf, "date"),
    FormatInner::Index(_) => write!(buf, "index"),
    FormatInner::List(_) => write!(buf, "list"),
    FormatInner::Object(_) => write!(buf, "object"),
    FormatInner::Quantity(_) => write!(buf, "quantity"),
    FormatInner::Text(_) => write!(buf, "text"),
  }?;
  Ok(())
}

fn gen_expression(
  buf: &mut String,
  indent: usize,
  expression: Expression,
) -> Result<()> {
  let expression = expression.expression_inner.unwrap();
  match expression {
    ExpressionInner::Value(value) => gen_expression_value(buf, indent, value),
    ExpressionInner::Comparison(comparison) => {
      gen_expression_comparison(buf, indent, *comparison)
    }
    ExpressionInner::Logic(logic) => gen_expression_logic(buf, indent, *logic),
    ExpressionInner::Control(control) => {
      gen_expression_control(buf, indent, *control)
    }
    ExpressionInner::Arithmetic(arithmetic) => {
      gen_expression_arithmetic(buf, indent, *arithmetic)
    }
  }?;
  Ok(())
}

fn gen_expression_value(
  buf: &mut String,
  indent: usize,
  value: Value,
) -> Result<()> {
  let value = value.value_inner.unwrap();
  match value {
    ValueInner::Text(value) => {
      write!(buf, "\"")?;
      for part in value.parts {
        let part = part.rich_text_inner.unwrap();
        match part {
          RichTextInner::Static(part) => {
            write!(buf, "{part}")?;
          }
          RichTextInner::Dynamic(expression) => {
            write!(buf, "${{")?;
            gen_expression(buf, indent, expression)?;
            write!(buf, "}}")?;
          }
        }
      }
      write!(buf, "\"")
    }
    ValueInner::Quantity(_) => todo!(),
    ValueInner::Boolean(value) => {
      write!(buf, "{}", if value { "true" } else { "false" })
    }
    ValueInner::Getter(_) => todo!(),
    ValueInner::List(_) => todo!(),
    ValueInner::Object(_) => todo!(),
  }?;
  Ok(())
}

fn gen_expression_comparison(
  _buf: &mut String,
  _indent: usize,
  _comparison: Comparison,
) -> Result<()> {
  Ok(())
}

fn gen_expression_logic(
  _buf: &mut String,
  _indent: usize,
  _logic: Logic,
) -> Result<()> {
  Ok(())
}

fn gen_expression_control(
  _buf: &mut String,
  _indent: usize,
  _control: Control,
) -> Result<()> {
  Ok(())
}

fn gen_expression_arithmetic(
  _buf: &mut String,
  _indent: usize,
  _arithmetic: Arithmetic,
) -> Result<()> {
  Ok(())
}
