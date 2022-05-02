use std::env::var;
use std::fs::copy;
use std::path::Path;

fn main() {
  let workspace_dir = var("CARGO_MANIFEST_DIR").unwrap();
  let dropin_modules_path = Path::new(&workspace_dir).join(Path::new(
    "target/wasm32-unknown-unknown/release/dropin_modules.wasm",
  ));
  if !dropin_modules_path.exists() {
    panic!(
      "Please compile drop\'in modules:\n\
      cargo b -p dropin-modules --target wasm32-unknown-unknown --release"
    );
  }
  let dropin_bootstrap_path = Path::new(&workspace_dir).join(Path::new(
    "target/wasm32-unknown-unknown/release/dropin_bootstrap.wasm",
  ));
  if !dropin_bootstrap_path.exists() {
    panic!(
      "Please compile drop\'in bootstrap:\n\
      cargo b -p dropin-bootstrap --target wasm32-unknown-unknown --release"
    );
  }
  let out_dir = var("OUT_DIR").unwrap();
  let out_path = Path::new(&out_dir).join("dropin_modules.wasm");
  copy(dropin_modules_path, out_path).unwrap();
  let out_path = Path::new(&out_dir).join("dropin_bootstrap.wasm");
  copy(dropin_bootstrap_path, out_path).unwrap();
}
