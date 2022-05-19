/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use pretty_hex::{config_hex, HexConfig};
use structopt::StructOpt;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

use std::fmt::{self, Display, Formatter};
use std::fs::read;
use std::path::PathBuf;
use std::str::{FromStr, Split};

use dropin_utils::path::{get_build, get_root};

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in debugger")]
pub struct Cli {
	#[structopt(parse(from_os_str))]
	file: PathBuf,
	#[structopt(long, short)]
	memory: Option<MemoryOpt>,
	#[structopt(long, short)]
	call: Vec<String>,
	// #[structopt(subcommand)]
	// cmd: Commands,
}

#[derive(Debug)]
struct MemoryOpt {
	pub start: usize,
	pub len: usize,
}

impl MemoryOpt {
	fn take_usize(
		split: &mut Split<char>,
	) -> Result<Option<usize>, MemoryParseError> {
		let opt = split.next();
		if opt.is_none() {
			return Err(MemoryParseError {});
		}
		let str_value = opt.unwrap();
		if str_value.is_empty() {
			Ok(None)
		} else {
			let res = str_value.parse::<usize>();
			if res.is_err() {
				return Err(MemoryParseError {});
			}
			Ok(Some(res.unwrap()))
		}
	}
}

impl FromStr for MemoryOpt {
	type Err = MemoryParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(':');
		let start = Self::take_usize(&mut split)?.unwrap_or(0);
		let len = Self::take_usize(&mut split)?.unwrap_or(128);

		Ok(Self { start, len })
	}
}

#[derive(Debug)]
struct MemoryParseError;

impl Display for MemoryParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		"invalid memory format".fmt(f)
	}
}

fn main() {
	let cli = Cli::from_args();
	let engine = Engine::default();
	let mut linker = Linker::new(&engine);
	wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
	let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
	let mut store = Store::new(&engine, wasi_ctx);

	let path = get_build(&get_root(), "blueforest", "dropin-core");
	let binary = read(&path).unwrap();
	let core_module = Module::new(&engine, binary).unwrap();
	let core_instance = linker.instantiate(&mut store, &core_module).unwrap();
	linker
		.instance(&mut store, "blueforest:dropin-core:v1", core_instance)
		.unwrap();
	let memory = if let Extern::Memory(memory) =
		core_instance.get_export(&mut store, "memory").unwrap()
	{
		memory
	} else {
		panic!("exported member \"memory\" is not Memory");
	};

	let module = Module::from_file(&engine, cli.file).unwrap();
	let instance = linker.instantiate(&mut store, &module).unwrap();

	if !cli.call.is_empty() {
		let fn_name = &cli.call[0];
		let arg = &cli.call[1];

		let alloc = core_instance
			.get_typed_func::<(u32, u32), (u32,), _>(&mut store, "alloc")
			.unwrap();
		let (addr,) = alloc.call(&mut store, (arg.len() as u32, 1)).unwrap();
		memory
			.write(&mut store, addr as usize, arg.as_bytes())
			.unwrap();

		let fn_instance = instance
			.get_typed_func::<(u32, u32), (), _>(&mut store, fn_name)
			.unwrap();
		fn_instance
			.call(&mut store, (addr, arg.len() as u32))
			.unwrap();
	} else {
		let fn_instance = instance
			.get_typed_func::<(), (), _>(&mut store, "_start")
			.unwrap();
		fn_instance.call(&mut store, ()).unwrap();
	}

	if let Some(memory_opt) = cli.memory {
		let data = memory
			.data(&store)
			.get(memory_opt.start..memory_opt.start + memory_opt.len)
			.unwrap();

		let cfg = HexConfig {
			title: false,
			..HexConfig::default()
		};
		println!("         0  1  2  3   4  5  6  7   8  9  a  b   c  d  e  f");
		println!("{}", config_hex(&data, cfg));
	}
}
