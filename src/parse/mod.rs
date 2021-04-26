mod ast;
mod error;
#[cfg(test)]
mod test;

use crate::{Symbol, Token, Tokens};
pub use ast::*;
pub use error::*;

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(tokens: Tokens) -> Result<Ast, ParseError> {
    Context::new(tokens).parse()
}

macro_rules! unexpected_symbol {
    ($symbol:expr) => {
        Err(ParseError::UnexpectedSymbol($symbol))
    };
}

macro_rules! invalid_syntax {
    () => {
        Err(ParseError::InvalidSyntax)
    };
}

struct Context {
    flavor: Flavor,
    encoding: Encoding,
    token_list: Vec<Token>,
    position: usize,
}

enum Right {
    Expression(Expression),
    ExpressionList(Option<Scalar>, Vec<Expression>),
}

impl Context {
    #[inline]
    fn new(
        Tokens {
            flavor,
            encoding,
            token_list,
        }: Tokens,
    ) -> Self {
        Context {
            flavor,
            encoding,
            token_list,
            position: 0,
        }
    }

    #[inline]
    fn parse(mut self) -> Result<Ast, ParseError> {
        let value_list = self.parse_value_list()?;

        let ast = Ast {
            flavor: self.flavor,
            encoding: self.encoding,
            value_list,
        };

        Ok(ast)
    }

    fn parse_value_list(&mut self) -> ParseResult<Vec<Value>> {
        let mut value_list = vec![];

        loop {
            self.skip_ignorable()?;
            if self.has_next() {
                value_list.push(self.parse_value()?);
            } else {
                break;
            }
        }

        Ok(value_list)
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        let key = self.parse_left()?;

        match self.parse_right()? {
            Some(Right::Expression(expression)) => Ok(Value::KeyExpression(KeyExpression {
                key,
                expression: Box::new(expression),
            })),
            Some(Right::ExpressionList(tag, expression_list)) => {
                Ok(Value::KeyExpressionList(KeyExpressionList {
                    key,
                    tag,
                    expression_list,
                }))
            }
            None => Ok(Value::Key(key)),
        }
    }

    fn parse_left(&mut self) -> ParseResult<Key> {
        self.parse_key()
    }

    fn parse_key(&mut self) -> ParseResult<Key> {
        let is_variable = match self.peek() {
            Some(Token::Symbol(Symbol::At)) => {
                self.consume();
                true
            }
            _ => false,
        };

        let path = self.parse_path()?;

        Ok(Key { path, is_variable })
    }

    fn parse_path(&mut self) -> ParseResult<Path> {
        let scalar = self.parse_scalar()?;

        match self.peek() {
            Some(Token::Symbol(Symbol::Colon)) => {
                self.consume();
                let path = self.parse_path()?;
                Ok(Path::Parent(scalar, Box::new(path)))
            }
            Some(Token::Symbol(Symbol::Dot)) => {
                self.consume();
                let path = self.parse_path()?;
                Ok(Path::Field(scalar, Box::new(path)))
            }
            _ => Ok(Path::Scalar(scalar)),
        }
    }

    fn parse_scalar(&mut self) -> ParseResult<Scalar> {
        match self.peek() {
            Some(Token::Scalar(scalar)) => {
                self.consume();
                Ok(scalar.to_owned())
            }
            Some(Token::Symbol(symbol)) => unexpected_symbol!(symbol.to_owned()),
            _ => invalid_syntax!(),
        }
    }

    fn parse_right(&mut self) -> ParseResult<Option<Right>> {
        match self.peek() {
            Some(Token::Symbol(Symbol::Equal)) => {
                self.consume();
            }
            Some(Token::Symbol(symbol)) => return unexpected_symbol!(symbol.to_owned()),
            Some(Token::Scalar(_)) | None => return Ok(None),
        }

        let expression = self.parse_expression()?;

        match self.peek() {
            Some(Token::Symbol(Symbol::LeftBrace)) => {
                self.consume();

                let scalar = match expression {
                    Some(Expression::Value(Value::Key(Key {
                        path: Path::Scalar(scalar),
                        is_variable: false,
                    }))) => Some(scalar),
                    Some(Expression::Value(Value::Key(_))) | Some(Expression::Binary(..)) => {
                        return invalid_syntax!()
                    }
                    _ => None,
                };

                let mut expression_list = vec![];
                while let Some(expression) = match self.peek() {
                    Some(Token::Symbol(Symbol::RightBrace)) => {
                        self.consume();
                        None
                    }
                    Some(Token::Symbol(Symbol::At)) | Some(Token::Scalar(_)) | None => {
                        self.parse_expression()?
                    }
                    Some(Token::Symbol(symbol)) => return unexpected_symbol!(symbol.to_owned()),
                } {
                    expression_list.push(expression);
                }

                Ok(Some(Right::ExpressionList(scalar, expression_list)))
            }
            Some(Token::Symbol(symbol)) => unexpected_symbol!(symbol.to_owned()),
            Some(Token::Scalar(_)) | None => match expression {
                Some(expression) => Ok(Some(Right::Expression(expression))),
                None => invalid_syntax!(),
            },
        }
    }

    fn parse_expression(&mut self) -> ParseResult<Option<Expression>> {
        match self.peek() {
            Some(Token::Symbol(Symbol::LeftBrace))
            | Some(Token::Symbol(Symbol::RightBrace))
            | None => Ok(None),
            _ => {
                let key = self.parse_key()?;

                let operator = match self.peek() {
                    Some(Token::Symbol(Symbol::LeftChevron)) => Operator::LessThan,
                    Some(Token::Symbol(Symbol::LeftChevronEqual)) => Operator::LessThanOrEqual,
                    Some(Token::Symbol(Symbol::RightChevron)) => Operator::GreaterThan,
                    Some(Token::Symbol(Symbol::RightChevronEqual)) => Operator::GreaterThanOrEqual,
                    Some(Token::Symbol(Symbol::RightBrace))
                    | Some(Token::Symbol(Symbol::At))
                    | Some(Token::Scalar(_))
                    | None => return Ok(Some(Expression::Value(Value::Key(key)))),
                    Some(Token::Symbol(symbol)) => return unexpected_symbol!(symbol.to_owned()),
                };

                Ok(Some(Expression::Binary(key, operator, self.parse_key()?)))
            }
        }
    }

    #[inline]
    fn skip_ignorable(&mut self) -> ParseResult<()> {
        while let Some(current_token) = self.peek() {
            match current_token {
                Token::Symbol(symbol) => match symbol {
                    Symbol::RightBrace | Symbol::RightBracket | Symbol::RightChevron => {
                        self.consume()
                    }
                    Symbol::At => break,
                    _ => return unexpected_symbol!(symbol.to_owned()),
                },
                _ => break,
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn has_next(&self) -> bool {
        self.position < self.token_list.len()
    }

    #[inline(always)]
    fn consume(&mut self) {
        self.position += 1;
    }

    #[inline(always)]
    fn peek(&self) -> Option<Token> {
        if self.has_next() {
            Some(self.token_list[self.position].clone())
        } else {
            None
        }
    }
}
