use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/pbxproj.pest"]
struct PbxprojParser;

pub fn parse_document(source: &str) -> Result<(), pest::error::Error<Rule>> {
    PbxprojParser::parse(Rule::file, source).map(|_| ())
}

