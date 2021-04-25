use super::*;

#[test]
fn text() {
    assert_eq!(Ok(("", Encoding::Text)), lex_encoding("text"));
}

#[test]
fn binary() {
    assert_eq!(Ok(("", Encoding::Binary)), lex_encoding("binary"));
}
