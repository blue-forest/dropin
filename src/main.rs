/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation under version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use structopt::StructOpt;

use std::path::PathBuf;

use dropin_helpers::fs;

mod embedder;
pub use embedder::Embedder;
mod interactive;
mod utils;

#[derive(StructOpt)]
enum Command {
	Run {
		#[structopt(long)]
		share_root: bool,
		#[structopt(parse(from_os_str))]
		file: PathBuf,
	},
}

#[derive(StructOpt)]
#[structopt(name = "drop'in", about = "a universe to shape your ideas")]
struct Opt {
	#[structopt(subcommand)]
	cmd: Option<Command>,
}

fn main() {
	let args = Opt::from_args();
	if let Some(Command::Run { file, share_root }) = args.cmd {
		let root = fs::root();
		let mut embedder = Embedder::new(&root);
		embedder.run(if share_root { Some(&root) } else { None }, &file, "_start");
	} else {
		interactive::Cli::new().run();
	}
}
