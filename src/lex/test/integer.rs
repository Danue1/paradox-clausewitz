use super::*;

#[test]
fn integer() {
    assert_eq!(Ok(("", Numeric::Integer(1))), lex_numeric("1"));
    assert_eq!(Ok(("", Numeric::Integer(12))), lex_numeric("12"));
    assert_eq!(Ok(("", Numeric::Integer(123))), lex_numeric("123"));
}
