#[derive(Debug, PartialEq)]
pub enum Token {
    Header(String, usize),
    Paragraph(String),
}

pub struct Lexer<'a> {
    input: &'a str,
    _chars: std::str::Chars<'a>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            _chars: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let mut level = 0;
        while self.input[self.position..].starts_with('#') {
            level += 1;
            self.position += 1;
        }

        if level > 0 && self.input[self.position..].starts_with(' ') {
            self.position += 1;
            let start = self.position;
            while self.position < self.input.len() && !self.input[self.position..].starts_with('\n')
            {
                self.position += 1;
            }
            let value = self.input[start..self.position].to_string();
            return Some(Token::Header(value, level));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let input = "# Header 1\n## Header 2\n### Header 3\n";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1, Token::Header("Header 1".to_string(), 1));

        let token2 = lexer.next_token().unwrap();
        assert_eq!(token2, Token::Header("Header 2".to_string(), 2));

        let token3 = lexer.next_token().unwrap();
        assert_eq!(token3, Token::Header("Header 3".to_string(), 3));

        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_paragraph() {
        let input = "I am a paragraph.\n";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Paragraph("I am a paragraph.".to_string()));
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "# Header\nI am a paragraph.\n";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1, Token::Header("Header".to_string(), 1));

        let token2 = lexer.next_token().unwrap();
        assert_eq!(token2, Token::Paragraph("I am a paragraph.".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let input1 = "";
        let input2 = "\n";

        let mut lexer1 = Lexer::new(input1);
        let mut lexer2 = Lexer::new(input2);

        assert_eq!(lexer1.next_token(), None);
        assert_eq!(lexer2.next_token(), None);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "   \n# Header 1\n   \nI am a paragraph.\n";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1, Token::Header("Header 1".to_string(), 1));

        let token2 = lexer.next_token().unwrap();
        assert_eq!(token2, Token::Paragraph("I am a paragraph.".to_string()));
    }
}
