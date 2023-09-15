use std::{env, fs};

use std::ffi::OsString;
use std::fs::read_dir;
use std::path::PathBuf;
use std::io;
use std::io::ErrorKind;

mod schema_generator;
use schema_generator::generate_schema;

mod generate_wasm;
use generate_wasm::generate_wasm;

pub fn get_project_root() -> io::Result<PathBuf> {
  let path = env::current_dir()?;
  let mut path_ancestors = path.as_path().ancestors();

  while let Some(p) = path_ancestors.next() {
    let has_cargo =
      read_dir(p)?
        .into_iter()
        .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
    if has_cargo {
      return Ok(PathBuf::from(p))
    }
  }
  Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}

fn main() -> anyhow::Result<()> {

  // generate schema file
  let schema_dir = get_project_root().unwrap();
  let schema_dir = schema_dir.join("../schema");
  fs::create_dir_all(schema_dir.clone()).unwrap();
  generate_schema(&schema_dir.clone().into_os_string());

  // generate wasm
  generate_wasm(&schema_dir.clone().into_os_string());

  Ok(())
}