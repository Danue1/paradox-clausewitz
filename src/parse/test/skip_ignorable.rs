use crate::*;

#[test]
fn skip_ignorable() {
    assert_eq!(
        Ok(Ast {
            flavor: Flavor::Ck3,
            encoding: Encoding::Text,
            value_list: vec![]
        }),
        parse(lex("ck3text\n}").unwrap())
    );
}
