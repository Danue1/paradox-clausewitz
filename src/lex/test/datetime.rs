use super::*;

#[test]
fn datetime() {
    assert_eq!(
        Ok((
            "".as_bytes(),
            Datetime {
                year: 2021,
                month: 04,
                date: 25
            }
        )),
        lex_datetime(b"2021.04.25")
    );
}
