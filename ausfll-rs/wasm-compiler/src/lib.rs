use schema_utils::schemas::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::*;

#[cfg(feature = "wasm")]
use serde_wasm_bindgen::*;

macro_rules! wasm_validator {
  ($name:ident, $type:ident) => {
    pub fn $name(answers: &Vec<ScoreAnswer>) -> Vec<ScoreError> {
      let game = $type;
      game.validate(answers.clone())
    }
    
    paste::item! {
      #[cfg(feature = "wasm")]
      #[wasm_bindgen]
      pub fn [<wasm_ $name>](answers: JsValue) -> JsValue {
          let a: Vec<ScoreAnswer> = serde_wasm_bindgen::from_value(answers).unwrap();
          let errors = $name(&a);
          serde_wasm_bindgen::to_value(&errors).unwrap()
        }
      }
  };
}

macro_rules! wasm_scorer {
  ($name:ident, $type:ident) => {
    pub fn $name(answers: &Vec<ScoreAnswer>) -> i32 {
      let game = $type;
      game.score(answers.clone())
    }
    
    paste::item! {
      #[cfg(feature = "wasm")]
      #[wasm_bindgen]
      pub fn [<wasm_ $name>](answers: JsValue) -> JsValue {
          let a: Vec<ScoreAnswer> = serde_wasm_bindgen::from_value(answers).unwrap();
          let errors = $name(&a);
          serde_wasm_bindgen::to_value(&errors).unwrap()
        }
      }
  };
}

wasm_validator!(masterpiece_validate, Masterpiece);
wasm_scorer!(masterpiece_score, Masterpiece);