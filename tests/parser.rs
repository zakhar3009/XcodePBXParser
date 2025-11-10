use serde_json::json;
use XcodePBXParser::{parse_document, PbxEntry, PbxValue};

#[test]
fn parses_dictionary_with_array() {
    let input = "{ outer = { inner = (First, Second, 7); }; flag = YES /* Begin PBXBuildFile section */; quoted = \"Value\"; }";
    let document = parse_document(input).expect("document parses");
    let expected = PbxValue::Dictionary(vec![
        PbxEntry {
            key: "outer".into(),
            value: PbxValue::Dictionary(vec![PbxEntry {
                key: "inner".into(),
                value: PbxValue::Array(vec![
                    PbxValue::Identifier("First".into()),
                    PbxValue::Identifier("Second".into()),
                    PbxValue::Number("7".into()),
                ]),
                comment: None,
            }]),
            comment: None,
        },
        PbxEntry {
            key: "flag".into(),
            value: PbxValue::Identifier("YES".into()),
            comment: Some("Begin PBXBuildFile section".into()),
        },
        PbxEntry {
            key: "quoted".into(),
            value: PbxValue::String("Value".into()),
            comment: None,
        },
    ]);

    assert_eq!(document.root, expected);

    let serialized = serde_json::to_value(&document.root).expect("json serialization");
    let expected_json = json!({
        "type": "dictionary",
        "value": [
            {
                "key": "outer",
                "value": {
                    "type": "dictionary",
                    "value": [
                        {
                            "key": "inner",
                            "value": {
                                "type": "array",
                                "value": [
                                    { "type": "identifier", "value": "First" },
                                    { "type": "identifier", "value": "Second" },
                                    { "type": "number", "value": "7" }
                                ]
                            }
                        }
                    ]
                }
            },
            {
                "key": "flag",
                "value": { "type": "identifier", "value": "YES" },
                "comment": "Begin PBXBuildFile section"
            },
            {
                "key": "quoted",
                "value": { "type": "string", "value": "Value" }
            }
        ]
    });
    assert_eq!(serialized, expected_json);
}

