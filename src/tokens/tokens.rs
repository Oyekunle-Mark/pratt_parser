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
    pub index: usize,
    pub tokens: Vec<Token>,
}

impl Tokens {
    pub fn next(&mut self) -> Option<&Token> {
        let ret = self.tokens.get(self.index);
        self.index += 1;

        ret
    }

    pub fn peek_ahead(&mut self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }
}
