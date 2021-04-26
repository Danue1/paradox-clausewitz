mod error;
#[cfg(test)]
mod test;
mod token;

use crate::{
    all_consuming, alt, char, fold_many0, is_line, is_not_boundary, is_not_double_quote,
    is_not_line, is_numeric, is_whitesapce, map, not, opt, tag, tag_no_case, take_while,
    take_while1, tuple, value, Boolean, Datetime, Encoding, Flavor, Scalar,
};
pub use error::*;
pub use token::*;

pub type LexResult<'lex, T> = nom::IResult<&'lex [u8], T, LexError>;

impl std::convert::TryFrom<&[u8]> for Tokens {
    type Error = LexError;

    fn try_from(source: &[u8]) -> Result<Self, Self::Error> {
        lex(source)
    }
}

impl std::convert::TryFrom<&str> for Tokens {
    type Error = LexError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        lex(source.as_bytes())
    }
}

impl std::str::FromStr for Tokens {
    type Err = LexError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        source.try_into()
    }
}

pub fn lex(source: &[u8]) -> Result<Tokens, LexError> {
    match map(
        tuple((lex_flavor, lex_encoding, lex_to_line, lex_token_list)),
        |(flavor, encoding, _, token_list)| Tokens {
            flavor,
            encoding,
            token_list,
        },
    )(source)
    {
        Ok((_, token_list)) => Ok(token_list),
        Err(error) => Err(error.into()),
    }
}

fn lex_flavor(source: &[u8]) -> LexResult<Flavor> {
    alt((
        map(tag_no_case("ck3"), |_| Flavor::Ck3),
        map(tag_no_case("eu4"), |_| Flavor::Eu4),
    ))(source)
}

fn lex_encoding(source: &[u8]) -> LexResult<Encoding> {
    alt((
        map(tag_no_case("txt"), |_| Encoding::Text),
        map(tag_no_case("text"), |_| Encoding::Text),
        map(tag_no_case("binary"), |_| Encoding::Binary),
    ))(source)
}

fn lex_token_list(source: &[u8]) -> LexResult<Vec<Token>> {
    fold_many0(
        alt((map(lex_ignorable, |_| None), map(lex_token, Some))),
        vec![],
        |mut token_list, token| {
            if let Some(token) = token {
                token_list.push(token);
            }
            token_list
        },
    )(source)
}

fn lex_ignorable(source: &[u8]) -> LexResult<()> {
    alt((lex_whitespace1, lex_line1, lex_comment))(source)
}

fn lex_token(token: &[u8]) -> LexResult<Token> {
    alt((
        map(lex_symbol, Token::Symbol),
        map(lex_scalar, Token::Scalar),
        map(lex_ident, |ident| {
            Token::Scalar(Scalar::Ident(ident.to_owned()))
        }),
    ))(token)
}

fn lex_symbol(source: &[u8]) -> LexResult<Symbol> {
    alt((
        map(char('{'), |_| Symbol::LeftBrace),
        map(char('}'), |_| Symbol::RightBrace),
        map(char('['), |_| Symbol::LeftBracket),
        map(char(']'), |_| Symbol::RightBracket),
        map(tag("<="), |_| Symbol::LeftChevronEqual),
        map(char('<'), |_| Symbol::LeftChevron),
        map(tag(">="), |_| Symbol::RightChevronEqual),
        map(char('>'), |_| Symbol::RightChevron),
        map(char('='), |_| Symbol::Equal),
        map(char('!'), |_| Symbol::Exclamation),
        map(char(':'), |_| Symbol::Colon),
        map(char('.'), |_| Symbol::Dot),
        map(char('@'), |_| Symbol::At),
    ))(source)
}

#[derive(Debug, PartialEq)]
enum Numeric {
    Integer(i64),
    Decimal(f64),
}

fn lex_scalar(source: &[u8]) -> LexResult<Scalar> {
    alt((
        map(lex_string, Scalar::String),
        map(lex_numeric, |dotted| match dotted {
            Numeric::Integer(dotted) => Scalar::Integer(dotted),
            Numeric::Decimal(dotted) => Scalar::Decimal(dotted),
        }),
        map(lex_datetime, Scalar::Datetime),
        map(lex_boolean, Scalar::Boolean),
    ))(source)
}

fn lex_ident(source: &[u8]) -> LexResult<&str> {
    map(take_while1(is_not_boundary), |ident| {
        std::str::from_utf8(ident).unwrap()
    })(source)
}

fn lex_comment(source: &[u8]) -> LexResult<()> {
    map(tuple((char('#'), lex_to_line)), |_| ())(source)
}

fn lex_string(source: &[u8]) -> LexResult<String> {
    fn lex_double_quote(source: &[u8]) -> LexResult<char> {
        char('"')(source)
    }

    map(
        tuple((
            lex_double_quote,
            take_while(is_not_double_quote),
            lex_double_quote,
        )),
        |(_, string, _): (_, &[u8], _)| std::str::from_utf8(string).unwrap().to_owned(),
    )(source)
}

fn lex_numeric(source: &[u8]) -> LexResult<Numeric> {
    fn lex_sign(source: &[u8]) -> LexResult<char> {
        alt((char('-'), char('+')))(source)
    }

    let (source, sign) = opt(lex_sign)(source)?;
    let (source, year) = lex_number(source)?;
    let (source, dot) = opt(lex_dot)(source)?;
    if dot.is_none() {
        let _ = not(lex_ident)(source)?;

        let integer = format!("{}{}", sign.unwrap_or('+'), year).parse().unwrap();

        return Ok((source, Numeric::Integer(integer)));
    }

    let (source, month) = lex_number(source)?;

    let decimal = format!("{}{}.{}", sign.unwrap_or('+'), year, month)
        .parse()
        .unwrap();

    Ok((source, Numeric::Decimal(decimal)))
}

fn lex_datetime(source: &[u8]) -> LexResult<Datetime> {
    map(
        tuple((lex_number, lex_dot, lex_number, lex_dot, lex_number)),
        |(year, _, month, _, date)| Datetime {
            year: year.parse().unwrap(),
            month: month.parse().unwrap(),
            date: date.parse().unwrap(),
        },
    )(source)
}

fn lex_boolean(source: &[u8]) -> LexResult<Boolean> {
    let (source, token) = lex_ident(source)?;
    let (_, boolean) = all_consuming(alt((
        value(Boolean::Yes, tag("yes")),
        value(Boolean::No, tag("no")),
        value(Boolean::True, tag("true")),
        value(Boolean::False, tag("false")),
    )))(token.as_bytes())?;

    Ok((source, boolean))
}

#[inline(always)]
fn lex_number(source: &[u8]) -> LexResult<&str> {
    map(take_while1(is_numeric), |number| {
        std::str::from_utf8(number).unwrap()
    })(source)
}

#[inline(always)]
fn lex_dot(source: &[u8]) -> LexResult<()> {
    map(char('.'), |_| ())(source)
}

#[inline(always)]
fn lex_whitespace1(source: &[u8]) -> LexResult<()> {
    map(take_while1(is_whitesapce), |_| ())(source)
}

#[inline(always)]
fn lex_line1(source: &[u8]) -> LexResult<()> {
    map(take_while1(is_line), |_| ())(source)
}

#[inline(always)]
fn lex_to_line(source: &[u8]) -> LexResult<()> {
    map(take_while(is_not_line), |_| ())(source)
}
