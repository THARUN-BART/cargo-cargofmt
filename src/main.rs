mod parser;
mod formatter;
use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).expect("Provide file path");
    let content = fs::read_to_string(&file)
        .expect("Failed to read file");
    let mut doc = parser::parse(&content);
    formatter::format(&mut doc);
    fs::write(&file, doc.to_string())
        .expect("Failed to write file");
    println!("Formatted {}", file);

}