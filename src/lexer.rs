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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let input = "# Header\n";
        let mut lexer = Lexer::new(input);
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Header("Header".to_string()));
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
        assert_eq!(token1, Token::Header("Header".to_string()));

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
        assert_eq!(token1, Token::Header("Header 1".to_string()));

        let token2 = lexer.next_token().unwrap();
        assert_eq!(token2, Token::Paragraph("I am a paragraph.".to_string()));
    }
}
