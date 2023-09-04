use crate::game_types::{Questions, Score, QuestionInput, DefaultValue};

use super::SuperPowered;

impl Questions for SuperPowered {
  fn get_questions(&self) -> Vec<Score<'static>> {
    vec![
      Score {
        id: "m00a",
        label: "If your robot and all your equipment fit completely in one launch area and are under a height limit of 12 inches",
        label_short: "Inspection?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
    
        default_value: DefaultValue::Text("No")
      },

      Score {
        id: "m01a",
        label: "Is your Innovation Project model at least partly in the hydrogen plant target area?",
        label_short: "In the area?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m02a",
        label: "Number of fuel units in the fuel truck?",
        label_short: "Fuel in truck?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m02b",
        label: "Is at least one fuel unit in the truck and the truck is at least partly over the fueling station target?",
        label_short: "Loaded and partly in target?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m03a",
        label: "How many energy units are completely in the energy storage bin?",
        label_short: "Energy in bin?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m03b",
        label: "Is the energy unit completely removed from the energy storage tray?",
        label_short: "Removed from tray?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m04a",
        label: "How many energy units have been completely removed from their starting circle?",
        label_short: "# Removed?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m05a",
        label: "Is your field's orange connector completely raised?",
        label_short: "Raised?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m05b",
        label: "Are both teams' orange connectors completely raised?",
        label_short: "Both?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "Yes",
            "No",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m06a",
        label: "Is the hybrid car no longer touching the ramp?",
        label_short: "Car not touching?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m06b",
        label: "Is the hybrid unit in the hybrid car?",
        label_short: "Hybrid unit?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m07a",
        label: "How many energy units are no longer touching the wind turbine?",
        label_short: "# removed?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m08a",
        label: "Is the television completely raised?",
        label_short: "Raised?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m08b",
        label: "Is an energy unit completely in the green television slot?",
        label_short: "Energy unit?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m09a",
        label: "Is the dinosaur toy completely in the left home area?",
        label_short: "Completely left?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m09b",
        label: "Is the dinosaur toy lid completely closed containing...?",
        label_short: "Closed with...?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Energy",
            "Rechargeable battery",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m10a",
        label: "How many energy units are no longer touching the power plant?",
        label_short: "# removed?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m11a",
        label: "Is the energy unit no longer touching the hydroelectric dam?",
        label_short: "Energy removed?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m12a",
        label: "How many looped water units are completely in the water reservoir, touching the mat?",
        label_short: "Water touching?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m12b",
        label: "How many hooks are holding a looped water unit?",
        label_short: "Hooks?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m13a",
        label: "How many energy units are completely in the hydrogen plant target area?",
        label_short: "# Energy?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m14a",
        label: "How many energy units are partly in the slot or in the red hopper?",
        label_short: "# Energy?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m14b",
        label: "Has the mini dinosaur toy been released?",
        label_short: "Mini dino?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "No",
            "Yes",
          ],
        },
        default_value: DefaultValue::Text("No"),
      },

      Score {
        id: "m15a",
        label: "How many energy units are completely in the rechargeable battery target area?",
        label_short: "# Energy?",
        question_input: QuestionInput::Categorical {
          options: vec![
            "0",
            "1",
            "2",
            "3",
          ],
        },
        default_value: DefaultValue::Text("0"),
      },

      Score {
        id: "m16a",
        label: "How many Precision Tokens are left on the field?",
        label_short: "Precision?",
        question_input: QuestionInput::Categorical {
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
        default_value: DefaultValue::Text("6"),
      },

      Score {
        id: "gp",
        label: "Gracious Professionalism® displayed at the robot game table?",
        label_short: "GP",
        question_input: QuestionInput::Categorical {
          options: vec![
            "2 - Developing",
            "3 - Accomplished",
            "4 - Exceeds"
          ],
        },
        default_value: DefaultValue::Text("3 - Accomplished"),
      },
    ]
  }


}