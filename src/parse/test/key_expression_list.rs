use crate::*;

#[test]
fn scalar_to_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![Expression::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: false
                })]
            })]
        }),
        parse(lex("ck3text\nfoo={bar}").unwrap())
    );
}

#[test]
fn scalar_to_scalar_and_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: false
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: false
                    })
                ]
            })]
        }),
        parse(lex("ck3text\nfoo={bar baz}").unwrap())
    );
}

#[test]
fn scalar_to_scalar_and_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: false
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: true
                    })
                ]
            })]
        }),
        parse(lex("ck3text\nfoo={bar @baz}").unwrap())
    );
}

#[test]
fn scalar_to_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![Expression::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: true
                })]
            })]
        }),
        parse(lex("ck3text\nfoo={@bar}").unwrap())
    );
}

#[test]
fn scalar_to_variable_and_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: true
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: false
                    })
                ]
            })]
        }),
        parse(lex("ck3text\nfoo={@bar baz}").unwrap())
    );
}

#[test]
fn scalar_to_variable_and_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: false
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: true
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: true
                    })
                ]
            })]
        }),
        parse(lex("ck3text\nfoo={@bar @baz}").unwrap())
    );
}

#[test]
fn variable_to_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![Expression::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: false
                })]
            })]
        }),
        parse(lex("ck3text\n@foo={bar}").unwrap())
    );
}

#[test]
fn variable_to_scalar_and_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: false
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: false
                    })
                ]
            })]
        }),
        parse(lex("ck3text\n@foo={bar baz}").unwrap())
    );
}

#[test]
fn variable_to_scalar_and_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: false
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: true
                    })
                ]
            })]
        }),
        parse(lex("ck3text\n@foo={bar @baz}").unwrap())
    );
}

#[test]
fn variable_to_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![Expression::Key(Key {
                    path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                    is_variable: true
                })]
            })]
        }),
        parse(lex("ck3text\n@foo={@bar}").unwrap())
    );
}

#[test]
fn variable_to_variable_and_scalar() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: true
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: false
                    })
                ]
            })]
        }),
        parse(lex("ck3text\n@foo={@bar baz}").unwrap())
    );
}

#[test]
fn variable_to_variable_and_variable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![Value::KeyExpressionList(KeyExpressionList {
                key: Key {
                    path: Path::Scalar(Scalar::Ident("foo".to_owned())),
                    is_variable: true
                },
                tag: None,
                expression_list: vec![
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("bar".to_owned())),
                        is_variable: true
                    }),
                    Expression::Key(Key {
                        path: Path::Scalar(Scalar::Ident("baz".to_owned())),
                        is_variable: true
                    })
                ]
            })]
        }),
        parse(lex("ck3text\n@foo={@bar @baz}").unwrap())
    );
}
