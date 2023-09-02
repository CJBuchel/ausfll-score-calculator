use crate::game_types::{ScoreAnswer, Score};

pub fn s_answer(res: Vec<ScoreAnswer>, q: &str) -> String {
  // let a = res.iter().find(|r| r.id == q.id).unwrap_or("");
  match res.iter().find(|r| r.id == q.to_string()) {
    Some(r) => r.answer.clone(),
    None => String::from(""),
  }
}

pub fn n_answer(res: Vec<ScoreAnswer>, q: &str) -> i32 {
  // let a = res.iter().find(|r| r.id == q.id).unwrap_or("");
  match res.iter().find(|r| r.id == q.to_string()) {
    Some(r) => r.answer.parse::<i32>().unwrap_or(0),
    None => 0,
  }
}