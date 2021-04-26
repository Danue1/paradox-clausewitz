use super::*;

#[test]
fn to_line() {
    assert_eq!(Ok(("".as_bytes(), ())), lex_to_line(b"text"));
}
