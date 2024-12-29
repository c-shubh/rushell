use std::fmt::Display;

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct ScannerError {
    pub message: String,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
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

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;

            let c = self.advance();
            if c == ' ' || c == '\t' {
            } else if c == '\'' {
                self.scan_single_quoted_string()?;
            } else if c == '"' {
                self.scan_double_quoted_string()?;
            } else {
                self.current -= 1;
                self.scan_non_quoted_word();
            }
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string()));
        Ok(&self.tokens)
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#Double-Quotes
    fn scan_double_quoted_string(&mut self) -> Result<(), ScannerError> {
        let mut value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            // The backslash retains its special meaning only when followed by
            // one of the following characters: ‘$’, ‘`’, ‘"’, ‘\’, or newline.
            // Within double quotes, backslashes that are followed by one of
            // these characters are removed.

            // current is at \
            if self.peek() == '\\' {
                // consume \
                self.advance();
                // check current
                match self.peek() {
                    '$' | '`' | '"' | '\\' => {
                        // only print matching character, and not backslash
                        value.push(self.advance());
                    }
                    'n' => {
                        value.push_str("\\n");
                        self.advance(); // consume n
                    }
                    _ => {
                        // Backslashes preceding characters without a special meaning are left unmodified.

                        // unknown escape sequence, print \ literally
                        value.push('\\');
                    }
                }
            } else {
                value.push(self.advance());
            }
        }
        if self.is_at_end() {
            return Err(ScannerError {
                message: "unexpected EOF while looking for matching `\"`".to_string(),
            });
        }
        self.advance(); // consume closing "
        self.tokens.push(Token::new(TokenType::String, value));
        Ok(())
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#Single-Quotes
    fn scan_single_quoted_string(&mut self) -> Result<(), ScannerError> {
        while !self.is_at_end() && self.peek() != '\'' {
            self.advance();
        }
        if self.is_at_end() {
            return Err(ScannerError {
                message: "unexpected EOF while looking for matching `''".to_string(),
            });
        }
        // exclude opening ' in substr
        let value = self.source[(self.start + 1)..self.current].to_string();
        self.advance(); // consume closing '
        self.tokens.push(Token::new(TokenType::String, value));
        Ok(())
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#index-metacharacter
    fn is_metacharacter(&self, c: char) -> bool {
        " \t\n|&;()<>".contains(c)
    }

    fn scan_non_quoted_word(&mut self) {
        let mut value = String::new();

        while !self.is_at_end() && !self.is_metacharacter(self.peek()) {
            // Each of the shell metacharacters has special meaning to the shell
            // and must be quoted (escape character) if it is to represent itself.

            // current is at \
            if self.peek() == '\\' {
                // consume \
                self.advance();
            }
            value.push(self.advance());
        }
        self.tokens.push(Token::new(TokenType::String, value));
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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = vec![
            Token::new(TokenType::String, "echo".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        println!("actual: {:#?}", tokens);
        println!("expected: {:#?}", expected);

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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "echo".to_string()),
            Token::new(TokenType::String, "hello".to_string()),
            Token::new(TokenType::String, "world".to_string()),
            Token::new(TokenType::String, "hey".to_string()),
            Token::new(TokenType::String, "there".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::String, "how are you?".to_string()),
            Token::new(TokenType::String, "word".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
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
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::String, "how are you?".to_string()),
            Token::new(TokenType::String, "word".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_single_quoted_string_escape_char_does_nothing() {
        let input = "'\\' 'hey\\nthere' '\\\\' '\\ '";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "\\".to_string()),
            Token::new(TokenType::String, "hey\\nthere".to_string()),
            Token::new(TokenType::String, "\\\\".to_string()),
            Token::new(TokenType::String, "\\ ".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_double_quoted_string_escape_char_works() {
        let input = "\"\\$\" \"\\`\" \"\\\"\" \"\\\\\" \"\\n\"";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "$".to_string()),
            Token::new(TokenType::String, "`".to_string()),
            Token::new(TokenType::String, "\"".to_string()),
            Token::new(TokenType::String, "\\".to_string()),
            Token::new(TokenType::String, "\\n".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        println!("actual: {:#?}", tokens);
        println!("expected: {:#?}", expected);

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in zip(expected, tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_escaped_double_quotes_in_double_quoted_string() {
        let input = "\"hello \\\"world\\\"\"";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello \"world\"".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in expected.iter().zip(tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_backslash_to_escape_itself() {
        let input = "\"hello\\\\world\"";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello\\world".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in expected.iter().zip(tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_backslash_in_unquoted_string() {
        let input = "echo \\'\\\"example shell\\\"\\' hello\\\\nworld hey\\nthere";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "echo".to_string()),
            Token::new(TokenType::String, "'\"example".to_string()),
            Token::new(TokenType::String, "shell\"'".to_string()),
            Token::new(TokenType::String, "hello\\nworld".to_string()),
            Token::new(TokenType::String, "heynthere".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];

        println!("actual: {:#?}", tokens);
        println!("expected: {:#?}", expected);

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in expected.iter().zip(tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_backslash_metacharacter_in_unquoted_string() {
        let input = "hello\\ wor\\>ld";
        let mut scanner = Scanner::new(input.to_string());
        let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

        let expected = [
            Token::new(TokenType::String, "hello wor>ld".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in expected.iter().zip(tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }
}
