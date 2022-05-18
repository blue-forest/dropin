use std::path::{Path, PathBuf};

#[cfg(target_family = "wasm")]
pub fn root() -> PathBuf {
	PathBuf::from("/")
}

#[cfg(not(target_family = "wasm"))]
pub fn root() -> PathBuf {
	use home::home_dir;
	use path_clean::PathClean;
	use std::env::{current_dir, var};
	use crate::error::PortableUnwrap;

	if let Ok(root) = var("DROPIN_ROOT") {
		println!("Using $DROPIN_ROOT ({})", root);
		return PathBuf::from(root);
	}
	let mut path = match home_dir() {
		Some(path) => path.join(".dropin.recipes"),
		None => current_dir().punwrap().join(".dropin.recipes"),
	};
	if path.is_relative() {
		path = current_dir().punwrap().join(path).clean();
	} else {
		path = path.clean();
	}
	path
}

pub fn model_path(
	root: &Path,
	owner: &str,
	model: &str,
	version: &str,
) -> PathBuf {
	let mut path = root.to_path_buf();
	path.push(owner);
	path.push("models");
	path.push(model);
	path.push(version);
	path
}

fn build(
	root: &Path,
	owner: &str,
	model: &str,
	version: &str,
	extension: &str,
) -> PathBuf {
	/*
	let owner = iter.next().pexpect("expected owner");
	let model = iter.next().pexpect("expected model");
	let version = iter.next().pexpect("expected version");
	*/
	let mut path = root.to_path_buf();
	path.push(".builds");
	path.push(owner);
	path.push(format!("{}_{}.{}", model, version, extension));
	path
}

pub fn wasm(root: &Path, owner: &str, model: &str, version: &str) -> PathBuf {
	build(root, owner, model, version, "wasm")
}

pub fn header(root: &Path, owner: &str, model: &str, version: &str) -> PathBuf {
	build(root, owner, model, version, "dh")
}

