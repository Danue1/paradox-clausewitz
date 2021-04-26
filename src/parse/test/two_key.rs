use crate::*;

#[test]
fn scalar_and_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![
                Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                }),
                Value::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: false
                })
            ]
        }),
        parse(lex(b"ck3text\nfoo bar").unwrap())
    );
}
