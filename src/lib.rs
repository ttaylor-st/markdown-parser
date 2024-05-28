mod lexer;
mod parser;
mod renderer;

use lexer::Lexer;
use parser::Parser;
use renderer::Renderer;

pub fn parse_markdown(input: &str) -> String {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let nodes = parser.parse();
    Renderer::generate(nodes)
}

// TODO: Add WASM bindings

// TODO: Add tests
