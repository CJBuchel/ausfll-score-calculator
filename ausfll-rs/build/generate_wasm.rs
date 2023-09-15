use std::process::Command;
use std::fs;
use std::ffi::OsString;
use std::path::Path;

// make sure `rustup target add wasm32-unknown-unknown` has been run
// and compile using `cargo build --target=wasm32-unknown-unknown --release`

pub fn generate_wasm(outdir: &OsString) {
  let schema_utils_path = "schema-utils";
  let status = Command::new("cargo")
    .args(&["build", "--target", "wasm32-unknown-unknown", "--release", "--features", "wasm"])
    .current_dir(schema_utils_path)
    .status()
    .expect("Failed to compile crate C with wasm target.");

  if !status.success() {
    panic!("Failed to build wasm for schema utils");
  }

  let wasm_file_source = format!("{}/target/wasm32-unknown-unknown/release/schema_utils.wasm", schema_utils_path); // Adjust the wasm file name.
  let wasm_file_destination = Path::new(outdir).join("ausfll_schema.wasm");

  // Copy the compiled wasm file to the desired location
  fs::copy(wasm_file_source, wasm_file_destination).expect("Failed to copy wasm file.");
}