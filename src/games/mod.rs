use crate::{game_types::{Score, DefaultValue, QuestionInput, Questions, ScoreError, ScoreAnswer}, functions::{s_answer, n_answer}};

pub struct SuperPowered;

const NUM_ENERGY: i32 = 20;

impl Questions for SuperPowered {
  fn get() -> Vec<Score<'static>> {
    vec![
      Score {
        id: "m00a",
        label: "If your robot and all your equipment fit completely in one launch area and are under a height limit of 12 inches",
        labelShort: "Inspection?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "Yes",
            "No",
          ],
        },
    
        defaultValue: DefaultValue::Text("No")
      },
    ]
  }

  fn validate(answers: Vec<ScoreAnswer>) -> Vec<ScoreError> {
    let mut errors: Vec<ScoreError> = vec![];
    
    let mut empty_q_ids: Vec<String> = vec![];

    // count the empty id's
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


}