#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    Number, // any number in decimal notation
    HexNumber, // any number in hex notation
    KeyWord,
    StringValue,
    Setter, // <-

    Multiply, // *
    Divide, // /
    Add, // +
    Substract, // -

    LeftParent, // (
    RightParent, // )

    EOF // end of file
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Number => "Number".to_string(),
            TokenType::HexNumber => "HexNumber".to_string(),
            TokenType::KeyWord => "KeyWord".to_string(),
            TokenType::StringValue => "StringValue".to_string(),
            TokenType::Setter => "<-".to_string(),
            TokenType::Multiply => "*".to_string(),
            TokenType::Divide => "/".to_string(),
            TokenType::Add => "+".to_string(),
            TokenType::Substract => "-".to_string(),
            TokenType::LeftParent => "(".to_string(),
            TokenType::RightParent => ")".to_string(),
            TokenType::EOF => "EOF".to_string()
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    pub token_type : TokenType,
    pub data: String
}

impl Token {
    pub fn new(token_type: TokenType, data: String) -> Token {
        Token {
            token_type: token_type,
            data: data
        }
    }
}
