use crate::{Val, tokenizer::*};

#[test]
fn test_tokenizer() {
    use Tag::*;
    use Token::*;
    check("hello", vec![w("hello")]);
    check("  hello", vec![w("  hello")]);
    check("  hello ", vec![w("  hello ")]);
    check("hello world", vec![w("hello "), w("world")]);
    check("hello world!!!", vec![w("hello "), w("world!!!")]);
    check("    ", vec![w("    ")]);

    check("{br}", vec![OpenTag(Br)]);
    check("{ br}", vec![OpenTag(Br)]);
    check("{ br }", vec![OpenTag(Br)]);

    check("\n", vec![OpenTag(Br)]);
    check("{pg}", vec![OpenTag(Pg)]);

    check("{/pg}", vec![CloseTag(Pg)]);
    check("{/pg }", vec![CloseTag(Pg)]);
    check("{/ pg }", vec![CloseTag(Pg)]);
    check("{ / pg }", vec![CloseTag(Pg)]);

    check("{br}{br}", vec![OpenTag(Br), OpenTag(Br)]);
    check(
        "oh{br}hi{br}mark",
        vec![w("oh"), OpenTag(Br), w("hi"), OpenTag(Br), w("mark")],
    );

    check(
        "{blegh}",
        vec![OpenTag(Unknown("blegh".to_string(), "".to_string()))],
    );
    check(
        "{ blegh }",
        vec![OpenTag(Unknown("blegh".to_string(), "".to_string()))],
    );
    check(
        "{ blegh args }",
        vec![OpenTag(Unknown("blegh".to_string(), "args".to_string()))],
    );
    check(
        "{ blegh    args }",
        vec![OpenTag(Unknown("blegh".to_string(), "args".to_string()))],
    );
    check("{clr1}", vec![OpenTag(Eff(TextEffect::Color(1)))]);
    check("{clr 1}", vec![OpenTag(Eff(TextEffect::Color(2)))]);
    check("{say hi}", vec![OpenTag(SayVar("hi".to_string()))]);
    check("{ say  hi }", vec![OpenTag(SayVar("hi".to_string()))]);
    check(
        r#"{say {item "cat"}}"#,
        vec![OpenTag(SayItem("cat".to_string()))],
    );
    check(
        r#"{exit "hi,3,4"}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );
    check(
        r#"{exit "hi",3,4}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );
    check(
        r#"{exit "hi", 3, 4}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );

    let val = Val::S("hello world!".to_string());
    let expr = Expr::SimpleExpr(SimpleExpr::Val(val));
    check(
        "{ a = hello world! }",
        vec![OpenTag(Set("a".to_string(), expr))],
    );
}

fn w(w: &str) -> Token {
    Token::Word(w.to_string())
}

fn check(given: &str, expected: Vec<Token>) {
    let tokenizer = Tokenizer::new(given);
    let actual: Vec<_> = tokenizer.collect();
    assert_eq!(actual, expected);
}
