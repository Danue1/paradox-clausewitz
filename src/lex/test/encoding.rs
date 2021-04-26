use super::*;

#[test]
fn txt() {
    assert_eq!(Ok(("".as_bytes(), Encoding::Text)), lex_encoding(b"txt"));
}

#[test]
fn text() {
    assert_eq!(Ok(("".as_bytes(), Encoding::Text)), lex_encoding(b"text"));
}

#[test]
fn binary() {
    assert_eq!(
        Ok(("".as_bytes(), Encoding::Binary)),
        lex_encoding(b"binary")
    );
}
