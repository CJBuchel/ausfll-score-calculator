use schemars::Map;
use serde::{Deserialize, Serialize, Serializer};

use super::Masterpiece;

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct ScoreAnswer {
  pub id: String,
  pub answer: String,
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub enum DefaultValue {
  Number(i32),
  Text(String),
}

// add for more inputs
#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub enum QuestionInput {
  Numerical {
    min: i32,
    max: i32,
  },
  
  Categorical {
    options: Vec<String>,
  }
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct Score {
  pub id: String,
  pub label: String,
  pub label_short: String,
  pub question_input: QuestionInput,
  pub default_value: DefaultValue,
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct ScoreError {
  pub id: String,
  pub message: String,
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct Mission {
  pub prefix: String,
  pub title: String,
  pub image: Option<String>,
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct MissionPicture { // this is kept as a static because nothing else uses it expects the missions. 
  pub prefix: &'static str, // m00/m01 etc..
  pub url: &'static str, // see firebase_links.rs
}

#[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct Game {
  pub name: String,
  pub program: String,
  pub missions: Vec<Mission>,
  pub questions: Vec<Score>,
}


pub trait AusFLLGame { // main template (does not get serialized, just for ease inside the library)
  fn get_questions(&self) -> Vec<Score>;
  fn get_missions(&self) -> Vec<Mission>;
  fn get_game(&self) -> Game;
  fn validate(&self, answers: Vec<ScoreAnswer>) -> Vec<ScoreError>;
}

// #[derive(schemars::JsonSchema, Deserialize, Serialize, Clone)]
pub struct GameMap(Map<u32, Box<dyn AusFLLGame>>);

impl GameMap {
  pub fn new() -> Self {
    GameMap(Map::new())
  }

  pub fn insert(&mut self, key: u32, value: Box<dyn AusFLLGame>) {
    self.0.insert(key, value);
  }

  pub fn iter(&self) -> impl Iterator<Item = (&u32, &Box<dyn AusFLLGame>)> {
    self.0.iter()
  }
}

impl Serialize for GameMap {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(Some(self.0.len()))?;
    for (key, value) in &self.0 {
      map.serialize_entry(key, &value.get_game())?;
    }
    map.end()
  }
}

pub struct Games;

// get all the games to generate the schemas
impl Games {
  pub fn get_games() -> GameMap {
    let mut games = GameMap::new();
    games.insert(2023, Box::new(Masterpiece));
    games
  }
}

