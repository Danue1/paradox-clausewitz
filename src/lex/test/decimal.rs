use super::*;

#[test]
fn decimal() {
    assert_eq!(
        Ok(("".as_bytes(), Numeric::Decimal(1.0))),
        lex_numeric(b"1.0")
    );
    assert_eq!(
        Ok(("".as_bytes(), Numeric::Decimal(12.0))),
        lex_numeric(b"12.0")
    );
    assert_eq!(
        Ok(("".as_bytes(), Numeric::Decimal(123.0))),
        lex_numeric(b"123.0")
    );
}
