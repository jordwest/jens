mod generator;

#[cfg(test)]
mod tests {
    use super::generator::tests::*;
    use insta::assert_snapshot_matches;

    #[test]
    fn test_derive_json_validator() {
        // A sample Json object schema
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

        assert_snapshot_matches!("test_derive_json_validator", generate(types).to_string());
    }
}
