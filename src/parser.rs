use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Node {
    Header(String),
    Paragraph(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Header(text) => nodes.push(Node::Header(text)),
                Token::Paragraph(text) => nodes.push(Node::Paragraph(text)),
            }
        }

        nodes
    }
}

// TODO: Add tests
