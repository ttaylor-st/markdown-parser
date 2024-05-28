use crate::parser::Node;

pub struct Renderer;

impl Renderer {
    pub fn generate(nodes: Vec<Node>) -> String {
        let mut html = String::new();

        for node in nodes {
            match node {
                Node::Header(text, level) => {
                    html.push_str(&format!("<h{}>{}</h{}>", level, text, level));
                }
                Node::Paragraph(text) => {
                    html.push_str(&format!("<p>{}</p>", text));
                }
            }
        }

        html
    }
}

// TODO: Add tests
