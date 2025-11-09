use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum PbxParseError {
    #[error("parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("missing root dictionary")]
    MissingRoot,
    #[error("unexpected rule {0:?}")]
    UnexpectedRule(Rule),
}

impl PbxDocument {
    pub fn new(root: PbxValue) -> Self {
        Self { root }
    }
}

pub fn parse_document(source: &str) -> Result<PbxDocument, PbxParseError> {
    let mut pairs = PbxprojParser::parse(Rule::file, source)?;
    let root_pair = pairs
        .next()
        .ok_or(PbxParseError::MissingRoot)?;
    let root_value = extract_dictionary(root_pair)?;
    Ok(PbxDocument::new(root_value))
}

fn extract_dictionary(pair: Pair<Rule>) -> Result<PbxValue, PbxParseError> {
    if pair.as_rule() != Rule::file {
        return Err(PbxParseError::UnexpectedRule(pair.as_rule()));
    }
    let mut inner_pairs = pair.into_inner();
    let inner = inner_pairs.next().ok_or(PbxParseError::MissingRoot)?;
    if inner.as_rule() == Rule::dictionary {
        Ok(PbxValue::Dictionary(Vec::new()))
    } else {
        Err(PbxParseError::UnexpectedRule(inner.as_rule()))
    }
}

