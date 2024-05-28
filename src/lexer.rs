#[derive(Debug, PartialEq)]
pub enum Token {
    Header(String),
    Paragraph(String),
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        // Check for end of input, prevents panics due to out of bounds access.
        if self.position >= self.input.len() {
            return None;
        }

        // Check for headers.
        if self.input[self.position..].starts_with("# ") {
            self.position += 2;
            let start = self.position;
            while self.position < self.input.len() && !self.input[self.position..].starts_with('\n')
            {
                self.position += 1;
            }
            let value = self.input[start..self.position].to_string();
            return Some(Token::Header(value));
        }

        // Check for paragraphs.
        let start = self.position;
        while self.position < self.input.len() && !self.input[self.position..].starts_with('\n') {
            self.position += 1;
        }
        let value = self.input[start..self.position].to_string();
        self.position += 1;
        Some(Token::Paragraph(value))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position..].starts_with(char::is_whitespace)
        {
            self.position += 1;
        }
    }
}
