use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/pbxproj.pest"]
struct PbxprojParser;

#[test]
fn single_entry_file_parsing() {
    let input = "{isa = PBXBuildFile; fileRef = 8E320BAFFB7B48F19CAA325CB834998A}";
    let result = PbxprojParser::parse(Rule::file, input).expect("file should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn dictionary_rule_parsing() {
    let input = "{isa = PBXBuildFile; fileRef = 8E320BAFFB7B48F19CAA325CB834998A /* AboutFeature.swift */}";
    let result = PbxprojParser::parse(Rule::dictionary, input)
        .expect("dictionary should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn array_rule_parsing() {
    let input = "(words, numbers, 27, \"quoted\")";
    let result = PbxprojParser::parse(Rule::array, input).expect("array should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn identifier_rule_parsing() {
    let input = "19FAF625E510475C8F2D3C08B89BE3DD";
    let result = PbxprojParser::parse(Rule::identifier, input)
        .expect("identifier should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn number_rule_parsing() {
    let input = "77";
    let result = PbxprojParser::parse(Rule::number, input).expect("number should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn string_rule_parsing() {
    let input = "\"Products.storekit\"";
    let result = PbxprojParser::parse(Rule::string, input).expect("string should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn pair_rule_parsing() {
    let input = "isa = PBXBuildFile";
    let result = PbxprojParser::parse(Rule::pair, input).expect("pair should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn key_rule_parsing() {
    let input = "fileRef";
    let result = PbxprojParser::parse(Rule::key, input).expect("key should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn key_as_string_parsing() {
    let input = "\"My Key\"";
    let result = PbxprojParser::parse(Rule::key, input).expect("key as string should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn pair_sequence_parsing() {
    let input = "isa = PBXBuildFile; fileRef = 8E320BAFFB7B48F19CAA325CB834998A";
    let result = PbxprojParser::parse(Rule::pair_sequence, input).expect("pair_sequence should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn pair_entry_parsing() {
    let input = "  archiveVersion = 1  ";
    let result = PbxprojParser::parse(Rule::pair_entry, input).expect("pair_entry should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn value_list_parsing() {
    let input = "item1, item2, item3";
    let result = PbxprojParser::parse(Rule::value_list, input).expect("value_list should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn value_entry_parsing() {
    let input = "  7E3D661B2EA2D7C900F28288  ";
    let result = PbxprojParser::parse(Rule::value_entry, input).expect("value_entry should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn block_comment_parsing() {
    let input = "/* Begin PBXBuildFile section */";
    let result = PbxprojParser::parse(Rule::BLOCK_COMMENT, input).expect("block comment should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn line_comment_parsing() {
    let input = "// !$*UTF8*$!\n";
    let result = PbxprojParser::parse(Rule::LINE_COMMENT, input).expect("line comment should parse");
    assert_eq!(result.as_str(), input);
}
