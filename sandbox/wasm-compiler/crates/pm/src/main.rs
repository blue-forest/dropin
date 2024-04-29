/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

use reqwest::blocking::Client;
use structopt::StructOpt;

use std::env::var;
use std::error::Error;
use std::fs::read;

use dropin_utils::path::{get_build, get_root};

use dropin_pm::HOST;

#[derive(StructOpt, Debug)]
struct Cli {
	#[structopt()]
	owner: String,
	#[structopt()]
	model: String,
	#[structopt()]
	version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let cli = Cli::from_args();

	let path = get_build(&get_root(), &cli.owner, &cli.model);
	if !path.exists() {
		panic!("Not found {}", path.to_str().unwrap());
	}

	let token = var("DROPIN_PM_TOKEN")?;

	let binary = read(&path)?;
	let client = Client::new();
	let url = format!(
		"{}/{}/{}/{}/{}",
		HOST, cli.owner, cli.model, cli.version, token,
	);
	let resp = client.post(&url).body(binary).send()?;
	if !resp.status().is_success() {
		panic!("unexpected status from {} : {}", url, resp.status());
	}
	Ok(())
}
