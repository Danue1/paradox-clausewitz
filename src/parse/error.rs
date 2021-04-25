use crate::Symbol;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedSymbol(Symbol),
    InvalidSyntax,
}
