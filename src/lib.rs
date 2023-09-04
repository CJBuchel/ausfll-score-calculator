

mod firebase_links;
mod game_types;
mod functions;
mod games;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Season {
  super_powered_2022: games::super_powered_2022::SuperPowered,
}

#[wasm_bindgen]
pub struct TestWasm {
  test: String,
}

#[wasm_bindgen]
impl TestWasm {
  pub fn new() -> TestWasm {
    TestWasm {
      test: String::from("Hello, world!"),
    }
  }

  pub fn test(&self) -> String {
    self.test.clone()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    println!("Hello, world!");
  }
}