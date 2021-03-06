use nom::error::{ErrorKind, ParseError};
use nom::Needed;

#[derive(Debug, PartialEq)]
pub enum LexError {
    Nom(ErrorKind),
    Needed(Needed),
    UnknownFlaver(String),
    UnknownEncoding(String),
}

impl<'lex> ParseError<&'lex [u8]> for LexError {
    fn from_error_kind(_: &'lex [u8], kind: ErrorKind) -> Self {
        LexError::Nom(kind)
    }

    fn append(_: &'lex [u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'lex> From<nom::Err<LexError>> for LexError {
    fn from(e: nom::Err<LexError>) -> LexError {
        match e {
            nom::Err::Incomplete(needed) => LexError::Needed(needed),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}
