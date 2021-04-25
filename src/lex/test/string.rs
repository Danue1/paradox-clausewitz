use super::*;

#[test]
fn string() {
    assert_eq!(Ok(("", "string".to_owned())), lex_string("\"string\""));
}
