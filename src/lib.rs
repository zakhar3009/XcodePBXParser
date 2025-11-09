use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/pbxproj.pest"]
struct PbxprojParser;

#[derive(Debug, Clone, PartialEq)]
pub struct PbxDocument {
    pub root: PbxValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PbxValue {
    Dictionary(Vec<PbxEntry>),
    Array(Vec<PbxValue>),
    String(String),
    Identifier(String),
    Number(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PbxEntry {
    pub key: String,
    pub value: PbxValue,
}

pub fn parse_document(source: &str) -> Result<(), pest::error::Error<Rule>> {
    PbxprojParser::parse(Rule::file, source).map(|_| ())
}

