enum ITokenType {
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

struct Token {
    token_type: ITokenType,
}
