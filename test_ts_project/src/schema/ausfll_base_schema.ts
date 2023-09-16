// To parse this data:
//
//   import { Convert, AusfllBaseSchema } from "./file";
//
//   const ausfllBaseSchema = Convert.toAusfllBaseSchema(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface AusfllBaseSchema {
    default_value:   DefaultValue;
    mission:         Mission;
    mission_picture: MissionPicture;
    question_input:  QuestionInput;
    score:           Score;
    score_answer:    ScoreAnswer;
    score_error:     ScoreError;
    [property: string]: any;
}

export interface DefaultValue {
    Number?: number;
    Text?:   string;
}

export interface Mission {
    image?: null | string;
    prefix: string;
    title:  string;
    [property: string]: any;
}

export interface MissionPicture {
    prefix: string;
    url:    string;
    [property: string]: any;
}

export interface QuestionInput {
    Numerical?:   Numerical;
    Categorical?: Categorical;
}

export interface Categorical {
    options: string[];
    [property: string]: any;
}

export interface Numerical {
    max: number;
    min: number;
    [property: string]: any;
}

export interface Score {
    default_value:  DefaultValue;
    id:             string;
    label:          string;
    label_short:    string;
    question_input: QuestionInput;
    [property: string]: any;
}

export interface ScoreAnswer {
    answer: string;
    id:     string;
    [property: string]: any;
}

export interface ScoreError {
    id:      string;
    message: string;
    [property: string]: any;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toAusfllBaseSchema(json: string): AusfllBaseSchema {
        return cast(JSON.parse(json), r("AusfllBaseSchema"));
    }

    public static ausfllBaseSchemaToJson(value: AusfllBaseSchema): string {
        return JSON.stringify(uncast(value, r("AusfllBaseSchema")), null, 2);
    }
}

function invalidValue(typ: any, val: any, key: any, parent: any = ''): never {
    const prettyTyp = prettyTypeName(typ);
    const parentText = parent ? ` on ${parent}` : '';
    const keyText = key ? ` for key "${key}"` : '';
    throw Error(`Invalid value${keyText}${parentText}. Expected ${prettyTyp} but got ${JSON.stringify(val)}`);
}

function prettyTypeName(typ: any): string {
    if (Array.isArray(typ)) {
        if (typ.length === 2 && typ[0] === undefined) {
            return `an optional ${prettyTypeName(typ[1])}`;
        } else {
            return `one of [${typ.map(a => { return prettyTypeName(a); }).join(", ")}]`;
        }
    } else if (typeof typ === "object" && typ.literal !== undefined) {
        return typ.literal;
    } else {
        return typeof typ;
    }
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = '', parent: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key, parent);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) {}
        }
        return invalidValue(typs, val, key, parent);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases.map(a => { return l(a); }), val, key, parent);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue(l("array"), val, key, parent);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue(l("Date"), val, key, parent);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue(l(ref || "object"), val, key, parent);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, key, ref);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key, ref);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val, key, parent);
    }
    if (typ === false) return invalidValue(typ, val, key, parent);
    let ref: any = undefined;
    while (typeof typ === "object" && typ.ref !== undefined) {
        ref = typ.ref;
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems")    ? transformArray(typ.arrayItems, val)
            : typ.hasOwnProperty("props")         ? transformObject(getProps(typ), typ.additional, val)
            : invalidValue(typ, val, key, parent);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function l(typ: any) {
    return { literal: typ };
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "AusfllBaseSchema": o([
        { json: "default_value", js: "default_value", typ: r("DefaultValue") },
        { json: "mission", js: "mission", typ: r("Mission") },
        { json: "mission_picture", js: "mission_picture", typ: r("MissionPicture") },
        { json: "question_input", js: "question_input", typ: r("QuestionInput") },
        { json: "score", js: "score", typ: r("Score") },
        { json: "score_answer", js: "score_answer", typ: r("ScoreAnswer") },
        { json: "score_error", js: "score_error", typ: r("ScoreError") },
    ], "any"),
    "DefaultValue": o([
        { json: "Number", js: "Number", typ: u(undefined, 0) },
        { json: "Text", js: "Text", typ: u(undefined, "") },
    ], false),
    "Mission": o([
        { json: "image", js: "image", typ: u(undefined, u(null, "")) },
        { json: "prefix", js: "prefix", typ: "" },
        { json: "title", js: "title", typ: "" },
    ], "any"),
    "MissionPicture": o([
        { json: "prefix", js: "prefix", typ: "" },
        { json: "url", js: "url", typ: "" },
    ], "any"),
    "QuestionInput": o([
        { json: "Numerical", js: "Numerical", typ: u(undefined, r("Numerical")) },
        { json: "Categorical", js: "Categorical", typ: u(undefined, r("Categorical")) },
    ], false),
    "Categorical": o([
        { json: "options", js: "options", typ: a("") },
    ], "any"),
    "Numerical": o([
        { json: "max", js: "max", typ: 0 },
        { json: "min", js: "min", typ: 0 },
    ], "any"),
    "Score": o([
        { json: "default_value", js: "default_value", typ: r("DefaultValue") },
        { json: "id", js: "id", typ: "" },
        { json: "label", js: "label", typ: "" },
        { json: "label_short", js: "label_short", typ: "" },
        { json: "question_input", js: "question_input", typ: r("QuestionInput") },
    ], "any"),
    "ScoreAnswer": o([
        { json: "answer", js: "answer", typ: "" },
        { json: "id", js: "id", typ: "" },
    ], "any"),
    "ScoreError": o([
        { json: "id", js: "id", typ: "" },
        { json: "message", js: "message", typ: "" },
    ], "any"),
};
