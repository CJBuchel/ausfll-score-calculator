
use std::ffi::OsString;
use std::path::Path;
use std::fs;

use schema_utils::schemas::{ScoreAnswer, DefaultValue, QuestionInput, Score, ScoreError, Mission, MissionPicture, Game, Games};
use schemars::JsonSchema;

#[derive(JsonSchema, Clone)]
pub struct AusFLLSchema {
  pub score_answer: ScoreAnswer,
  pub default_value: DefaultValue,
  pub question_input: QuestionInput,
  pub score: Score,
  pub score_error: ScoreError,
  pub mission: Mission,
  pub mission_picture: MissionPicture,
  pub game: Game,
}

fn generate_games_json(outdir: &OsString) {
  let games = Games::get_games();
  let json = serde_json::to_string(&games).expect("Failed to serialize missions");
  let json_file = Path::new(outdir).join("games.json");
  fs::write(json_file, json).expect("Failed to write missions json");
}

fn generate_types_schema(outdir: &OsString) {
  let schema = schemars::schema_for!(AusFLLSchema);
  let schema_file = Path::new(outdir).join("ausfll_schema.json");
  fs::write(schema_file, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
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