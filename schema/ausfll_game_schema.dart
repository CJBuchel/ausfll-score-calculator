// To parse this JSON data, do
//
//     final ausfllGameSchema = ausfllGameSchemaFromJson(jsonString);

import 'dart:convert';

AusfllGameSchema ausfllGameSchemaFromJson(String str) => AusfllGameSchema.fromJson(json.decode(str));

String ausfllGameSchemaToJson(AusfllGameSchema data) => json.encode(data.toJson());

class AusfllGameSchema {
    List<Mission> missions;
    String name;
    String program;
    List<Score> questions;

    AusfllGameSchema({
        required this.missions,
        required this.name,
        required this.program,
        required this.questions,
    });

    factory AusfllGameSchema.fromJson(Map<String, dynamic> json) => AusfllGameSchema(
        missions: List<Mission>.from(json["missions"].map((x) => Mission.fromJson(x))),
        name: json["name"],
        program: json["program"],
        questions: List<Score>.from(json["questions"].map((x) => Score.fromJson(x))),
    );

    Map<String, dynamic> toJson() => {
        "missions": List<dynamic>.from(missions.map((x) => x.toJson())),
        "name": name,
        "program": program,
        "questions": List<dynamic>.from(questions.map((x) => x.toJson())),
    };
}

class Mission {
    String? image;
    String prefix;
    String title;

    Mission({
        this.image,
        required this.prefix,
        required this.title,
    });

    factory Mission.fromJson(Map<String, dynamic> json) => Mission(
        image: json["image"],
        prefix: json["prefix"],
        title: json["title"],
    );

    Map<String, dynamic> toJson() => {
        "image": image,
        "prefix": prefix,
        "title": title,
    };
}

class Score {
    DefaultValue defaultValue;
    String id;
    String label;
    String labelShort;
    QuestionInput questionInput;

    Score({
        required this.defaultValue,
        required this.id,
        required this.label,
        required this.labelShort,
        required this.questionInput,
    });

    factory Score.fromJson(Map<String, dynamic> json) => Score(
        defaultValue: DefaultValue.fromJson(json["default_value"]),
        id: json["id"],
        label: json["label"],
        labelShort: json["label_short"],
        questionInput: QuestionInput.fromJson(json["question_input"]),
    );

    Map<String, dynamic> toJson() => {
        "default_value": defaultValue.toJson(),
        "id": id,
        "label": label,
        "label_short": labelShort,
        "question_input": questionInput.toJson(),
    };
}

class DefaultValue {
    int? number;
    String? text;

    DefaultValue({
        this.number,
        this.text,
    });

    factory DefaultValue.fromJson(Map<String, dynamic> json) => DefaultValue(
        number: json["Number"],
        text: json["Text"],
    );

    Map<String, dynamic> toJson() => {
        "Number": number,
        "Text": text,
    };
}

class QuestionInput {
    Numerical? numerical;
    Categorical? categorical;

    QuestionInput({
        this.numerical,
        this.categorical,
    });

    factory QuestionInput.fromJson(Map<String, dynamic> json) => QuestionInput(
        numerical: json["Numerical"] == null ? null : Numerical.fromJson(json["Numerical"]),
        categorical: json["Categorical"] == null ? null : Categorical.fromJson(json["Categorical"]),
    );

    Map<String, dynamic> toJson() => {
        "Numerical": numerical?.toJson(),
        "Categorical": categorical?.toJson(),
    };
}

class Categorical {
    List<String> options;

    Categorical({
        required this.options,
    });

    factory Categorical.fromJson(Map<String, dynamic> json) => Categorical(
        options: List<String>.from(json["options"].map((x) => x)),
    );

    Map<String, dynamic> toJson() => {
        "options": List<dynamic>.from(options.map((x) => x)),
    };
}

class Numerical {
    int max;
    int min;

    Numerical({
        required this.max,
        required this.min,
    });

    factory Numerical.fromJson(Map<String, dynamic> json) => Numerical(
        max: json["max"],
        min: json["min"],
    );

    Map<String, dynamic> toJson() => {
        "max": max,
        "min": min,
    };
}
