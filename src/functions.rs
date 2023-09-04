use crate::game_types::{ScoreAnswer, Score};

pub fn s_answer(res: Vec<ScoreAnswer>, q: &str) -> String {
  match res.iter().find(|r| r.id == q.to_string()) {
    Some(r) => r.answer.clone(),
    None => String::from(""),
  }
}

pub fn n_answer(res: Vec<ScoreAnswer>, q: &str) -> i32 {
  match res.iter().find(|r| r.id == q.to_string()) {
    Some(r) => r.answer.parse::<i32>().unwrap_or(0),
    None => 0,
  }
}