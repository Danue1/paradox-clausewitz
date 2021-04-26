use super::*;

macro_rules! eof {
    () => {
        nom::Err::Error(LexError::Nom(nom::error::ErrorKind::Eof))
    };
}

#[test]
fn yes() {
    assert_eq!(Ok(("".as_bytes(), Boolean::Yes)), lex_boolean(b"yes"));
    assert_eq!(Err(eof!()), lex_boolean(b"yesa"));
}

#[test]
fn no() {
    assert_eq!(Ok(("".as_bytes(), Boolean::No)), lex_boolean(b"no"));
    assert_eq!(Err(eof!()), lex_boolean(b"noa"));
}

#[test]
fn r#true() {
    assert_eq!(Ok(("".as_bytes(), Boolean::True)), lex_boolean(b"true"));
    assert_eq!(Err(eof!()), lex_boolean(b"truea"));
}

#[test]
fn r#false() {
    assert_eq!(Ok(("".as_bytes(), Boolean::False)), lex_boolean(b"false"));
    assert_eq!(Err(eof!()), lex_boolean(b"falsea"));
}
