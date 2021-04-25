use super::*;

#[test]
fn ck3() {
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("eu4"));
}

#[test]
fn eu4() {
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("eu4"));
}
