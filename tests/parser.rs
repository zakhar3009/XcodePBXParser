use XcodePBXParser::{parse_document, PbxEntry, PbxValue};

#[test]
fn parses_nested_dictionary() {
    let input = "{ outer = { inner = value; }; flag = YES; quoted = \"Value\"; }";
    let document = parse_document(input).expect("document parses");
    let expected = PbxValue::Dictionary(vec![
        PbxEntry {
            key: "outer".into(),
            value: PbxValue::Dictionary(vec![PbxEntry {
                key: "inner".into(),
                value: PbxValue::Identifier("value".into()),
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

