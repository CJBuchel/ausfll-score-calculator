"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const ausfll_game_schema_1 = require("./schema/ausfll_game_schema");
const ausfll_games_json_1 = __importDefault(require("./schema/ausfll_games.json"));
// import wasm
const wasm = __importStar(require("ausfll-wasm"));
const rawGameData = JSON.parse(ausfll_games_json_1.default);
const masterpiece = rawGameData["2023"]; // get the 2023 game from the file
try {
    const games_data = ausfll_game_schema_1.Convert.toAusfllGameSchema(JSON.stringify(masterpiece));
    console.log("Displaying Question 1");
    console.log(games_data.questions.find((question) => question.id === "m14a"));
    console.log(games_data.questions.find((question) => question.id === "m14b"));
    const answers = [
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
    let errors = wasm.wasm_masterpiece_validate(answers);
    let score = wasm.wasm_masterpiece_score(answers);
    console.log("Validating answers");
    for (const error of errors) {
        console.log("Error: " + error.id + ", m: " + error.message);
    }
    console.log("Scoring Answers: " + score);
}
catch (error) {
    console.error(error);
}
