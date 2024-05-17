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

use std::io::Result;

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();
  let parser_feature = std::env::var("CARGO_FEATURE_PARSER");
  println!("cargo:rerun-if-changed=../protobufs");
  if parser_feature.is_ok() {
    println!("cargo:rerun-if-changed=src/parser/grammar.abnf");
    config
      .type_attribute("components.KeyFormat", "#[derive(serde::Deserialize)]");
  }
  config.btree_map(["."]);
  config.compile_protos(
    &["../../protobufs/components.proto"],
    &["../../protobufs/"],
  )?;
  Ok(())
}
