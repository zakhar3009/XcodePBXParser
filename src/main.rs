fn main() {
    let sample = "{productName=Sample}";
    match XcodePBXParser::parse_document(sample) {
        Ok(_) => println!("Sample parsed successfully."),
        Err(err) => {
            eprintln!("Failed to parse sample: {err}");
            std::process::exit(1);
        }
    }
}
