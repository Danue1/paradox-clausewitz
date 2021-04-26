use crate::*;

#[test]
fn scalar_list_and_scalar_list() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![
                Value::KeyExpression(KeyExpression {
                    key: Key {
                        path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                        is_variable: false
                    },
                    expression: Box::new(Expression::Value(Value::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: false
                    })))
                }),
                Value::KeyExpression(KeyExpression {
                    key: Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: false
                    },
                    expression: Box::new(Expression::Value(Value::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bax".to_owned())),
                        is_variable: false
                    })))
                }),
            ]
        }),
        parse(lex(b"ck3text\nfoo=bar baz=bax").unwrap())
    );
}
