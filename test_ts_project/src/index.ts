import { Convert, AusfllGameSchema } from "./schema/ausfll_game_schema";
import { ScoreAnswer, ScoreError } from "./schema/ausfll_base_schema";
import jsonData from "./schema/ausfll_games.json";

// import wasm
import * as wasm from "ausfll-wasm";

const rawGameData = JSON.parse(jsonData);
const masterpiece = rawGameData["2023"]; // get the 2023 game from the file

try {
  const games_data: AusfllGameSchema = Convert.toAusfllGameSchema(JSON.stringify(masterpiece));

  console.log("Displaying Question 1");
  console.log(games_data.questions.find((question) => question.id === "m14a"));
  console.log(games_data.questions.find((question) => question.id === "m14b"));
  
  const answers: ScoreAnswer[] = [
    {
      id: "m14a",
      answer: "1",
    },
    {
      id: "m14b",
      answer: "0",
    }
  ];

  // test wasm compiled for nodejs
  let errors: ScoreError[] = wasm.wasm_masterpiece_validate(answers);
  let score: number = wasm.wasm_masterpiece_score(answers);
  
  console.log("Validating answers");
  for (const error of errors) {
    console.log("Validation Error: " + error.id + ", m: " + error.message);
  }
  
  console.log("Scoring Answers: " + score);

} catch (error) {
  console.error(error);
}