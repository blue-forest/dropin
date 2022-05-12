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

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
enum Commands {
	Memory {
		#[structopt(parse(from_os_str))]
		file: PathBuf,
		#[structopt(long, short, default_value = "0")]
		start: usize,
		#[structopt(long, short, default_value = "128")]
		len: usize,
	},
}

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in debugger")]
pub struct Cli {
	#[structopt(subcommand)]
	cmd: Commands,
}

fn main() {
	let cli = Cli::from_args();
	match cli.cmd {
		Commands::Memory { file, start, len } => memory(file, start, len),
	}
}

fn memory(file: PathBuf, start: usize, len: usize) {
	let engine = Engine::default();
	let mut linker = Linker::new(&engine);
	wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
	let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
	let mut store = Store::new(&engine, wasi_ctx);

	let module = Module::from_file(&engine, file).unwrap();
	let instance = linker.instantiate(&mut store, &module).unwrap();

	let start_fn = instance
		.get_typed_func::<(), (), _>(&mut store, "_start")
		.unwrap();
	start_fn.call(&mut store, ()).unwrap();
	let memory = if let Extern::Memory(memory) =
		instance.get_export(&mut store, "memory").unwrap()
	{
		memory
	} else {
		panic!("exported member \"memory\" is not Memory");
	};
	let data = memory.data(&store).get(start..start + len).unwrap();

	let cfg = HexConfig {
		title: false,
		..HexConfig::default()
	};
	println!("         0  1  2  3   4  5  6  7   8  9  a  b   c  d  e  f");
	println!("{}", config_hex(&data, cfg));
}
