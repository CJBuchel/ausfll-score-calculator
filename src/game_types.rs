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
  pub labelShort: &'a str,
  pub questionInput: QuestionInput<'a>,
  pub defaultValue: DefaultValue<'a>,
}

pub struct ScoreError {
  pub id: String,
  pub message: String,
}

pub struct Mission<'a> {
  pub prefix: &'a str,
  pub title: &'a str,
  pub image: &'a str,
}

pub trait Questions {
  fn get() -> Vec<Score<'static>>;
  fn validate(answers: Vec<ScoreAnswer>) -> Vec<ScoreError>;
  fn score(answers: Vec<ScoreAnswer>) -> i32;
}