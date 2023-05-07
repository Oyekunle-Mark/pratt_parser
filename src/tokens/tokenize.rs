use std::collections::HashMap;

use super::tokens::{ITokenType, Token, Tokens};

fn build_tokens(expression: String) -> Tokens {
    let mut tokens = vec![];
    let char_to_token_type_map = HashMap::from([
        ('(', ITokenType::LParen),
        (')', ITokenType::RParen),
        (',', ITokenType::Comma),
        ('=', ITokenType::Assign),
        ('+', ITokenType::Plus),
        ('-', ITokenType::Minus),
        ('*', ITokenType::Asterisk),
        ('/', ITokenType::Slash),
        ('^', ITokenType::Caret),
        ('~', ITokenType::Tilde),
        ('!', ITokenType::Bang),
        ('?', ITokenType::Question),
        (':', ITokenType::Colon),
    ]);

    let char_vec: Vec<char> = expression.chars().collect();
    let mut name = String::from("");

    for i in 0..char_vec.len() {
        let c = &char_vec[i];
        if char_to_token_type_map.contains_key(c) {
            if !name.is_empty() {
                tokens.push(Token {
                    token_type: ITokenType::Name,
                    text: name.clone(),
                });
                name.clear();
            }

            tokens.push(Token {
                token_type: *char_to_token_type_map.get(c).unwrap(),
                text: c.to_string(),
            })
        } else if c.is_alphabetic() {
            name.push(*c);
        } else {
            panic!("Invalid token: {}", c);
        }
    }

    if !name.is_empty() {
        tokens.push(Token {
            token_type: ITokenType::Name,
            text: name.clone(),
        });
        name.clear();
    }

    Tokens { tokens }
}
