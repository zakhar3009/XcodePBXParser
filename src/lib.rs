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
    #[error("expected child value")]
    MissingChild,
}

impl PbxDocument {
    pub fn new(root: PbxValue) -> Self {
        Self { root }
    }
}

pub fn parse_document(source: &str) -> Result<PbxDocument, PbxParseError> {
    let mut pairs = PbxprojParser::parse(Rule::file, source)?;
    let root_pair = pairs.next().ok_or(PbxParseError::MissingRoot)?;
    let root_value = parse_rule(root_pair)?;
    Ok(PbxDocument::new(root_value))
}

fn parse_rule(pair: Pair<Rule>) -> Result<PbxValue, PbxParseError> {
    match pair.as_rule() {
        Rule::file => {
            let mut inner = pair.into_inner();
            let child = inner.next().ok_or(PbxParseError::MissingRoot)?;
            parse_rule(child)
        }
        Rule::dictionary => parse_dictionary(pair),
        Rule::array => parse_array(pair),
        Rule::string => Ok(PbxValue::String(unquote(pair.as_str()))),
        Rule::identifier => Ok(PbxValue::Identifier(pair.as_str().to_string())),
        Rule::number => Ok(PbxValue::Number(pair.as_str().to_string())),
        Rule::value => {
            let mut inner = pair.into_inner();
            let child = inner.next().ok_or(PbxParseError::MissingChild)?;
            parse_rule(child)
        }
        other => Err(PbxParseError::UnexpectedRule(other)),
    }
}

fn parse_dictionary(pair: Pair<Rule>) -> Result<PbxValue, PbxParseError> {
    let mut entries = Vec::new();
    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::pair_sequence => entries.extend(parse_pair_sequence(child)?),
            Rule::pair => entries.push(parse_pair(child)?),
            Rule::pair_entry => entries.push(parse_pair_entry(child)?),
            Rule::skip | Rule::COMMENT => {}
            other => return Err(PbxParseError::UnexpectedRule(other)),
        }
    }
    Ok(PbxValue::Dictionary(entries))
}

fn parse_pair_sequence(pair: Pair<Rule>) -> Result<Vec<PbxEntry>, PbxParseError> {
    let mut entries = Vec::new();
    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::pair_entry => entries.push(parse_pair_entry(child)?),
            Rule::skip | Rule::COMMENT => {}
            other => return Err(PbxParseError::UnexpectedRule(other)),
        }
    }
    Ok(entries)
}

fn parse_pair_entry(pair: Pair<Rule>) -> Result<PbxEntry, PbxParseError> {
    for child in pair.into_inner() {
        if child.as_rule() == Rule::pair {
            return parse_pair(child);
        }
    }
    Err(PbxParseError::MissingChild)
}

fn parse_pair(pair: Pair<Rule>) -> Result<PbxEntry, PbxParseError> {
    let mut inner = pair.into_inner();
    let key_pair = inner.next().ok_or(PbxParseError::MissingChild)?;
    let key = parse_key(key_pair)?;
    let value_pair = inner.next().ok_or(PbxParseError::MissingChild)?;
    let value = parse_rule(value_pair)?;
    Ok(PbxEntry { key, value })
}

fn parse_key(pair: Pair<Rule>) -> Result<String, PbxParseError> {
    let mut inner = pair.into_inner();
    let token = inner.next().ok_or(PbxParseError::MissingChild)?;
    match token.as_rule() {
        Rule::string => Ok(unquote(token.as_str())),
        Rule::identifier => Ok(token.as_str().to_string()),
        other => Err(PbxParseError::UnexpectedRule(other)),
    }
}

fn parse_array(pair: Pair<Rule>) -> Result<PbxValue, PbxParseError> {
    let mut values = Vec::new();
    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::value_list => values.extend(parse_value_list(child)?),
            Rule::value => values.push(parse_rule(child)?),
            Rule::value_entry => values.push(parse_value_entry(child)?),
            Rule::skip | Rule::COMMENT => {}
            other => return Err(PbxParseError::UnexpectedRule(other)),
        }
    }
    Ok(PbxValue::Array(values))
}

fn parse_value_list(pair: Pair<Rule>) -> Result<Vec<PbxValue>, PbxParseError> {
    let mut values = Vec::new();
    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::value_entry => values.push(parse_value_entry(child)?),
            Rule::value => values.push(parse_rule(child)?),
            Rule::skip | Rule::COMMENT => {}
            other => return Err(PbxParseError::UnexpectedRule(other)),
        }
    }
    Ok(values)
}

fn parse_value_entry(pair: Pair<Rule>) -> Result<PbxValue, PbxParseError> {
    for child in pair.into_inner() {
        if child.as_rule() == Rule::value {
            return parse_rule(child);
        }
    }
    Err(PbxParseError::MissingChild)
}

fn unquote(s: &str) -> String {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}

