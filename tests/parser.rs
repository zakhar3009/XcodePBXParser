use XcodePBXParser::{parse_document, PbxValue};

#[test]
fn file_root_is_dictionary() {
    let document = parse_document("{key=value;}").expect("document parses");
    match document.root {
        PbxValue::Dictionary(_) => {}
        other => panic!("expected dictionary root, got {:?}", other),
    }
}

