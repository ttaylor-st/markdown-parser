use crate::lexer::Token;

pub struct Renderer;

impl Renderer {
    pub fn generate(nodes: Vec<Token>) -> String {
        let mut html = String::new();

        for node in nodes {
            match node {
                Token::Header(text, level) => {
                    html.push_str(&format!("<h{}>{}</h{}>", level, text, level));
                }
                Token::Paragraph(text) => {
                    html.push_str(&format!("<p>{}</p>", text));
                }
            }
        }

        html
    }
}

// TODO: Add tests
