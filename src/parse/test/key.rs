use crate::*;

#[test]
fn scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                is_variable: false
            })]
        }),
        parse(lex("ck3text\nfoo").unwrap())
    );
}

#[test]
fn parent_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Parent(
                    Scalar::Ident("foo".to_owned()),
                    Box::new(Path::Scalar(Scalar::Ident("bar".to_owned())))
                ),
                is_variable: false
            })]
        }),
        parse(lex("ck3text\nfoo:bar").unwrap())
    );
}

#[test]
fn field_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Field(
                    Scalar::Ident("foo".to_owned()),
                    Box::new(Path::Scalar(Scalar::Ident("bar".to_owned())))
                ),
                is_variable: false
            })]
        }),
        parse(lex("ck3text\nfoo.bar").unwrap())
    );
}

#[test]
fn variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                is_variable: true
            })]
        }),
        parse(lex("ck3text\n@foo").unwrap())
    );
}

#[test]
fn parent_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Parent(
                    Scalar::Ident("foo".to_owned()),
                    Box::new(Path::Scalar(Scalar::Ident("bar".to_owned())))
                ),
                is_variable: true
            })]
        }),
        parse(lex("ck3text\n@foo:bar").unwrap())
    );
}

#[test]
fn field_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::Key(Key {
                path: Path::Field(
                    Scalar::Ident("foo".to_owned()),
                    Box::new(Path::Scalar(Scalar::Ident("bar".to_owned())))
                ),
                is_variable: true
            })]
        }),
        parse(lex("ck3text\n@foo.bar").unwrap())
    );
}
