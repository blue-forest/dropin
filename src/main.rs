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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use structopt::StructOpt;
// use wasmtime_wasi::sync::WasiCtxBuilder;
// use wasmtime::*;

use std::fs::read_to_string;
use std::path::PathBuf;

use dropin::parser::{print_pairs, read_file, read_type};

#[derive(StructOpt, Debug)]
enum Commands {
  /// Compile
  Compile {
    /// Path to the recipe to compile
    #[structopt(parse(from_os_str))]
    file: PathBuf
  },
  /// Debug tools. To learn more: dropin debug --help
  Debug {
    #[structopt(subcommand)]
    cmd: DebugTools,
  },
}

#[derive(StructOpt, Debug)]
struct CompileOpts {
}

#[derive(StructOpt, Debug)]
enum DebugTools {
  /// Print the recipe parser output
  Recipe {
    /// Recipe path
    #[structopt(parse(from_os_str))]
    file: PathBuf,
  },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in compiler")]
pub struct Cli {
  #[structopt(subcommand)]
  cmd: Commands,
}

fn main() {
  let cli = Cli::from_args();
  match cli.cmd {
    Commands::Compile{file} => compile(file),
    Commands::Debug{cmd} => debug(cmd),
  }
}

fn compile(path: PathBuf) {
  let type_ = read_type(path);
  println!("{:?}", type_);
  let ir = type_.compile();
  println!("{:?}", ir);
}

fn debug(cmd: DebugTools) {
  match cmd {
    DebugTools::Recipe{file} => {
      let content = read_to_string(file).unwrap();
      let pair = read_file(content.as_str());
      print_pairs(pair.into_inner(), 0);
    }
  };
}

