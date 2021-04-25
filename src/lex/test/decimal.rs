use super::*;

#[test]
fn decimal() {
    assert_eq!(Ok(("", Numeric::Decimal(1.0))), lex_numeric("1.0"));
    assert_eq!(Ok(("", Numeric::Decimal(12.0))), lex_numeric("12.0"));
    assert_eq!(Ok(("", Numeric::Decimal(123.0))), lex_numeric("123.0"));
}
