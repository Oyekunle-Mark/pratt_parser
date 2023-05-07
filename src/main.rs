use std::{
    env::{self, Args},
    error::Error,
    process,
};

use crate::tokens::tokenize::build_tokens;

pub mod tokens;

const USAGE_MESSAGE: &str =
    "Usage: <cargo run>|<./executable> parse \"<expression to be evaluated>\"";
const ERROR_MESSAGE: &str = "Invalid command line arguments provided!";

fn main() {
    let mut args = env::args();
    let expr = validate_and_extract_expression(&mut args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("{}", USAGE_MESSAGE);
        process::exit(1);
    });

    let mut tokens = build_tokens(expr);

    while let Some(token) = tokens.next() {
        println!("{:#?}", token);
    }
}

fn validate_and_extract_expression(args: &mut Args) -> Result<String, Box<dyn Error>> {
    args.next(); // for the first command line argument which is the name of the file

    match args.next() {
        None => Err(ERROR_MESSAGE.into()),
        Some(arg) => match arg.as_str() {
            "parse" => match args.next() {
                Some(expr) => Ok(expr),
                _ => Err(ERROR_MESSAGE.into()),
            },
            _ => Err(ERROR_MESSAGE.into()),
        },
    }
}
