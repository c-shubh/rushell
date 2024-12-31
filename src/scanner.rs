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
                let value = self.scan_single_quoted_string(self.start)?;
                self.tokens.push(Token::new(TokenType::String, value.1));
                self.current = value.0;
            } else if c == '"' {
                let value = self.scan_double_quoted_string(self.start)?;
                self.tokens.push(Token::new(TokenType::String, value.1));
                self.current = value.0;
            } else {
                self.current -= 1;
                self.scan_unquoted_word()?;
            }
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string()));
        Ok(&self.tokens)
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#Double-Quotes
    fn scan_double_quoted_string(&self, start: usize) -> Result<(usize, String), ScannerError> {
        let mut end_at: Option<usize> = None; // points to index of closing "
        let mut value = String::new();
        // `start` is "
        // start iterating from `start+1`
        let mut iter = self.source.chars().skip(start + 1).enumerate();
        while let Some((i, c)) = iter.next() {
            // stop once we find closing "
            if c == '"' {
                end_at = Some(start + i + 1);
                break;
            }

            // The backslash retains its special meaning only when followed by
            // one of the following characters: ‘$’, ‘`’, ‘"’, ‘\’, or newline.
            // Within double quotes, backslashes that are followed by one of
            // these characters are removed.

            // current is at \
            if c == '\\' {
                // consume \
                let (_, c) = iter.next().unwrap_or((i + 1, '\0'));
                // check current
                match c {
                    '$' | '`' | '"' | '\\' => {
                        // only print matching character, and not backslash
                        value.push(c);
                    }
                    'n' => {
                        value.push_str("\\n");
                    }
                    _ => {
                        // Backslashes preceding characters without a special meaning are left unmodified.

                        // unknown escape sequence, print \ literally
                        value.push('\\');
                    }
                }
            } else {
                value.push(c);
            }
        }
        if end_at.is_none() {
            return Err(ScannerError {
                message: "unexpected EOF while looking for matching `\"'".to_string(),
            });
        }
        let end_at = end_at.unwrap();

        // exclude opening " in substr
        // let value = self.source[(start + 1)..end_at].to_string();
        Ok((end_at + 1, value))
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#Single-Quotes
    fn scan_single_quoted_string(&self, start: usize) -> Result<(usize, String), ScannerError> {
        let mut end_at: Option<usize> = None; // points to index of closing '
        for (i, c) in self.source.chars().skip(start + 1).enumerate() {
            // stop once we find closing '
            if c == '\'' {
                end_at = Some(start + i + 1);
                break;
            }
        }
        if end_at.is_none() {
            return Err(ScannerError {
                message: "unexpected EOF while looking for matching `''".to_string(),
            });
        }
        let end_at = end_at.unwrap();

        // exclude opening ' in substr
        let value = self.source[(start + 1)..end_at].to_string();
        Ok((end_at + 1, value))
    }

    /// https://www.gnu.org/software/bash/manual/bash.html#index-metacharacter
    fn is_metacharacter(&self, c: char) -> bool {
        " \t\n|&;()<>".contains(c)
    }

    fn scan_unquoted_word(&mut self) -> Result<(), ScannerError> {
        let mut value = String::new();
        while !self.is_at_end() && !self.is_metacharacter(self.peek()) {
            // current is at \
            if self.peek() == '\\' {
                // consume \
                self.advance();
                if self.is_metacharacter(self.peek()) || self.peek() == '\\' || self.peek() == '\''
                {
                    value.push(self.advance());
                }
            } else if self.peek() == '\'' {
                let ret = self.scan_single_quoted_string(self.current)?;
                value.push_str(&ret.1);
                self.current = ret.0;
            } else {
                value.push(self.advance());
            }
        }
        self.tokens.push(Token::new(TokenType::String, value));
        Ok(())
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
        let expected = vec![
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
    }

    #[test]
    fn test_multiple_double_quoted_strings() {
        let input = "\"hello world\" \"how are you?\" \"word\"";
        let expected = vec![
            Token::new(TokenType::String, "hello world".to_string()),
            Token::new(TokenType::String, "how are you?".to_string()),
            Token::new(TokenType::String, "word".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
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
        let expected = vec![
            Token::new(TokenType::String, "$".to_string()),
            Token::new(TokenType::String, "`".to_string()),
            Token::new(TokenType::String, "\"".to_string()),
            Token::new(TokenType::String, "\\".to_string()),
            Token::new(TokenType::String, "\\n".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
    }

    #[test]
    fn test_escaped_double_quotes_in_double_quoted_string() {
        let input = "\"hello \\\"world\\\"\"";
        let expected = vec![
            Token::new(TokenType::String, "hello \"world\"".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
    }

    #[test]
    fn test_double_quoted_backslash_to_escape_itself() {
        // input:
        // "hello\\world"
        let input = "\"hello\\\\world\"";
        let expected = vec![
            Token::new(TokenType::String, "hello\\world".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
    }

    #[test]
    fn test_backslash_in_unquoted_string() {
        // input:
        // echo \'\"example shell\"\' hello\\nworld hey\nthere
        // output tokens:
        // echo
        // '"example
        // shell"'
        // hello\nworld
        // heynthere

        let input = "echo \\'\\\"example shell\\\"\\' hello\\\\nworld hey\\nthere";
        let expected = vec![
            Token::new(TokenType::String, "echo".to_string()),
            Token::new(TokenType::String, "'\"example".to_string()),
            Token::new(TokenType::String, "shell\"'".to_string()),
            Token::new(TokenType::String, "hello\\nworld".to_string()),
            Token::new(TokenType::String, "heynthere".to_string()),
            Token::new(TokenType::Eof, "".to_string()),
        ];
        test(input.to_string(), expected);
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

    fn test(input: String, expected: Vec<Token>) {
        let mut scanner = Scanner::new(input.to_string());
        let tokens = scanner.scan_tokens().unwrap();

        println!("actual: {:#?}", tokens);
        println!("expected: {:#?}", expected);

        assert_eq!(tokens.len(), expected.len());

        for (expected_token, actual_token) in expected.iter().zip(tokens) {
            assert_eq!(actual_token.type_, expected_token.type_);
            assert_eq!(actual_token.lexeme, expected_token.lexeme);
        }
    }

    #[test]
    fn test_adjacent_unquoted_double_quoted() {
        test(
            "hey\"there how\"".to_string(),
            vec![
                Token::new(TokenType::String, "heythere how".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
    #[test]
    fn test_adjacent_unquoted_single_quoted() {
        test(
            "hey'there how'".to_string(),
            vec![
                Token::new(TokenType::String, "heythere how".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
    #[test]
    fn test_adjacent_double_quoted_unquoted() {
        test(
            "\"hey there\"how".to_string(),
            vec![
                Token::new(TokenType::String, "hey therehow".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
    #[test]
    fn test_adjacent_single_quoted_unquoted() {
        test(
            "'hey there'how".to_string(),
            vec![
                Token::new(TokenType::String, "hey therehow".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
    #[test]
    fn test_adjacent_double_quoted_single_quoted() {
        test(
            "\"hey there\"'how are'".to_string(),
            vec![
                Token::new(TokenType::String, "hey therehow are".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
    #[test]
    fn test_adjacent_single_quoted_double_quoted() {
        test(
            "'hey there'\"how are\"".to_string(),
            vec![
                Token::new(TokenType::String, "hey therehow are".to_string()),
                Token::new(TokenType::Eof, "".to_string()),
            ],
        );
    }
}
