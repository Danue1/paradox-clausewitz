mod boolean;
mod comment;
mod datetime;
mod decimal;
mod encoding;
mod flavor;
mod integer;
mod line;
mod string;
mod whitespace;

use super::*;

#[test]
fn header_only() {
    assert_eq!(
        Ok(Tokens {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            token_list: vec![]
        }),
        lex("ck3text")
    )
}

#[test]
fn header_with_key() {
    assert_eq!(
        Ok(Tokens {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            token_list: vec![Token::Scalar(Scalar::Ident("key".to_owned()))]
        }),
        lex("ck3text\nkey")
    )
}

#[test]
fn header_with_key_and_value() {
    assert_eq!(
        Ok(Tokens {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            token_list: vec![
                Token::Scalar(Scalar::Ident("key".to_owned())),
                Token::Symbol(Symbol::Equal),
                Token::Scalar(Scalar::Ident("value".to_owned()))
            ]
        }),
        lex("ck3text\nkey=value")
    )
}
