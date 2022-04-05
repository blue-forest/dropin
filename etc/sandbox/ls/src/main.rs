use wasmtime::{self, Engine, Extern, Linker, Store, Memory};
use wasmtime_wasi::sync::{Dir, WasiCtxBuilder};

use wasm_encoder::{
  CodeSection, EntityType, Export, ExportSection, Function, FunctionSection,
  ImportSection, Instruction, MemoryType, Module, TypeSection, ValType,
};

use std::fs::{File, write};
use std::path::Path;

fn main() {
  let mut m = Module::new();

  let mut types = TypeSection::new();
  // let fd_readdir_type = 0;
  types.function(
    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I64, ValType::I32],
    vec![ValType::I32],
  );
  // let fd_write_type = 1;
  types.function(
    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
    vec![ValType::I32],
  );
  let parse_type = 2;
  types.function(Vec::new(), Vec::new());

  let mut imports = ImportSection::new();
  imports.import("env", "memory", MemoryType {
    minimum:  1,
    maximum:  None,
    memory64: false,
  });
  let fd_readdir_fn = 0;
  imports.import(
    "wasi_unstable", "fd_readdir", EntityType::Function(fd_readdir_fn),
  );
  let fd_write_fn = 1;
  imports.import(
    "wasi_unstable", "fd_write", EntityType::Function(fd_write_fn),
  );

  let mut functions = FunctionSection::new();
  let parse_fn = 2;
  functions.function(parse_type);

  let mut exports = ExportSection::new();
  exports.export("parse", Export::Function(parse_fn));
  exports.export("memory", Export::Memory(0));

  let mut codes = CodeSection::new();
  let mut parse_code = Function::new(Vec::new());
  parse_code.instruction(&Instruction::I32Const(3));     // fd
  parse_code.instruction(&Instruction::I32Const(0));     // buf
  parse_code.instruction(&Instruction::I32Const(1000));  // buf_len
  parse_code.instruction(&Instruction::I64Const(0));     // cookie
  parse_code.instruction(&Instruction::I32Const(1000));  // size
  parse_code.instruction(&Instruction::Call(fd_readdir_fn));
  parse_code.instruction(&Instruction::Drop);
  parse_code.instruction(&Instruction::End);
  codes.function(&parse_code);

  m.section(&types);
  m.section(&imports);
  m.section(&functions);
  m.section(&exports);
  m.section(&codes);
  let binary = m.finish();
  write("ls.wasm", &binary).unwrap();

  let wasi_ctx = WasiCtxBuilder::new()
    .inherit_stdio()
    .preopened_dir(
      Dir::from_std_file(File::open("/home/vulcain/.dropin.recipes").unwrap()),
      Path::new("/"),
    ).unwrap()
    .build();
  let engine = Engine::default();
  let mut store = Store::new(&engine, wasi_ctx);
  let mut linker = Linker::new(&engine);
  let memory = Memory::new(
    &mut store, wasmtime::MemoryType::new(1, None),
  ).unwrap();
  linker.define("env", "memory", Extern::Memory(memory)).unwrap();
  wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
  let module = wasmtime::Module::new(&engine, binary).unwrap();
  let instance = linker.instantiate(&mut store, &module).unwrap();
  let run = instance.get_typed_func::<(), (), _>(&mut store, "parse").unwrap();
  run.call(&mut store, ()).unwrap();

  let data = memory.data(&store);
  let buf_len = *data.get(1000).unwrap() as usize;
  println!("{:?}", data.get(..buf_len).unwrap());
}
