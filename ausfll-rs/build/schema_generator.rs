
use std::ffi::OsString;
use std::path::Path;
use std::fs;

use fll_games::schemas::{Game, Games, ScoreAnswer, DefaultValue, QuestionInput, Score, ScoreError, Mission, MissionPicture};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, Clone)]
struct BaseSchema {
  score_answer: ScoreAnswer,
  default_value: DefaultValue,
  question_input: QuestionInput,
  score: Score,
  score_error: ScoreError,
  mission: Mission,
  mission_picture: MissionPicture,
}

fn generate_games_json(outdir: &OsString) {
  let games = Games::get_games();
  let json = serde_json::to_string(&games).expect("Failed to serialize missions");
  let json_file = Path::new(outdir).join("ausfll_games.json");
  fs::write(json_file, serde_json::to_string(&json).unwrap()).expect("Failed to write missions json");
}

fn generate_types_schema(outdir: &OsString) {
  let base_schema = schemars::schema_for!(BaseSchema);
  let game_schema = schemars::schema_for!(Game);
  let game_schema_file = Path::new(outdir).join("ausfll_game_schema.json");
  let base_schema_file = Path::new(outdir).join("ausfll_base_schema.json");
  fs::write(game_schema_file.clone(), serde_json::to_string_pretty(&game_schema).unwrap()).unwrap();
  fs::write(base_schema_file.clone(), serde_json::to_string_pretty(&base_schema).unwrap()).unwrap();
}

pub fn generate_schema(outdir: &OsString) {
  generate_games_json(outdir);
  generate_types_schema(outdir);

  // run command to convert them to relative lanuages
  // let npm_dir = Path::new(outdir).join("../");
  let status = std::process::Command::new("npm")
    .args(&["run", "convert"])
    .status()
    .expect("Failed to convert to relative languages.");

  if !status.success() {
    panic!("Failed to convert schema files");
  }
}