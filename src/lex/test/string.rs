use super::*;

#[test]
fn string() {
    assert_eq!(
        Ok(("".as_bytes(), "string".to_owned())),
        lex_string(b"\"string\"")
    );
}
