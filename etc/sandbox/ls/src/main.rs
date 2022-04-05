use wasmtime::{self, Engine, Extern, Linker, Store, Memory};
use wasmtime_wasi::sync::{Dir, WasiCtxBuilder};

use wasm_encoder::{
  BlockType, CodeSection, EntityType, Export, ExportSection, Function,
  FunctionSection, ImportSection, Instruction, MemArg, MemoryType, Module,
  TypeSection, ValType,
};

use std::fs::{File, write};
use std::path::Path;

fn main() {
  let mut m = Module::new();
  set_types(&mut m);
  set_imports(&mut m);
  set_functions(&mut m);
  set_exports(&mut m);
  let buf_len       = 1024;
  let iovec_offset  = 1032;
  let fd_write_size = 1048;
  let data_offset   = 1052;
  set_codes(&mut m, buf_len, iovec_offset, fd_write_size, data_offset);

  let binary = m.finish();
  write("ls.wasm", &binary).unwrap();
  run(binary);
}

fn set_types(m: &mut Module) {
  let mut types = TypeSection::new();
  types.function(
    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I64, ValType::I32],
    vec![ValType::I32],
  );
  types.function(
    vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
    vec![ValType::I32],
  );
  types.function(Vec::new(), Vec::new());
  m.section(&types);
}

fn set_imports(m: &mut Module) {
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
  m.section(&imports);
}

fn set_functions(m: &mut Module) {
  let mut functions = FunctionSection::new();
  functions.function(2);
  m.section(&functions);
}

fn set_exports(m: &mut Module) {
  let mut exports = ExportSection::new();
  exports.export("parse", Export::Function(2));
  exports.export("memory", Export::Memory(0));
  m.section(&exports);
}

fn set_codes(
  m:             &mut Module,
  buf_len:       i32,
  iovec_offset:  i32,
  fd_write_size: i32,
  data_offset:   i32,
) {
  let mut codes = CodeSection::new();
  let mut parse_code = Function::new(vec![(1, ValType::I32)]);
  let dir_offset_local = 0;
  readdir(&mut parse_code, 3, 0, buf_len, 0, buf_len);
  // *data_offset = '\n'
  store(&mut parse_code, data_offset, vec![  
    Instruction::I32Const(10),
  ]);
  // iovec[1] -> $data_offset
  store(&mut parse_code, iovec_offset+8, vec![  
    Instruction::I32Const(data_offset),
  ]);
  store(&mut parse_code, iovec_offset+12, vec![  
    Instruction::I32Const(1),
  ]);

  parse_code.instruction(&Instruction::Loop(
    BlockType::FunctionType(2),
  ));
  store(&mut parse_code, iovec_offset+4, vec![
    Instruction::LocalGet(dir_offset_local),
    Instruction::I32Const(16), // pass d_next & inode to get name_len
    Instruction::I32Add,
    Instruction::I32Load(get_memarg())
  ]);
  store(&mut parse_code, iovec_offset, vec![
    Instruction::LocalGet(dir_offset_local),
    Instruction::I32Const(24), // pass dirent to get name
    Instruction::I32Add,
  ]);

  print(&mut parse_code, iovec_offset, fd_write_size);

  // $dir_offset_local += 24 + name_len
  parse_code.instruction(&Instruction::LocalGet(dir_offset_local));
  parse_code.instruction(&Instruction::I32Const(24));
  parse_code.instruction(&Instruction::I32Const(iovec_offset+4));
  parse_code.instruction(&Instruction::I32Load(get_memarg()));
  parse_code.instruction(&Instruction::I32Add);
  parse_code.instruction(&Instruction::I32Add);
  parse_code.instruction(&Instruction::LocalSet(dir_offset_local));

  // loop if $dir_offset_local > $size
  parse_code.instruction(&Instruction::I32Const(buf_len));
  parse_code.instruction(&Instruction::I32Load(get_memarg()));
  parse_code.instruction(&Instruction::LocalGet(dir_offset_local));
  parse_code.instruction(&Instruction::I32GeU);
  parse_code.instruction(&Instruction::BrIf(0));
  parse_code.instruction(&Instruction::End);

  parse_code.instruction(&Instruction::End);
  codes.function(&parse_code);
  m.section(&codes);
}

fn get_memarg() -> MemArg {
  MemArg{
    offset:       0,
    align:        2,
    memory_index: 0,
  }
}

fn readdir(
  function: &mut Function,
  fd:       i32,
  buf:      i32,
  buf_len:  i32,
  cookie:   i64,
  size:     i32,
) {
  function.instruction(&Instruction::I32Const(fd));
  function.instruction(&Instruction::I32Const(buf));
  function.instruction(&Instruction::I32Const(buf_len));
  function.instruction(&Instruction::I64Const(cookie));
  function.instruction(&Instruction::I32Const(size));
  function.instruction(&Instruction::Call(0));
  function.instruction(&Instruction::Drop);
}

fn store(function: &mut Function, offset: i32, data: Vec<Instruction>) {
  function.instruction(&Instruction::I32Const(offset));
  for d in data {
    function.instruction(&d);
  }
  function.instruction(&Instruction::I32Store(get_memarg()));
}

fn print(
  function:   &mut Function,
  iovec_base: i32,
  size:       i32,
) {
  function.instruction(&Instruction::I32Const(1));             // stdout
  function.instruction(&Instruction::I32Const(iovec_base));
  function.instruction(&Instruction::I32Const(2));             // iovecs.len
  function.instruction(&Instruction::I32Const(size));
  function.instruction(&Instruction::Call(1));
  function.instruction(&Instruction::Drop);
}

fn run(binary: Vec<u8>) {
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
}
