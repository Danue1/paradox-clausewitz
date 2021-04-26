use super::*;

#[test]
fn ck3() {
    assert_eq!(Ok(("".as_bytes(), Flavor::Ck3)), lex_flavor(b"ck3"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Ck3)), lex_flavor(b"cK3"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Ck3)), lex_flavor(b"Ck3"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Ck3)), lex_flavor(b"CK3"));
}

#[test]
fn eu4() {
    assert_eq!(Ok(("".as_bytes(), Flavor::Eu4)), lex_flavor(b"eu4"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Eu4)), lex_flavor(b"eU4"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Eu4)), lex_flavor(b"Eu4"));
    assert_eq!(Ok(("".as_bytes(), Flavor::Eu4)), lex_flavor(b"EU4"));
}
