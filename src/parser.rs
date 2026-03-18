use toml_edit::DocumentMut;

pub fn parse(content: &str) -> DocumentMut {
    content.parse::<DocumentMut>()
        .expect("Invalid TOML")
}