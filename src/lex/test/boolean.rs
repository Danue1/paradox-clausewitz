use super::*;

macro_rules! eof {
    () => {
        nom::Err::Error(LexError::Nom(nom::error::ErrorKind::Eof))
    };
}

#[test]
fn yes() {
    assert_eq!(Ok(("", Boolean::Yes)), lex_boolean("yes"));
    assert_eq!(Err(eof!()), lex_boolean("yesa"));
}

#[test]
fn no() {
    assert_eq!(Ok(("", Boolean::No)), lex_boolean("no"));
    assert_eq!(Err(eof!()), lex_boolean("noa"));
}

#[test]
fn r#true() {
    assert_eq!(Ok(("", Boolean::True)), lex_boolean("true"));
    assert_eq!(Err(eof!()), lex_boolean("truea"));
}

#[test]
fn r#false() {
    assert_eq!(Ok(("", Boolean::False)), lex_boolean("false"));
    assert_eq!(Err(eof!()), lex_boolean("falsea"));
}
