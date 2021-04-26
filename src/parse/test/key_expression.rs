use crate::*;

#[test]
fn scalar_to_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpression(KeyExpression {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                expression: Box::new(Expression::Value(Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: false
                })))
            })]
        }),
        parse(lex(b"ck3text\nfoo=bar").unwrap())
    );
}

#[test]
fn scalar_to_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpression(KeyExpression {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                expression: Box::new(Expression::Value(Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: true
                })))
            })]
        }),
        parse(lex(b"ck3text\nfoo=@bar").unwrap())
    );
}

#[test]
fn variable_to_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpression(KeyExpression {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                expression: Box::new(Expression::Value(Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: false
                })))
            })]
        }),
        parse(lex(b"ck3text\n@foo=bar").unwrap())
    );
}

#[test]
fn variable_to_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpression(KeyExpression {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                expression: Box::new(Expression::Value(Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: true
                })))
            })]
        }),
        parse(lex(b"ck3text\n@foo=@bar").unwrap())
    );
}
