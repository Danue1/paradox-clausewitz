use super::*;

#[test]
fn integer() {
    assert_eq!(Ok(("".as_bytes(), Numeric::Integer(1))), lex_numeric(b"1"));
    assert_eq!(
        Ok(("".as_bytes(), Numeric::Integer(12))),
        lex_numeric(b"12")
    );
    assert_eq!(
        Ok(("".as_bytes(), Numeric::Integer(123))),
        lex_numeric(b"123")
    );
}
