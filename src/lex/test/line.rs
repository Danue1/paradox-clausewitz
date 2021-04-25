use super::*;

#[test]
fn to_line() {
    assert_eq!(Ok(("", ())), lex_to_line("text"));
}
