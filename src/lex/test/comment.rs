use super::*;

#[test]
fn comment() {
    assert_eq!(Ok(("", ())), lex_comment("#"));
    assert_eq!(Ok(("", ())), lex_comment("#comment"));
    assert_eq!(Ok(("", ())), lex_comment("# comment"));
}
