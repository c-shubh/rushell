use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string()));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        if c == ' ' || c == '\t' {
        } else if c == '\'' {
            self.scan_single_quoted_string();
        } else if c == '"' {
            self.scan_double_quoted_string();
        } else {
            self.scan_non_quoted_word();
        }
    }

    fn scan_double_quoted_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            eprintln!("unexpected EOF while looking for matching `\"'");
            return;
        }
        let value = self.source[(self.start + 1)..self.current].to_string();
        self.advance();
        self.tokens.push(Token::new(TokenType::String, value));
    }

    fn scan_single_quoted_string(&mut self) {
        while self.peek() != '\'' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            eprintln!("unexpected EOF while looking for matching `''");
            return;
        }
        let value = self.source[(self.start + 1)..self.current].to_string();
        self.advance();
        self.tokens.push(Token::new(TokenType::String, value));
    }

    fn scan_non_quoted_word(&mut self) {
        while self.peek() != ' ' && self.peek() != '\t' && !self.is_at_end() {
            self.advance();
        }
        let value = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(TokenType::String, value.to_string()));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let to_ret = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        to_ret
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::Scanner;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_single_word() {
        let input = "echo";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "echo".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_multiple_words() {
        let input = "echo hello world hey there";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "echo".to_string()));
        expected.push(Token::new(TokenType::String, "hello".to_string()));
        expected.push(Token::new(TokenType::String, "world".to_string()));
        expected.push(Token::new(TokenType::String, "hey".to_string()));
        expected.push(Token::new(TokenType::String, "there".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_1_single_quoted_string() {
        let input = "'hello world'";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "hello world".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_multiple_single_quoted_strings() {
        let input = "'hello world' 'how are you?' 'word'";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "hello world".to_string()));
        expected.push(Token::new(TokenType::String, "how are you?".to_string()));
        expected.push(Token::new(TokenType::String, "word".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_1_double_quoted_string() {
        let input = "\"hello world\"";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "hello world".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_multiple_double_quoted_strings() {
        let input = "\"hello world\" \"how are you?\" \"word\"";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens();

        let mut expected = Vec::new();
        expected.push(Token::new(TokenType::String, "hello world".to_string()));
        expected.push(Token::new(TokenType::String, "how are you?".to_string()));
        expected.push(Token::new(TokenType::String, "word".to_string()));
        expected.push(Token::new(TokenType::Eof, "".to_string()));

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }
}
