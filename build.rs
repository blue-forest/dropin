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

use std::env::var;
use std::error::Error;
use std::fs::write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
  let out_dir = var("OUT_DIR")?;
  let host = var("DROPIN_PM_HOST")?;

  let mut dropin_core_url = host.clone();
  dropin_core_url.push_str("/blueforest/dropin-core/v1");
  let resp = reqwest::blocking::get(&dropin_core_url)?;
  if !resp.status().is_success() {
    panic!("unexpected status from {} : {}", dropin_core_url, resp.status());
  }
  let out_path = Path::new(&out_dir).join("dropin-core_v1.wasm");
  write(out_path, resp.bytes()?)?;

  let mut dropin_bootstrap_url = host.clone();
  dropin_bootstrap_url.push_str("/blueforest/dropin-bootstrap/v1");
  let resp = reqwest::blocking::get(&dropin_bootstrap_url)?;
  if !resp.status().is_success() {
    panic!("unexpected status from {} : {}", dropin_bootstrap_url, resp.status());
  }
  let out_path = Path::new(&out_dir).join("dropin-bootstrap_v1.wasm");
  write(out_path, resp.bytes()?)?;
  Ok(())
}
