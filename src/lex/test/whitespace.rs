use super::*;

#[test]
fn whitespace() {
    assert_eq!(Ok(("text".as_bytes(), ())), lex_whitespace1(b" text"));
    assert_eq!(Ok(("text".as_bytes(), ())), lex_whitespace1(b"  text"));
}
