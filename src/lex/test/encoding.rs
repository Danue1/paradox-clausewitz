use super::*;

#[test]
fn txt() {
    assert_eq!(Ok(("", Encoding::Text)), lex_encoding("txt"));
}

#[test]
fn text() {
    assert_eq!(Ok(("", Encoding::Text)), lex_encoding("text"));
}

#[test]
fn binary() {
    assert_eq!(Ok(("", Encoding::Binary)), lex_encoding("binary"));
}
