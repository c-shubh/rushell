#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof,
    String,
}

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String) -> Self {
        Token { type_, lexeme }
    }
}
