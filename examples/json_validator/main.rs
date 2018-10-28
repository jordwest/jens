extern crate jens;

use jens::{Block, File};

enum Json {
    JsString,
    JsNumber,
    JsArray(Box<Json>),
    JsObject(String),
}

impl Json {
    fn get_ts_type(&self) -> String {
        use Json::*;
        match self {
            JsString => "string".into(),
            JsNumber => "number".into(),
            JsArray(subtype) => format!("{}[]", subtype.get_ts_type()),
            JsObject(subtype) => format!("{}.T", subtype),
        }
    }

    fn get_serialize_func(&self) -> String {
        use Json::*;
        match self {
            JsString => format!("noop"),
            JsNumber => format!("noop"),
            JsArray(subtype) => format!("serialize_array({})", subtype.get_serialize_func()),
            JsObject(subtype) => format!("{}.serialize", subtype),
        }
    }

    fn get_deserialize_func(&self) -> String {
        use Json::*;
        match self {
            JsString => format!("deserialize_string"),
            JsNumber => format!("deserialize_number"),
            JsArray(subtype) => format!("deserialize_array({})", subtype.get_deserialize_func()),
            JsObject(subtype) => format!("{}.deserialize", subtype),
        }
    }
}

struct TsField {
    field_name: String,
    field_type: Json,
}

struct TsType {
    type_name: String,
    fields: Vec<TsField>,
}

fn main() {
    let types = vec![
        TsType {
            type_name: "Book".into(),
            fields: vec![TsField {
                field_name: "title".into(),
                field_type: Json::JsString,
            }],
        },
        TsType {
            type_name: "LibraryMeta".into(),
            fields: vec![
                TsField {
                    field_name: "founded_year".into(),
                    field_type: Json::JsNumber,
                },
                TsField {
                    field_name: "name".into(),
                    field_type: Json::JsString,
                },
            ],
        },
        TsType {
            type_name: "Library".into(),
            fields: vec![
                TsField {
                    field_name: "count".into(),
                    field_type: Json::JsNumber,
                },
                TsField {
                    field_name: "meta".into(),
                    field_type: Json::JsObject("LibraryMeta".into()),
                },
                TsField {
                    field_name: "books".into(),
                    field_type: Json::JsArray(Box::new(Json::JsObject("Book".into()))),
                },
            ],
        },
    ];

    let f = File::parse(include_str!("./template.jens"));

    let output = f.template("main").set(
        "types",
        Block::join_map(types, |t, _| {
            f.template("type")
                .set("type_name", t.type_name.clone())
                .set("type_def_fields", type_def_fields(&f, &t))
                .set("serialize_fields", serialize_fields(&f, &t))
                .set("deserialize_fields", deserialize_fields(&f, &t))
        }),
    );

    println!("{}", output);
}

fn type_def_fields(f: &File, t: &TsType) -> Block {
    Block::join_map(&t.fields, |field, _| {
        f.template("type_def")
            .set("field_name", field.field_name.clone())
            .set("field_type", field.field_type.get_ts_type())
    })
}

fn serialize_fields(f: &File, t: &TsType) -> Block {
    Block::join_map(&t.fields, |field, _| {
        f.template("serialize_field")
            .set("field_name", field.field_name.clone())
            .set("serialize_func", field.field_type.get_serialize_func())
    })
}

fn deserialize_fields(f: &File, t: &TsType) -> Block {
    Block::join_map(&t.fields, |field, _| {
        f.template("deserialize_field")
            .set("field_name", field.field_name.clone())
            .set("deserialize_func", field.field_type.get_deserialize_func())
    })
}
