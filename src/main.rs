#![allow(non_snake_case)]

use anyhow::Context;
use clap::{Arg, Command};
use std::fs;

use XcodePBXParser::parse_document;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("pbxparse")
        .version("0.1.0")
        .author("XcodePBXParser contributors")
        .about("Parse Xcode .pbxproj files and output JSON")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the .pbxproj file to parse")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("credits")
                .long("credits")
                .help("Display credits information")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("credits") {
        println!("XcodePBXParser v0.1.0");
        println!("A Rust parser for Xcode .pbxproj files");
        println!("Author: Zakhar Litvinchuk");
        println!("Uses pest grammar for parsing and outputs structured JSON");
        return Ok(());
    }

    let file_path = matches
        .get_one::<String>("file")
        .context("File path is required. Use -f or --file to specify a file.")?;

    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let document = parse_document(&content)
        .with_context(|| format!("Failed to parse pbxproj: {}", file_path))?;

    let json = document.to_json();
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
