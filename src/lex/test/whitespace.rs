use super::*;

#[test]
fn whitespace() {
    assert_eq!(Ok(("text", ())), lex_whitespace1(" text"));
    assert_eq!(Ok(("text", ())), lex_whitespace1("  text"));
}
