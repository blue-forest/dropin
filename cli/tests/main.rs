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

use clap::{
	error::{Error, ErrorKind, KindFormatter},
	Parser,
};
use dropin::{Args, Target};

#[test]
fn version() {
	let version = env!("CARGO_PKG_VERSION");
	for command in &["-V", "--version"] {
		let result = test_command(vec![command]);
		assert!(result.is_err());
		let error = result.unwrap_err();
		assert_eq!(error.kind(), ErrorKind::DisplayVersion);
		let message = error.to_string();
		assert_eq!(message, format!("dropin {}\n", version));
	}
}

#[test]
fn help() {
	for command in &["-h", "--help"] {
		let result = test_command(vec![command]);
		assert!(result.is_err());
		let error = result.unwrap_err();
		assert_eq!(error.kind(), ErrorKind::DisplayHelp);
		let message = error.to_string();
		assert!(message.starts_with("drop'in "));
	}
}

#[test]
fn empty() {
	let result = test_command(vec![]);
	assert!(result.is_err());
	let error = result.unwrap_err();
	assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
	let message = error.to_string();
	assert_eq!(
		message,
		"error: one or more required arguments were not provided\n"
	);
}

#[test]
fn unknown_argument() {
	let result = test_command(vec!["--unknown"]);
	assert!(result.is_err());
	let error = result.unwrap_err();
	assert_eq!(error.kind(), ErrorKind::UnknownArgument);
	let message = error.to_string();
	assert_eq!(message, "error: unexpected argument found\n");
}

#[test]
fn unknown_target() {
	for command in &["-t", "--target"] {
		let result = test_command(vec![command, "unknown", "a b"]);
		assert!(result.is_err());
		let error = result.unwrap_err();
		assert_eq!(error.kind(), ErrorKind::InvalidValue);
		let message = error.to_string();
		assert_eq!(
			message,
			"error: one of the values isn't valid for an argument\n"
		);
	}
}

#[test]
fn target_typescript() {
	let result = test_command(vec!["-t", "typescript", "a b"]);
	assert!(result.is_ok());
	let args = result.unwrap();
	assert_eq!(args.target, Target::Typescript);
	assert_eq!(args.input, "a b");
}

#[test]
fn target_dart() {
	let result = test_command(vec!["-t", "dart", "a b"]);
	assert!(result.is_ok());
	let args = result.unwrap();
	assert_eq!(args.target, Target::Dart);
	assert_eq!(args.input, "a b");
}

#[test]
fn target_wasm() {
	let result = test_command(vec!["-t", "wasm", "a b"]);
	assert!(result.is_ok());
	let args = result.unwrap();
	assert_eq!(args.target, Target::Wasm);
	assert_eq!(args.input, "a b");
}

fn test_command(params: Vec<&str>) -> Result<Args, Error<KindFormatter>> {
	let mut command = vec!["dropin"];
	for param in params {
		command.push(param);
	}
	Args::try_parse_from(command.clone()).map_err(|e| e.apply::<KindFormatter>())
}
