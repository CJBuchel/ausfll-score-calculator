use crate::game_types::{Missions, Mission};


use super::SuperPowered;
use super::super::super::firebase_links::SUPER_POWERED_MISSION_PICS as MISSION_PICS;

impl Missions for SuperPowered {
  fn get_missions(&self) -> Vec<Mission<'static>> {
    vec![
      Mission {
        prefix: "m00",
        title: "M00 - Equipment Inspection Bonus",
        image: Some(MISSION_PICS[00]),
      },

      Mission {
        prefix: "m01",
        title: "M01 - Innovation Project Model",
        image: Some(MISSION_PICS[01]),
      },

      Mission {
        prefix: "m02",
        title: "M02 - Oil Platform",
        image: Some(MISSION_PICS[02]),
      },

      Mission {
        prefix: "m03",
        title: "M03 - Energy Storage",
        image: Some(MISSION_PICS[03]),
      },

      Mission {
        prefix: "m04",
        title: "M04 - Solar Farm",
        image: Some(MISSION_PICS[04]),
      },

      Mission {
        prefix: "m05",
        title: "M05 - Smart Grid",
        image: Some(MISSION_PICS[05]),
      },

      Mission {
        prefix: "m06",
        title: "M06 - Hybrid Car",
        image: Some(MISSION_PICS[06]),
      },

      Mission {
        prefix: "m07",
        title: "M07 - Wind Turbine",
        image: Some(MISSION_PICS[07]),
      },

      Mission {
        prefix: "m08",
        title: "M08 - Watch Television",
        image: Some(MISSION_PICS[08]),
      },

      Mission {
        prefix: "m09",
        title: "M09 - Dinosaur Toy",
        image: Some(MISSION_PICS[09]),
      },

      Mission {
        prefix: "m10",
        title: "M10 - Power Plant",
        image: Some(MISSION_PICS[10]),
      },

      Mission {
        prefix: "m11",
        title: "M11 - Hydroelectric Dam",
        image: Some(MISSION_PICS[11]),
      },

      Mission {
        prefix: "m12",
        title: "M12 - Water Reservoir",
        image: Some(MISSION_PICS[12]),
      },

      Mission {
        prefix: "m13",
        title: "M13 - Power-To-X",
        image: Some(MISSION_PICS[13]),
      },

      Mission {
        prefix: "m14",
        title: "M14 - Toy Factory",
        image: Some(MISSION_PICS[14]),
      },

      Mission {
        prefix: "m15",
        title: "M15 - Rechargeable battery",
        image: Some(MISSION_PICS[15]),
      },

      Mission {
        prefix: "m16",
        title: "M16 - Precision Tokens",
        image: Some(MISSION_PICS[16]),
      },

      Mission {
        prefix: "gp",
        title: "Gracious Professionalism®",
        image: None,
      }
    ]
  }
}