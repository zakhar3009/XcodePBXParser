use XcodePBXParser::{parse_document, PbxEntry, PbxValue};

#[test]
fn parses_dictionary_with_array() {
    let input = "{ outer = { inner = (First, Second, 7); }; flag = YES; quoted = \"Value\"; }";
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
            }]),
        },
        PbxEntry {
            key: "flag".into(),
            value: PbxValue::Identifier("YES".into()),
        },
        PbxEntry {
            key: "quoted".into(),
            value: PbxValue::String("Value".into()),
        },
    ]);

    assert_eq!(document.root, expected);
}

