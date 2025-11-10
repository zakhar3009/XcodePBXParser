#![allow(non_snake_case)]

fn main() {
    let sample = "{productName=Sample}";
    match XcodePBXParser::parse_document(sample) {
        Ok(document) => println!("Sample parsed successfully: {:?}", document.root),
        Err(err) => {
            eprintln!("Failed to parse sample: {err}");
            std::process::exit(1);
        }
    }
}
