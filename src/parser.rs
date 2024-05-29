use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut nodes = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Header(text, level) => nodes.push(Token::Header(text, level)),
                Token::Paragraph(text) => nodes.push(Token::Paragraph(text)),
            }
        }

        nodes
    }
}

// TODO: Add tests
