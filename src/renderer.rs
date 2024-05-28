use crate::parser::Node;

pub struct Renderer;

impl Renderer {
    pub fn generate(nodes: Vec<Node>) -> String {
        let mut html = String::new();

        for node in nodes {
            match node {
                Node::Header(text) => {
                    html.push_str(&format!("<h1>{}</h1>", text));
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
