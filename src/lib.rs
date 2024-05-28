use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod renderer;

use lexer::Lexer;
use parser::Parser;
use renderer::Renderer;

#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let nodes = parser.parse();
    Renderer::generate(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let input = "# Header\nThis is a paragraph.";
        let expected_html = "<h1>Header</h1><p>This is a paragraph.</p>";
        assert_eq!(parse_markdown(input), expected_html);
    }
}
