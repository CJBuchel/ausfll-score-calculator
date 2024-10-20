use std::fs;
use std::process::Command;
use std::ffi::OsString;
use std::path::Path;
use fs_extra::dir::copy;

pub fn generate_wasm(outdir: &OsString) {
  let wasm_compiler_path = "ausfll-wasm";

  let status_wasm_pack = Command::new("cargo")
    .args(&["install", "wasm-pack"])
    .current_dir(wasm_compiler_path)
    .status()
    .expect("Failed to install wasm-pack");

  if !status_wasm_pack.success() {
    panic!("Failed to install wasm-pack");
  }

  let status_wasm_pack_build = Command::new("wasm-pack")
    .args(&["build", "--target", "nodejs", "--out-dir", "ausfll_wasm_pkg"])
    .current_dir(wasm_compiler_path)
    .status()
    .expect("Failed to compile crate C with wasm target.");

  if !status_wasm_pack_build.success() {
    panic!("Failed to create wasm package");
  }

  let source_dir = Path::new(wasm_compiler_path).join("ausfll_wasm_pkg");
  let dest_dir = Path::new(outdir);

  // remove the gitignore becuase it's medling
  fs::remove_file(source_dir.join(".gitignore")).expect("Failed to remove .gitignore");

  // Remove the destination directory if it exists
  if dest_dir.join("ausfll_wasm_pkg").exists() {
    fs::remove_dir_all(dest_dir.join("ausfll_wasm_pkg")).expect("Failed to remove destination directory.");
  }

  // Copy the compiled wasm package to the desired location
  copy(source_dir, dest_dir, &fs_extra::dir::CopyOptions::new())
    .expect("Failed to copy directory.");
}



