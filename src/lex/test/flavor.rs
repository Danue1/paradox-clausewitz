use super::*;

#[test]
fn ck3() {
    assert_eq!(Ok(("", Flavor::Ck3)), lex_flavor("ck3"));
    assert_eq!(Ok(("", Flavor::Ck3)), lex_flavor("cK3"));
    assert_eq!(Ok(("", Flavor::Ck3)), lex_flavor("Ck3"));
    assert_eq!(Ok(("", Flavor::Ck3)), lex_flavor("CK3"));
}

#[test]
fn eu4() {
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("eu4"));
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("eU4"));
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("Eu4"));
    assert_eq!(Ok(("", Flavor::Eu4)), lex_flavor("EU4"));
}
