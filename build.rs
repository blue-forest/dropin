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
use std::fs::copy;
use std::path::Path;

fn main() {
    let workspace_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let dropin_core_path = Path::new(&workspace_dir).join(Path::new(
        "target/wasm32-unknown-unknown/release/dropin_core.wasm",
    ));
    if !dropin_core_path.exists() {
        panic!(
            "Please compile drop\'in core:\n\
      cargo b -p dropin-core --target wasm32-unknown-unknown --release"
        );
    }
    println!(
        "cargo:rerun-if-changed={}",
        dropin_core_path.to_str().unwrap(),
    );
    let dropin_bootstrap_path = Path::new(&workspace_dir).join(Path::new(
        "target/wasm32-unknown-unknown/release/dropin_bootstrap.wasm",
    ));
    if !dropin_bootstrap_path.exists() {
        panic!(
            "Please compile drop\'in bootstrap:\n\
      cargo b -p dropin-bootstrap --target wasm32-unknown-unknown --release"
        );
    }
    println!(
        "cargo:rerun-if-changed={}",
        dropin_bootstrap_path.to_str().unwrap(),
    );
    let out_dir = var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("dropin_core.wasm");
    copy(dropin_core_path, out_path).unwrap();
    let out_path = Path::new(&out_dir).join("dropin_bootstrap.wasm");
    copy(dropin_bootstrap_path, out_path).unwrap();
}
