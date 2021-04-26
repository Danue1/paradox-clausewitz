use super::*;

#[test]
fn comment() {
    assert_eq!(Ok(("".as_bytes(), ())), lex_comment(b"#"));
    assert_eq!(Ok(("".as_bytes(), ())), lex_comment(b"#comment"));
    assert_eq!(Ok(("".as_bytes(), ())), lex_comment(b"# comment"));
}
