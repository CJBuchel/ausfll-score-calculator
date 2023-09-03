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
            "No",
            "Yes",
          ],
        },
    
        defaultValue: DefaultValue::Text("No")
      },

      Score {
        id: "m01a",
        label: "Is your Innovation Project model at least partly in the hydrogen plant target area?",
        labelShort: "In the area?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m02a",
        label: "Number of fuel units in the fuel truck?",
        labelShort: "Fuel in truck?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m02b",
        label: "Is at least one fuel unit in the truck and the truck is at least partly over the fueling station target?",
        labelShort: "Loaded and partly in target?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m03a",
        label: "How many energy units are completely in the energy storage bin?",
        labelShort: "Energy in bin?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m03b",
        label: "Is the energy unit completely removed from the energy storage tray?",
        labelShort: "Removed from tray?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m04a",
        label: "How many energy units have been completely removed from their starting circle?",
        labelShort: "# Removed?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m05a",
        label: "Is your field's orange connector completely raised?",
        labelShort: "Raised?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m05b",
        label: "Are both teams' orange connectors completely raised?",
        labelShort: "Both?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "Yes",
            "No",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m06a",
        label: "Is the hybrid car no longer touching the ramp?",
        labelShort: "Car not touching?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m06b",
        label: "Is the hybrid unit in the hybrid car?",
        labelShort: "Hybrid unit?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m07a",
        label: "How many energy units are no longer touching the wind turbine?",
        labelShort: "# removed?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m08a",
        label: "Is the television completely raised?",
        labelShort: "Raised?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m08b",
        label: "Is an energy unit completely in the green television slot?",
        labelShort: "Energy unit?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m09a",
        label: "Is the dinosaur toy completely in the left home area?",
        labelShort: "Completely left?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m09b",
        label: "Is the dinosaur toy lid completely closed containing...?",
        labelShort: "Closed with...?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Energy",
            "Rechargeable battery",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m10a",
        label: "How many energy units are no longer touching the power plant?",
        labelShort: "# removed?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m11a",
        label: "Is the energy unit no longer touching the hydroelectric dam?",
        labelShort: "Energy removed?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m12a",
        label: "How many looped water units are completely in the water reservoir, touching the mat?",
        labelShort: "Water touching?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m12b",
        label: "How many hooks are holding a looped water unit?",
        labelShort: "Hooks?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m13a",
        label: "How many energy units are completely in the hydrogen plant target area?",
        labelShort: "# Energy?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m14a",
        label: "How many energy units are partly in the slot or in the red hopper?",
        labelShort: "# Energy?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m14b",
        label: "Has the mini dinosaur toy been released?",
        labelShort: "Mini dino?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        defaultValue: DefaultValue::Text("No"),
      },

      Score {
        id: "m15a",
        label: "How many energy units are completely in the rechargeable battery target area?",
        labelShort: "# Energy?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        defaultValue: DefaultValue::Text("0"),
      },

      Score {
        id: "m16a",
        label: "How many Precision Tokens are left on the field?",
        labelShort: "Precision?",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
          ],
        },
        defaultValue: DefaultValue::Text("6"),
      },

      Score {
        id: "gp",
        label: "Gracious Professionalism® displayed at the robot game table?",
        labelShort: "GP",
        questionInput: QuestionInput::Categorical {
          options: vec![
            "2 - Developing",
            "3 - Accomplished",
            "4 - Exceeds"
          ],
        },
        defaultValue: DefaultValue::Text("3 - Accomplished"),
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


  fn score(answers: Vec<ScoreAnswer>) -> i32 {
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