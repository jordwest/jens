#[cfg(test)]
pub mod tests {
    use jens::Block;
    use jens_derive::Jens;
    pub enum Json {
        JsString,
        JsNumber,
        JsArray(Box<Json>),
        JsObject(String),
    }

    pub struct TsField {
        pub field_name: String,
        pub field_type: Json,
    }

    pub struct TsType {
        pub type_name: String,
        pub fields: Vec<TsField>,
    }

    #[derive(Jens)]
    #[template = "json_validator/template.jens"]
    struct Template {}

    impl Json {
        fn get_ts_type(&self) -> String {
            use Json::*;
            match self {
                JsString => "string".into(),
                JsNumber => "number".into(),
                JsArray(subtype) => format!("{}[]", subtype.get_ts_type()),
                JsObject(module_name) => format!("{}.T", module_name),
            }
        }

        fn get_serialize_func(&self) -> String {
            use Json::*;
            match self {
                JsString => format!("noop"),
                JsNumber => format!("noop"),
                JsArray(subtype) => format!("serialize_array({})", subtype.get_serialize_func()),
                JsObject(module_name) => format!("{}.serialize", module_name),
            }
        }

        fn get_deserialize_func(&self) -> String {
            use Json::*;
            match self {
                JsString => format!("deserialize_string"),
                JsNumber => format!("deserialize_number"),
                JsArray(subtype) => {
                    format!("deserialize_array({})", subtype.get_deserialize_func())
                }
                JsObject(module_name) => format!("{}.deserialize", module_name),
            }
        }
    }

    fn type_def_fields(t: &TsType) -> Block {
        Block::join_map(&t.fields, |field, _| {
            Template::type_def(field.field_name.clone(), field.field_type.get_ts_type())
        })
    }

    fn serialize_fields(t: &TsType) -> Block {
        Block::join_map(&t.fields, |field, _| {
            Template::serialize_field(
                field.field_name.clone(),
                field.field_type.get_serialize_func(),
            )
        })
    }

    fn deserialize_fields(t: &TsType) -> Block {
        Block::join_map(&t.fields, |field, _| {
            Template::deserialize_field(
                field.field_name.clone(),
                field.field_type.get_deserialize_func(),
            )
        })
    }

    pub fn generate(types: Vec<TsType>) -> Block {
        Template::main(Block::join_map(types, |t, _| {
            Template::type_module(
                t.type_name.clone(),
                type_def_fields(&t),
                serialize_fields(&t),
                deserialize_fields(&t),
            )
        }))
    }
}
