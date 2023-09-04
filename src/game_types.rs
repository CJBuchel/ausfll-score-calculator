use std::vec;

use crate::{functions::s_answer, games::super_powered_2022::SuperPowered};

#[derive(Clone)]
pub struct ScoreAnswer {
  pub id: String,
  pub answer: String,
}

pub enum DefaultValue<'a> {
  Number(i32),
  Text(&'a str)
}

// add for more inputs
pub enum QuestionInput<'a> {
  Numerical {
    min: i32,
    max: i32,
  },
  Categorical {
    options: Vec<&'a str>,
  }
}

pub struct Score<'a> {
  pub id: &'a str,
  pub label: &'a str,
  pub label_short: &'a str,
  pub question_input: QuestionInput<'a>,
  pub default_value: DefaultValue<'a>,
}

pub struct ScoreError {
  pub id: String,
  pub message: String,
}

pub struct Mission<'a> {
  pub prefix: &'a str,
  pub title: &'a str,
  pub image: Option<&'a str>,
}

pub trait Missions {
  fn get_missions(&self) -> Vec<Mission<'static>>;
}

pub trait Questions {
  fn get_questions(&self) -> Vec<Score<'static>>;
}

pub trait Game {
  fn get_name(&self) -> &'static str;
  fn get_program(&self) -> &'static str;
  fn questions(&self) -> Vec<Score<'static>>;
  fn missions(&self) -> Vec<Mission<'static>>;

  fn answer(&self, res: Vec<ScoreAnswer>, q: &str) -> String {
    s_answer(res, q)
  }

  // main score/validations
  fn validate(&self, answers: Vec<ScoreAnswer>) -> Vec<ScoreError>;
  fn score(&self, answers: Vec<ScoreAnswer>) -> i32;
}