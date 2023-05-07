#[derive(Clone, Copy, Debug)]
pub enum ITokenType {
    LParen,
    RParen,
    Comma,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Tilde,
    Bang,
    Question,
    Colon,
    Name,
    EOF,
}

impl ITokenType {
    pub fn get_char_equivalent(&self) -> Option<char> {
        match self {
            ITokenType::LParen => Some('('),
            ITokenType::RParen => Some(')'),
            ITokenType::Comma => Some(','),
            ITokenType::Assign => Some('='),
            ITokenType::Plus => Some('+'),
            ITokenType::Minus => Some('-'),
            ITokenType::Asterisk => Some('*'),
            ITokenType::Slash => Some('/'),
            ITokenType::Caret => Some('^'),
            ITokenType::Tilde => Some('~'),
            ITokenType::Bang => Some('!'),
            ITokenType::Question => Some('?'),
            ITokenType::Colon => Some(':'),
            ITokenType::Name | ITokenType::EOF => None,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: ITokenType,
    pub text: String,
}

impl Token {
    pub fn get_type(&self) -> ITokenType {
        self.token_type
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
    // char_to_token_type_map: HashMap<char, ITokenType>,
}
