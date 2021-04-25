use crate::{Encoding, Flavor, Scalar};

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub flavor: Flavor,
    pub encoding: Encoding,
    pub token_list: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Symbol(Symbol),
    Scalar(Scalar),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    LeftChevron,       // <
    RightChevron,      // >
    LeftChevronEqual,  // <=
    RightChevronEqual, // >=
    Equal,             // =
    Exclamation,       // !
    Colon,             // :
    Dot,               // .
    At,                // @
}
