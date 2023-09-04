use crate::{game_types::{Game, ScoreError, ScoreAnswer, Questions, Missions}, functions::{n_answer, s_answer}};

use super::SuperPowered;

const NUM_ENERGY: i32 = 20;

impl Game for SuperPowered {
  fn get_name(&self) -> &'static str {
    "Super Powered"
  }

  fn get_program(&self) -> &'static str {
    "FLL_CHALLENGE"
  }

  fn questions(&self) -> Vec<crate::game_types::Score<'static>> {
    self.get_questions()
  }

  fn missions(&self) -> Vec<crate::game_types::Mission<'static>> {
    self.get_missions()
  }

  fn validate(&self, answers: Vec<ScoreAnswer>) -> Vec<ScoreError> {
    let mut errors: Vec<ScoreError> = vec![];
    let mut empty_q_ids: Vec<String> = vec![];
    let n_empty_ids = answers.iter().filter(|r| r.answer == "").count();

    // push any empty questions
    for r in answers.clone() {
      if r.answer.is_empty() {
        empty_q_ids.push(r.id);
      }
    }

    if n_empty_ids > 0 {
      errors.push(ScoreError {
        id: empty_q_ids.join(","),
        message: format!("{} unanswered questions", n_empty_ids)
      })
    }

    if s_answer(answers.clone(), "m02a") == "0" && s_answer(answers.clone(), "m02b") == "Yes" {
      errors.push(ScoreError {
        id: "m02a,m02b".to_string(),
        message: "The fuel truck cannot contain fuel if there is no fuel in the fuel truck.".to_string()
      })
    }

    let m03a = n_answer(answers.clone(), "m03a");
    let m03b = if s_answer(answers.clone(), "m03b") == "No" {1} else {0};
    let m04a = 3 - n_answer(answers.clone(), "m04a");
    let m07a = 3 - n_answer(answers.clone(), "m07a");
    let m08b = if s_answer(answers.clone(), "m08b") == "Yes" { 1 } else { 0 };
    let m09b = if s_answer(answers.clone(), "m09b") == "Energy" && s_answer(answers.clone(), "m09a") == "Yes" { 1 } else { 0 };
    let m10a = 3 - n_answer(answers.clone(), "m10a");
    let m11a = if s_answer(answers.clone(), "m11a") == "No" { 1 } else { 0 };
    let m13a = n_answer(answers.clone(), "m13a");
    let m14a = n_answer(answers.clone(), "m14a");
    let m15a = n_answer(answers.clone(), "m15a");

    // Total number of mutually exclusive containers:
    if m03a + m03b + m04a + m07a + m08b + m09b + m10a + m11a + m13a + m14a + m15a > NUM_ENERGY {
      errors.push(ScoreError {
        id: format!("{},{},{},{},{},{},{},{},{},{},{}", 
                    if m03a != 0 { "m03a" } else { "" }, 
                    if m03b != 0 { "m03b" } else { "" }, 
                    if m04a != 0 { "m04a" } else { "" }, 
                    if m07a != 0 { "m07a" } else { "" },
                    if m08b != 0 { "m08b" } else { "" },
                    if m09b != 0 { "m09b" } else { "" },
                    if m10a != 0 { "m10a" } else { "" },
                    if m11a != 0 { "m11a" } else { "" },
                    if m13a != 0 { "m13a" } else { "" },
                    if m14a != 0 { "m14a" } else { "" },
                    if m15a != 0 { "m15a" } else { "" }),
        message: format!("Too many energy units have been used - there are only {}!", NUM_ENERGY),
      });
    }

    // Water units
    let m12a = n_answer(answers.clone(), "m12a");
    let m12b = n_answer(answers.clone(), "m12b");
    if m12a + m12b > 3 {
      errors.push(ScoreError {
        id: "m12a,m12b".to_string(),
        message: "Looped water units cannot be touching the ground and hanging on the hooks.".to_string(),
      });
    }

    errors
  }

  fn score(&self, answers: Vec<ScoreAnswer>) -> i32 {
    let mut _score = 0;

    // M00
    if s_answer(answers.clone(), "m00a") == "Yes" { _score += 20; }

    // M01
    if s_answer(answers.clone(), "m01a") == "Yes" { _score += 10; }

    // M02
    _score += n_answer(answers.clone(), "m02a") * 5;
    if s_answer(answers.clone(), "m02b") == "Yes" { _score += 10; }

    // M03
    _score += n_answer(answers.clone(), "m03a") * 10;
    if s_answer(answers.clone(), "m03b") == "Yes" { _score += 5; }

    // M04
    let m04a = n_answer(answers.clone(), "m04a");
    _score += m04a * 5;
    if m04a == 3 { _score += 5; }

    // M05
    if s_answer(answers.clone(), "m05a") == "Yes" { _score += 20; }
    if s_answer(answers.clone(), "m05b") == "Yes" { _score += 10; }

    // M06
    if s_answer(answers.clone(), "m06a") == "Yes" { _score += 10; }
    if s_answer(answers.clone(), "m06b") == "Yes" { _score += 10; }

    // M07
    _score += n_answer(answers.clone(), "m07a") * 10;

    // M08
    if s_answer(answers.clone(), "m08a") == "Yes" { _score += 10; }
    if s_answer(answers.clone(), "m08b") == "Yes" { _score += 10; }

    // M09
    if s_answer(answers.clone(), "m09a") == "Yes" { _score += 10; }
    let m09b = s_answer(answers.clone(), "m09b");
    if m09b == "Energy" { _score += 10; }
    else if m09b == "Rechargeable battery" { _score += 20; }

    // M10
    let m10a = n_answer(answers.clone(), "m10a");
    _score += m10a * 5;
    if m10a == 3 { _score += 10; }

    // M11
    if s_answer(answers.clone(), "m11a") == "Yes" { _score += 20; }

    // M12
    _score += n_answer(answers.clone(), "m12a") * 5;
    _score += n_answer(answers.clone(), "m12b") * 10;

    // M13
    _score += n_answer(answers.clone(), "m13a") * 5;

    // M14
    _score += n_answer(answers.clone(), "m14a") * 5;
    if s_answer(answers.clone(), "m14b") == "Yes" { _score += 10; }

    // M15
    _score += n_answer(answers.clone(), "m15a") * 5;

    // M16
    match s_answer(answers.clone(), "m16a").as_str() {
      "6" | "5" => _score += 50,
      "4" => _score += 35,
      "3" => _score += 25,
      "2" => _score += 15,
      "1" => _score += 10,
      _ => _score += 0,
    }
    _score
  }
}