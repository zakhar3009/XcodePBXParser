use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/pbxproj.pest"]
struct PbxprojParser;

#[test]
fn single_entry_file_parsing() {
    let input = "{productName=Alamofire}";
    let result = PbxprojParser::parse(Rule::file, input).expect("file should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn multiple_entries_dictionary_parsing() {
    let input = "{ isa = PBXBuildFile; fileRef = ABCD1234; includes = (First, Second, 7); path = \"SomePath\" }";
    let result = PbxprojParser::parse(Rule::dictionary, input)
        .expect("dictionary should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn invalid_dictionary_parsing() {
    let input = "{productName=\"SomeApp\"";
    assert!(PbxprojParser::parse(Rule::dictionary, input).is_err());
}

#[test]
fn dictionary_with_comments_parsing() {
    let input = "{ /* leading */ key = value; value2 = other /* trailing */; }";
    let result = PbxprojParser::parse(Rule::dictionary, input)
        .expect("dictionary with comments should parse");
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
    let input = "7E7345D22EA6A18B0049F47D";
    let result = PbxprojParser::parse(Rule::identifier, input)
        .expect("identifier should parse");
    assert_eq!(result.as_str(), input);
}

#[test]
fn number_rule_parsing() {
    let input = "-12345";
    let result = PbxprojParser::parse(Rule::number, input).expect("number should parse");
    assert_eq!(result.as_str(), input);
}
